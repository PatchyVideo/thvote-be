import re
from typing import Tuple

from model import Data
from utils import get_post_time, get_redirect_url
from utils.network import request_api


async def bilidata(aid: str) -> Tuple[str, str, Data]:
    '''根据aid(BV号)获取视频相关数据'''
    api = f'https://api.bilibili.com/x/web-interface/view?aid={aid}'
    r = await request_api(api)
    data = r.get('data')
    if data is None:
        return 'biliapierr', f"bilimsg: {r['message']}", Data()

    ptime = get_post_time(data['pubdate'])
    uid = data['owner']['mid']
    author = f'bilibili-author:{uid}'
    return 'ok', f"bilimsg: {r['message']}", Data(
        title=data['title'],
        udid=f'av{aid}',
        desc=data['desc'],
        ptime=ptime,
        author=author
    )


async def get_aid(text: str) -> str:
    '''获取文本中的第一个aid（av号）'''
    aid = None
    if match_b23url := re.match(r'.*((?:http|https)://(?:(?:bili(?:22|23|33|2233).cn)|(?:b23.tv))/[A-Za-z0-9]+)', text, re.DOTALL):
        # 短链接
        b23url = match_b23url.group(1)
        text = await get_redirect_url(b23url)
    if match_aid := re.match(r'.*(?<![A-Za-z0-9])(?:AV|av)(\d+)', text, re.DOTALL):
        # av号
        aid = match_aid.group(1)
    if match_bvid := re.match(r'.*(?<![A-Za-z0-9])(BV[A-Za-z0-9]{10})(?![A-Za-z0-9])', text, re.DOTALL):
        # bv号
        bvid = match_bvid.group(1)
        aid = await bvid_converter(bvid=bvid)
    return aid


async def bvid_converter(bvid=None, aid=None) -> str:
    '''aid和bvid互转'''
    if bvid is None and aid is None:
        return None
    table = 'fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF'
    tr = {}
    for i in range(58):
        tr[table[i]] = i
    s = [11, 10, 3, 8, 4, 6]
    xor = 177451812
    add = 8728348608

    if bvid:
        r = 0
        for i in range(6):
            r += tr[bvid[s[i]]]*58**i
        return (r-add) ^ xor

    if aid:
        if type(aid) is str:
            if aid.isdigit():
                aid = int(aid)
            else:
                return None
        else:
            if type(aid) is not int:
                return None
        aid = (aid ^ xor)+add
        r = list('BV1  4 1 7  ')
        for i in range(6):
            r[s[i]] = table[aid//58**i % 58]
        return ''.join(r)
