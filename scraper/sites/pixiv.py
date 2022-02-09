import datetime as dt
import re
from typing import Tuple

import ujson
from model import Data
from pixivpy_async import AppPixivAPI, PixivClient
from pytz import timezone
from utils import with_cache
from utils.cache import get_cache


@with_cache(site='pixiv', limit=0.2)
async def pixdata(pid: str, udid: str) -> Tuple[str, str, Data]:
    async with PixivClient(proxy=get_cache('proxies')['all://']) as client:
        aapi = AppPixivAPI(client=client)
        await aapi.login(refresh_token=get_cache('pixiv_token'))
        info = await aapi.illust_detail(pid)
        if info.get('error'):
            return 'pixapierr', ujson.encode(info['error']), Data()
        data = info['illust']
        uid = data['user']['id']
        author = f'pixiv-author:{uid}'
        if data['meta_single_page']:
            media = [data['meta_single_page']['original_image_url']]
        elif data['meta_pages']:
            media = [x['image_urls']['original'] for x in data['meta_pages']]

        touhou = None
        for tag in data['tags']:
            if '東方' in tag['name']:
                touhou = 'ok'
                break

        return 'ok', touhou or 'may not touhou', Data(
            title=data['title'],
            udid=udid,
            media=media,
            desc=data['caption'],
            ptime=get_ptime(data['create_date']),
            author=author,
        )


async def get_pid(text: str) -> str:
    pid = None
    if match_mobile := re.match(r'.*(?:pixiv|pixivdl).net/(?:(?:(?:artworks|i)/)|member_illust.php?.*id=)([0-9]+)', text):
        pid = match_mobile.group(1)
    return pid


def get_ptime(create_date: str) -> str:
    # 2022-02-08T21:02:00+09:00
    PIXIV_FORMAT = '%Y-%m-%dT%H:%M:%S+09:00'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(create_date, PIXIV_FORMAT)
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-02-08 20:02:00 +0800
    return dt_struct.strftime(PTIME_FORMAT)
