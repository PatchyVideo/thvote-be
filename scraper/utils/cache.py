import pickle
import time
from datetime import timedelta
from functools import wraps
from os import path
from typing import Any, List, Optional

import redis
import toml
from loguru import logger
from model import RespBody

cwd = path.dirname(__file__)
prjdir = path.abspath(path.join(cwd, '..'))

config_file = path.join(prjdir, 'config.toml')
if not path.isfile(config_file):
    logger.error('NO CONFIG FILE')
config = toml.load(config_file)
redis_config = config.get('redis_config')
redis_client = redis.Redis(
    redis_config['host'],
    redis_config['port'],
    redis_config['db'],
    charset="utf-8",
)

CACHE_FORMAT = 'scraper_cache_{key}'


def get_cache(key: str) -> Any:
    name = CACHE_FORMAT.format(key=key)
    logger.debug(f'REDIS GET {name}')
    cache = redis_client.get(name)
    return pickle.loads(cache) if cache else cache


def set_cache(key: str, cache: Any, ex: Optional[timedelta] = None) -> None:
    name = CACHE_FORMAT.format(key=key)
    logger.debug(f'REDIS SET {name}')
    redis_client.set(name, pickle.dumps(cache), ex)


def del_cache(key: str) -> int:
    name = CACHE_FORMAT.format(key=key)
    logger.debug(f'REDIS DELETE {name}')
    return redis_client.delete(name)


def scan_cache(head: str) -> List[str]:
    logger.debug(f'REDIS SCAN {head}*')
    return [cache.decode() for cache in redis_client.scan_iter(f'{head}*')]


def clean_cache():
    for cache in scan_cache('scraper_cache'):
        logger.debug(f'REDIS DELETE {cache}')
        redis_client.delete(cache)
    logger.success('CACHE CLEAR SUCCESSFUL')


def init_cache():
    cache_config = [
        'proxies',
        'twiapi_auth',
        'pixiv_token',
        'pixiv_bad_tags',
        'ytbapi_key',
        'clear_key',
        'melon_proxy',
    ]
    for conf in cache_config:
        set_cache(conf, config.get(conf))
    logger.success('CACHE INIT SUCCESSFUL')


# clean_cache()
init_cache()


def with_cache(site: str, limit: float = None):
    def wrapper(func):
        @wraps(func)
        async def inner(wid: str, udid: str = None):
            if site == 'thbwiki' and wid == get_cache('clear_key'):
                clean_cache()
                init_cache()
                return RespBody(status='success')
            udid = f'{site}:{wid}'
            cached = get_cache(udid)
            if cached:
                return cached
            if limit:
                last = get_cache(f'{site}_limit')
                if last:
                    wait = 0
                    while wait < limit:
                        time.sleep(0.01)
                        wait = time.time() - last

            resp: RespBody = await func(wid, udid)
            set_cache(f'{site}_limit', time.time())
            if resp.status == 'ok':
                udid = resp.data.udid
                if resp.data.cover:
                    resp.data.cover = resp.data.cover.replace('http:', 'https:')
                set_cache(udid, resp)
            else:
                set_cache(udid, resp, timedelta(minutes=1))
            return resp
        return inner
    return wrapper
