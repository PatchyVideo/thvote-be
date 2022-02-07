import pickle
from datetime import timedelta
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


set_cache('proxies', config.get('proxies'))
set_cache('twiapi_auth', config.get('twiapi_auth'))
