import datetime as dt

from model import RespBody
from pytz import timezone
from utils.cache import get_cache, with_cache
from utils.network import request_abroad_api


@with_cache(site='youtube')
async def ytbdata(vid: str, udid: str) -> RespBody:
    api = 'https://www.googleapis.com/youtube/v3/videos'
    resp = await request_abroad_api(
        api,
        params={
            'key': get_cache('ytbapi_key'),
            'id': vid,
            'part': 'snippet'}
    )
    if len(resp['items']) == 0:
        return RespBody(status='apierr', msg='ytbapierr: no such content')
    snippet = resp['items'][0]['snippet']
    pic = get_pic_url(snippet['thumbnails'])
    channelId = snippet['channelId']
    author = f'youtube-author:{channelId}'
    data = RespBody.Data(
        title=snippet['title'],
        udid=udid,
        cover=pic,
        desc=snippet['description'],
        ptime=get_ptime(snippet['publishedAt']),
        author=[author],
        author_name=[snippet['channelTitle']],
    )
    return RespBody(data=data)


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
