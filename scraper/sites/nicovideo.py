import datetime as dt
import re
from typing import Tuple

import ujson
from lxml import etree
from model import Data
from pytz import timezone
from utils.cache import with_cache
from utils.network import request_abroad_website


@with_cache(site='nicovideo', limit=0.2)
async def nicovideodata(smid: str, udid: str) -> Tuple[str, str, Data]:
    '''根据sm号获取视频相关数据'''
    smurl = f'https://www.nicovideo.jp/watch/sm{smid}'
    r = await request_abroad_website(smurl)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        videojson = page.xpath('//script[@class="LdJson"]')[0].text
        data = ujson.loads(videojson)
        uploadDate = data['uploadDate']
        user_url = data['author']['url']
        uid = re.match(r'.*user/(\d+)', user_url).group(1)
        author = f'nicovideo-author:{uid}'
    except Exception as e:
        return 'nicovideoparsererr', repr(e), Data()

    return 'ok', 'ok', Data(
        title=data['name'],
        udid=udid,
        cover=data['thumbnailUrl'][0],
        desc=data['description'],
        ptime=get_ptime(uploadDate),
        author=author,
        author_name=data['author']['name'],
    )


def get_ptime(uploadDate: str) -> str:
    # 2022-01-30T19:00:00+09:00
    NICO_FORMAT = '%Y-%m-%dT%H:%M:%S%z'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(uploadDate, NICO_FORMAT)
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-01-30 18:00:00 +0800
    return dt_struct.strftime(PTIME_FORMAT)
