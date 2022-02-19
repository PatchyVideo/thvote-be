import datetime as dt
from typing import Tuple

from model import Data
from pytz import timezone
from utils.cache import get_cache, with_cache
from utils.network import request_abroad_api


@with_cache(site='youtube')
async def ytbdata(vid: str, udid: str) -> Tuple[str, str, Data]:
    api = 'https://www.googleapis.com/youtube/v3/videos'
    r = await request_abroad_api(
        api,
        params={
            'key': get_cache('ytbapi_key'),
            'id': vid,
            'part': 'snippet'}
    )
    if len(r['items']) == 0:
        return 'ytbapierr', 'no item', Data()
    snippet = r['items'][0]['snippet']
    pic = get_pic_url(snippet['thumbnails'])
    channelId = snippet['channelId']
    author = f'youtube-author:{channelId}'
    return 'ok', 'ok', Data(
        title=snippet['title'],
        udid=udid,
        cover=pic,
        desc=snippet['description'],
        ptime=get_ptime(snippet['publishedAt']),
        author=[author],
        author_name=[snippet['channelTitle']],
    )


def get_pic_url(thumbnails: dict) -> str:
    pic_url = None
    res_list = ['maxres', 'standard', 'high', 'medium', 'default']
    for res in res_list:
        pic = thumbnails.get(res)
        if pic is not None:
            pic_url = pic['url']
            break
    return pic_url


def get_ptime(publishedAt: str) -> str:
    # 2022-01-22T05:00:13Z
    YOUTUBE_FORMAT = '%Y-%m-%dT%H:%M:%SZ'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(publishedAt, YOUTUBE_FORMAT)
    dt_struct = dt_struct.replace(tzinfo=timezone('UTC'))
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-01-22 13:00:13 +0800
    return dt_struct.strftime(PTIME_FORMAT)
