import httpx
import ujson

from .cache import get_cache


async def get_redirect_url(url: str) -> str:
    '''获取重定向后的链接'''
    async with httpx.AsyncClient() as client:
        resp = await client.head(url)
    return resp.headers['Location']


async def request_website(url: str, **kwargs) -> httpx.Response:
    '''向网站发送请求，不走代理'''
    async with httpx.AsyncClient() as client:
        if not kwargs.get('data') and not kwargs.get('json'):
            resp = await client.get(url=url, **kwargs, timeout=30)
        else:
            resp = await client.post(url=url, **kwargs, timeout=30)
        return resp


async def request_abroad_website(url: str, **kwargs) -> httpx.Response:
    '''向国外网站发送请求，有代理则走代理'''
    proxies = get_cache('proxies')
    if proxies:
        try:
            async with httpx.AsyncClient(proxies=proxies) as client:
                if not kwargs.get('data') and not kwargs.get('json'):
                    if kwargs.get('my_metod') == 'post':
                        del kwargs['my_metod']
                        resp = resp = await client.post(url=url, **kwargs, timeout=30)
                    else:
                        resp = await client.get(url=url, **kwargs, timeout=30)
                else:
                    resp = await client.post(url=url, **kwargs, timeout=30)
                return resp
        except httpx.ProxyError:
            pass
    return await request_website(url=url, **kwargs)


async def request_api(url: str, **kwargs) -> dict:
    '''向API发送请求，不走代理'''
    async with httpx.AsyncClient() as client:
        if not kwargs.get('data') and not kwargs.get('json'):
            resp = await client.get(url=url, **kwargs, timeout=30)
        else:
            resp = await client.post(url=url, **kwargs, timeout=30)
        return ujson.decode(resp.content)


async def request_abroad_api(url: str, **kwargs) -> dict:
    '''向国外API发送请求，有代理则走代理'''
    proxies = get_cache('proxies')
    if proxies:
        try:
            async with httpx.AsyncClient(proxies=proxies) as client:
                if not kwargs.get('data') and not kwargs.get('json'):
                    if kwargs.get('my_metod') == 'post':
                        del kwargs['my_metod']
                        resp = resp = await client.post(url=url, **kwargs, timeout=30)
                    else:
                        resp = await client.get(url=url, **kwargs, timeout=30)
                else:
                    resp = await client.post(url=url, **kwargs, timeout=30)
                return ujson.decode(resp.content)
        except httpx.ProxyError:
            pass
    return await request_api(url=url, **kwargs)
