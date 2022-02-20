from typing import Tuple

from lxml import etree
from model import Data
from utils.cache import with_cache
from utils.network import request_website

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36'}


@with_cache(site='acfun', limit=0.2)
async def acdata(acid: str, udid: str) -> Tuple[str, str, Data]:
    '''根据acid(ac号)获取视频相关数据'''
    acurl = f'https://www.acfun.cn/v/ac{acid}'
    r = await request_website(acurl, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        title = page.xpath('//h1[@class="title"]/span')[0].text
        up_name = page.xpath('//a[@class="up-name"]')[0]
        uid = up_name.attrib['href'][3:]
        desc = page.xpath('//div[@class="description-container"]')[0].text
        publish_time = page.xpath('//div[@class="publish-time"]')[0].text[4:]
        author = f'acfun-author:{uid}'
    except Exception as e:
        return 'parsererr', f'acparsererr: {repr(e)}', Data()

    return 'ok', 'ok', Data(
        title=title,
        udid=udid,
        desc=desc,
        ptime=f'{publish_time} 00:00:00 +0800',
        author=[author],
        author_name=[up_name.text],
    )
