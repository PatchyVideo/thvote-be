import time
from functools import wraps

import httpx

from .cache import get_cache, set_cache
from .match import *


def get_post_time(ctime: int) -> str:
    '''根据时间戳(int类型)获取投稿时间'''
    # 2022-02-07 13:34:53 +0800
    return time.strftime(
        "%Y-%m-%d %H:%M:%S %z", time.localtime(ctime))


async def get_redirect_url(url: str) -> str:
    '''获取重定向后的链接'''
    async with httpx.AsyncClient() as client:
        resp = await client.head(url)
    return resp.headers['Location']


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
            set_cache(f'{site}_limit', time.time())
            set_cache(udid, ret)
            return ret
        return inner
    return wrapper
