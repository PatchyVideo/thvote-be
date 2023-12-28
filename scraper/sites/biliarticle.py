from model import RespBody
from utils.cache import with_cache, get_cache
from utils.network import request_api, request_website
from utils.biliutils import get_header, get_cookies
from loguru import logger
from lxml import etree
import ujson
import time


@with_cache(site='biliarticle', limit=0.2)
async def biliarticledata(cvid: str, udid: str) -> RespBody:
    '''根据cvid(cv号)获取文章相关数据'''
    api = f'https://api.bilibili.com/x/article/viewinfo?id={cvid}'
    r = await request_api(api, headers=get_header(), cookies=get_cookies())
    json = r.get('data')
    if json is None:
        if r['code'] == -352:
            return RespBody(status='apierr', msg=f'biliapi: banned')
        return RespBody(status='apierr', msg=f'biliapimsg: {r["message"]}')
    
    uid = json['mid']
    author = [f'bilibili-author:{uid}']
    author_name = [json['author_name']]
    data = RespBody.Data(
        title=json['title'].strip(),
        udid=udid,
        cover=json['banner_url'],
        ptime='0000-00-00 00:00:00 +0800',
        author=author,
        author_name=author_name,
    )
    return RespBody(msg=f'biliapimsg: {r["message"]}', data=data)
    
    # r = await request_website(api, headers=header, cookies={'SESSDATA': biliconfig['SESSDATA']})
    # html = r.content.decode('utf-8')
    # try:
    #     page = etree.HTML(html)
    #     test = page.xpath('/html/body/script')[0].text
    #     test = test.replace('window.__INITIAL_STATE__ = ','')
    #     test = test[:test.find(';(function()')]
    #     json = ujson.loads(test)
    
    # except Exception as e:
    #     logger.exception(e)
    #     return RespBody(status='parsererr', msg=f'tiebaparsererr: {repr(e)}')

    # data = RespBody.Data(
    #     title=json['title'].strip(),
    #     udid=udid,
    #     cover=json['banner_url'],
    #     desc=json['summary'],
    #     ptime=get_ptime(json['publish_time']),
    #     author=[f'biliarticle-author:{json["author"]["mid"]}'],
    #     author_name=[json["author"]["name"]],
    #     tname='OTHER',
    # )
    # return RespBody(data=data)


def get_ptime(ctime: int) -> str:
    '''根据时间戳(int类型)获取投稿时间'''
    # 2022-02-07 13:34:53 +0800
    return time.strftime(
        "%Y-%m-%d %H:%M:%S %z", time.localtime(ctime))
