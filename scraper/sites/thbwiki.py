from urllib.parse import unquote
from typing import Tuple

import time
import ujson
from model import Data
from utils.cache import with_cache
from utils.network import request_abroad_website, get_redirect_url
from utils.match import match_thbwiki

api = 'https://thwiki.cc/api.php'

@with_cache(site='thbwiki')
async def thbdata(entry: str, udid: str) -> Tuple[str, str, Data]:
    if 'http' in entry:
        entry = await match_thbwiki(await get_redirect_url(entry))
    if '%' in entry:
        entry = unquote(entry)
    resp = await request_abroad_website(api, data={
        'action': 'ask',
        'format': 'json',
        'formatversion': 2,
        'query': f'[[{entry}]]|?封面图片|?发售日期|?制作方'
    })
    r = ujson.loads(resp.content.decode('utf-8'))
    result = r['query']['results']
    if not result:
        return 'thbapierr', f'no result for "{entry}"', Data()
    data = list(result.values())[0]
    d = data['printouts']
    title = data['fulltext']
    udid = f'thbwiki:{title}'
    pic = d['封面图片']
    cover = None
    if pic:
        cover =  d['封面图片'][0]['fullurl']

    ptime = None
    release_date = d['发售日期']
    if release_date:
        ctime = release_date[0]['timestamp']
        ptime = time.strftime(
            "%Y-%m-%d %H:%M:%S %z", time.localtime(int(ctime)))
    author = None
    producer = d['制作方']
    if producer:
        author = f"thbwiki-author:{d['制作方'][0]['fulltext']}"

    return 'ok', 'ok', Data(
        title=title,
        udid=udid,
        cover=cover,
        ptime=ptime,
        author=author,
    )
