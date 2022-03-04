import pickle
import time
from datetime import timedelta
from functools import wraps
from os import path
from typing import Any, Optional

import redis
import toml
from loguru import logger

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
    cache = redis_client.get(CACHE_FORMAT.format(key=key))
    return pickle.loads(cache) if cache else cache


def set_cache(key: str, cache: Any, ex: Optional[timedelta] = None) -> None:
    redis_client.set(CACHE_FORMAT.format(key=key), pickle.dumps(cache), ex)


def del_cache(key: str) -> None:
    redis_client.delete(CACHE_FORMAT.format(key=key))


cache_config = [
    'proxies',
    'twiapi_auth',
    'pixiv_token',
    'pixiv_bad_tags',
    'ytbapi_key',
]
for conf in cache_config:
    set_cache(conf, config.get(conf))


def with_cache(site: str, limit: float = None):
    def wrapper(func):
        @wraps(func)
        async def inner(wid: str, udid: str = None):
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

            ret = await func(wid, udid)
            if ret[2].cover:
                ret[2].cover=ret[2].cover.replace('http:','https:')
            set_cache(f'{site}_limit', time.time())
            if ret == 'ok':
                set_cache(udid, ret)
            else:
                set_cache(udid, ret, timedelta(minutes=5))
            return ret
        return inner
    return wrapper
