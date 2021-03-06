import datetime as dt
from loguru import logger

from lxml import etree
from model import RespBody
from pytz import timezone
from utils.cache import with_cache
from utils.network import request_abroad_website


@with_cache(site='nicoseiga', limit=0.2)
async def nicoseigadata(imid: str, udid: str) -> RespBody:
    '''根据im号获取视频相关数据'''
    imurl = f'https://seiga.nicovideo.jp/seiga/im{imid}'
    r = await request_abroad_website(imurl)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        title = title = page.xpath('//div[@class="lg_ttl_illust"]/h1')[0].text
        uid = page.xpath(
            '//*[@id="header"]/div[2]/ul[1]/li[2]/a')[0].attrib['href'][13:]
        cover = page.xpath(
            '//a[@id="link_thumbnail_main"]/img')[0].attrib['src']
        desc = page.xpath('//table[@id="illust_area"]/tr[2]/td/div[3]')[0].text
        post_time = page.xpath(
            '//table[@id="illust_area"]/tr[2]/td/div[4]')[0].text[:-3]
        author = f'nicoseiga-author:{uid}'
        author_name = page.xpath(
            '//table[@id="illust_area"]/tr[2]/td/div[2]/strong')[0].text
    except Exception as e:
        logger.exception(e)
        return RespBody(status='parsererr', msg=f'seigaparsererr: {repr(e)}')

    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        media=[cover],
        desc=desc,
        ptime=get_ptime(post_time),
        author=[author],
        author_name=[author_name],
        tname='DRAWING',
    )
    return RespBody(data=data)


def get_ptime(post_time: str) -> str:
    # 2022年02月09日 00:03:57
    SEIGA_FORMAT = '%Y年%m月%d日 %H:%M:%S'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(post_time, SEIGA_FORMAT)
    dt_struct = timezone('Asia/Tokyo').localize(dt_struct)
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-02-08 23:03:57 +0800
    return dt_struct.strftime(PTIME_FORMAT)
