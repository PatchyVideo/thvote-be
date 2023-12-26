import re
import time
from urllib.parse import quote, unquote

import ujson
from model import RespBody
from utils.cache import get_cache, set_cache, with_cache
from utils.network import request_abroad_api, request_abroad_website

api = 'https://thwiki.cc/api.php'
udid_format = 'thbwiki:{entry}'


@with_cache(site='thbwiki')
async def thbdata(entry: str, udid: str) -> RespBody:
    short = None
    urlen = None
    jump = None
    if '%' in entry:
        urlen = entry
        entry = unquote(urlen)
    if entry[:2] == '-/':
        short = entry
        entry = await prase_short(short)
    resp = await request_abroad_website(api, data={
        'action': 'ask',
        'format': 'json',
        'formatversion': 2,
        'query': f'[[{entry}]]|?封面图片|?专辑名称|?同人志名称|?视频名称|?软件名称|?发售日期|?制作方|?发售方|?出品方|?原画师|?模型名称'
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
    author_list = d['制作方'] + d['发售方'] + d['出品方'] + d['原画师']
    author_list = [x['fulltext'] for x in author_list]
    author = []
    [author.append(x) for x in author_list if x not in author]
    author_name = [f'thbwiki-author:{x}' for x in author]

    tname = 'OTHER'
    if d.get('专辑名称'):
        tname = 'MUSIC'
    elif d.get('同人志名称'):
        tname = 'DRAWING'
    elif d.get('视频名称'):
        tname = 'VIDEO'
    elif d.get('软件名称'):
        tname = 'SOFTWARE'
    elif d.get('模型名称'):
        tname = 'CRAFT'

    fulltext = data['fulltext']
    if fulltext != entry:
        jump = entry
        entry = fulltext
    udid = udid_format.format(entry=fulltext)
    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        ptime=ptime,
        author=author,
        author_name=author_name,
        tname=tname,
    )
    ret = RespBody(data=data)
    if short:
        set_cache(udid_format.format(entry=short), ret)
    if urlen:
        set_cache(udid_format.format(entry=urlen), ret)
    else:
        set_cache(udid_format.format(entry=quote(entry)), ret)
    if jump:
        set_cache(udid_format.format(entry=jump), ret)
    return ret


async def prase_short(short: str) -> str:
    short = short.replace('-/', '')
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
