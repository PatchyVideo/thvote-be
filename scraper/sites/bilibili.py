from typing import Tuple

from model import Data
from utils.cache import with_cache
from utils.network import request_api
import time


@with_cache(site='bilibili', limit=0.2)
async def bilidata(aid: str, udid: str) -> Tuple[str, str, Data]:
    '''根据aid(av号)获取视频相关数据'''
    api = f'https://api.bilibili.com/x/web-interface/view?aid={aid}'
    r = await request_api(api)
    data = r.get('data')
    if data is None:
        return 'biliapierr', f"bilimsg: {r['message']}", Data()

    staffs = data.get('staff')
    if staffs:
        author = [f'bilibili-author:{x["mid"]}' for x in staffs]
        author_name = [x["name"] for x in staffs]
    else:
        uid = data['owner']['mid']
        author = [f'bilibili-author:{uid}']
        author_name = [data['owner']['name']]
    if data['copyright'] == 1:
        repost = False
    else:
        repost = True
    return 'ok', f"bilimsg: {r['message']}", Data(
        title=data['title'],
        udid=udid,
        cover=data['pic'],
        desc=data['desc'],
        ptime=get_ptime(data['pubdate']),
        author=author,
        author_name=author_name,
        repost=repost
    )


def get_ptime(ctime: int) -> str:
    '''根据时间戳(int类型)获取投稿时间'''
    # 2022-02-07 13:34:53 +0800
    return time.strftime(
        "%Y-%m-%d %H:%M:%S %z", time.localtime(ctime))
