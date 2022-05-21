import re
import time
from urllib.parse import unquote

import ujson
from model import RespBody
from utils.cache import with_cache
from utils.network import request_abroad_api, request_abroad_website

api = 'https://thwiki.cc/api.php'


@with_cache(site='thbwiki')
async def thbdata(entry: str, udid: str) -> RespBody:
    if '%' in entry:
        entry = unquote(entry)
    if 'http' in entry:
        entry = await prase_short(entry)
    resp = await request_abroad_website(api, data={
        'action': 'ask',
        'format': 'json',
        'formatversion': 2,
        'query': f'[[{entry}]]|?封面图片|?发售日期|?制作方'
    })
    r = ujson.loads(resp.content.decode('utf-8'))
    result = r['query']['results']
    if not result:
        return RespBody(status='apierr', msg=f'thbapierr: no result for {entry}')
    data = list(result.values())[0]
    d = data['printouts']
    title = data['fulltext']
    udid = f'thbwiki:{title}'
    pic = d['封面图片']
    cover = None
    if pic:
        if d['封面图片'][0]['exists'] == '1':
            cover_entry = d['封面图片'][0]['fulltext'].replace('文件:', '')
            cover = await get_cover(cover_entry)

    ptime = None
    release_date = d['发售日期']
    if release_date:
        ctime = release_date[0]['timestamp']
        ptime = time.strftime(
            "%Y-%m-%d %H:%M:%S %z", time.localtime(int(ctime)))
    author = None
    author_name = None
    producer = d['制作方']
    if producer:
        author_name = producer[0]['fulltext']
        author = f'thbwiki-author:{author_name}'

    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        ptime=ptime,
        author=[author],
        author_name=[author_name],
    )
    return RespBody(data=data)


async def prase_short(link: str) -> str:
    short = re.match(r'.*thwiki.cc/-/(\w+)', link).group(1)
    pageid = short2pageid(short)
    resp = await request_abroad_api(api, data={
        'action': 'parse',
        'format': 'json',
        'pageid': pageid,
        'formatversion': 2,
        'prop': 'displaytitle',
    })
    return resp['parse']['title']


async def get_cover(file_entry: str) -> str:
    resp = await request_abroad_api(api, data={
        'action': 'parse',
        'format': 'json',
        'text': '{{filepath:FILE_ENTRY | 378}}'.replace('FILE_ENTRY', file_entry),
        'formatversion': 2,
        'prop': 'text',
        'disablelimitreport': '1',
        'disableeditsection': '1',
        'disablestylededuplication': '1',
        'disabletoc': '1'
    })
    if match_url := re.match(r'.*"(http.+?)"', resp['parse']['text']):
        return match_url.group(1)


def short2pageid(short: str) -> int:
    code = '0123456789abcdefghijklmnopqrstuvwxyz'
    result = 0
    for b, n in enumerate(short):
        result += code.find(n) * 32 ** (len(short)-b-1)
    return result
