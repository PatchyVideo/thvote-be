import time

import httpx

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
