import ujson
from lxml import etree
from model import RespBody
from utils.cache import with_cache
from utils.network import request_website

from .bilibili import get_ptime

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36'}


@with_cache(site='acarticle', limit=0.2)
async def acadata(acid: str, udid: str) -> RespBody:
    '''根据acid(ac号)获取文章相关数据'''
    acurl = f'https://www.acfun.cn/a/ac{acid}'
    r = await request_website(acurl, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        script = page.xpath('//*[@id="main"]/script')[0].text
        m = [0, 0, 0]
        while m[2] != -1:
            m[0] = m[1]
            m[1] = m[2]
            m[2] = script.find(';', m[1]+1)
        json = script[script.find('{'):m[0]]
        data = ujson.loads(json)
        cover=data['coverUrl']
        if i:=cover.find('?'):
            cover=cover[:i]
        ctime = data['createTimeMillis']
        uid = data['user']['id']
        author = f'acfun-author:{uid}'
        
        area = data['channel']['name']
        sub_area = data['realm']['realmName']
        tname = 'ARTICLE'
        if area == '二次元画师':
            tname = 'DRAWING'
        elif area == '漫画文学':
            if sub_area == '漫画':
                tname = 'DRAWING'
    except Exception as e:
        return RespBody(status='parsererr', msg=f'acaparsererr: {repr(e)}')

    data = RespBody.Data(
        title=data['title'],
        udid=udid,
        cover=cover,
        desc=data['description'],
        ptime=get_ptime(ctime//1000),
        author=[author],
        author_name=[data['user']['name']],
        tname=tname,
    )
    return RespBody(data=data)
