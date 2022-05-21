import ujson
from lxml import etree
from model import RespBody
from utils.cache import with_cache
from utils.network import request_website

from .bilibili import get_ptime

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36'}


@with_cache(site='acfun', limit=0.2)
async def acdata(acid: str, udid: str) -> RespBody:
    '''根据acid(ac号)获取视频相关数据'''
    acurl = f'https://www.acfun.cn/v/ac{acid}'
    r = await request_website(acurl, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        script = page.xpath('/html/body/script[7]')[0].text
        json = script[script.find('{'):script.find(';')]
        data = ujson.loads(json)
        ctime = data['currentVideoInfo']['uploadTime']
        uid = data['user']['id']
        author = f'acfun-author:{uid}'
        if data.get('originalDeclare') == 1:
            repost = False
        else:
            repost = True
    except Exception as e:
        return RespBody(status='parsererr', msg=f'acparsererr: {repr(e)}')

    data = RespBody.Data(
        title=data['title'],
        udid=udid,
        cover=data['coverImgInfo']['thumbnailImageCdnUrl'],
        desc=data['description'],
        ptime=get_ptime(ctime//1000),
        author=[author],
        author_name=[data['user']['name']],
        repost=repost
    )
    return RespBody(data=data)
