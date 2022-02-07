import httpx
import ujson

# from .config import get_config


async def request_website(url: str, **kwargs) -> httpx.Response:
    '''向网站发送请求，不走代理'''
    async with httpx.AsyncClient() as client:
        if not kwargs.get('data') and not kwargs.get('json'):
            resp = await client.get(url=url, **kwargs, timeout=30)
        else:
            resp = await client.post(url=url, **kwargs, timeout=30)
        return resp


# async def request_abroad_website(url: str, **kwargs) -> httpx.Response:
#     '''向国外网站发送请求，有代理则走代理'''
#     if get_config('proxies'):
#         try:
#             async with httpx.AsyncClient(proxies=get_config('proxies')) as client:
#                 if not kwargs.get('data') and not kwargs.get('json'):
#                     resp = await client.get(url=url, **kwargs, timeout=30)
#                 else:
#                     resp = await client.post(url=url, **kwargs, timeout=30)
#                 return resp
#         except httpx.ProxyError:
#             pass
#     return await request_website(url=url, **kwargs)


async def request_api(url: str, **kwargs) -> dict:
    '''向API发送请求，不走代理'''
    async with httpx.AsyncClient() as client:
        if not kwargs.get('data') and not kwargs.get('json'):
            resp = await client.get(url=url, **kwargs, timeout=30)
        else:
            resp = await client.post(url=url, **kwargs, timeout=30)
        return ujson.decode(resp.content)


# async def request_abroad_api(url: str, **kwargs) -> dict:
#     '''向国外API发送请求，有代理则走代理'''
#     if get_config('proxies'):
#         try:
#             async with httpx.AsyncClient(proxies=get_config('proxies')) as client:
#                 if not kwargs.get('data') and not kwargs.get('json'):
#                     resp = await client.get(url=url, **kwargs, timeout=30)
#                 else:
#                     resp = await client.post(url=url, **kwargs, timeout=30)
#                 return ujson.decode(resp.content)
#         except httpx.ProxyError:
#             pass
#     return await request_api(url=url, **kwargs)
