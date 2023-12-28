from model import RespBody
from utils.cache import with_cache, get_cache
from utils.network import request_api
import time

biliconfig = get_cache('bilibili_config')
header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36'}


@with_cache(site='bilibili', limit=0.2)
async def bilidata(aid: str, udid: str) -> RespBody:
    '''根据aid(av号)获取视频相关数据'''
    api = f'https://api.bilibili.com/x/web-interface/view?aid={aid}'
    r = await request_api(api, headers=header, cookies={'SESSDATA': biliconfig['SESSDATA']})
    data = r.get('data')
    if data is None:
        return RespBody(status='apierr', msg=f'biliapimsg: {r["message"]}')

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
    area = data['tname']
    tname = 'VIDEO'
    if area in music_area:
        tname = 'MUSIC'
    elif area == '绘画':
        tname = 'DRAWING'
    elif area == '手工':
        tname = 'CRAFT'
    data = RespBody.Data(
        title=data['title'],
        udid=udid,
        cover=data['pic'],
        desc=data['desc'],
        ptime=get_ptime(data['pubdate']),
        author=author,
        author_name=author_name,
        repost=repost,
        tname=tname,
    )
    return RespBody(msg=f'biliapimsg: {r["message"]}', data=data)


def get_ptime(ctime: int) -> str:
    '''根据时间戳(int类型)获取投稿时间'''
    # 2022-02-07 13:34:53 +0800
    return time.strftime(
        "%Y-%m-%d %H:%M:%S %z", time.localtime(ctime))


music_area = [
    '原创音乐',
    '翻唱',
    '演奏',
    'VOCALOID·UTAU',
    '音乐现场',
    'MV',
    '乐评盘点',
    '音乐教学',
    '音乐综合',
    '音频',
    '说唱',
]
