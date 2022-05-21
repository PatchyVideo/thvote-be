import datetime as dt

import ujson
from lxml import etree
from model import RespBody
from pytz import timezone
from utils.cache import with_cache
from utils.network import request_website

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36'}


@with_cache(site='weibo', limit=0.2)
async def wbdata(wid: str, udid: str) -> RespBody:
    wburl = f'https://m.weibo.cn/detail/{wid}'
    r = await request_website(wburl, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        test = page.xpath('/html/body/script[1]')[0].text
        test = test[test.find('$render_data'):]
        test = test[test.find('[{')+1:test.find('}]')+1]
        json = ujson.loads(test)
        data = json['status']
        created_at = data['created_at']
        uid = data['user']['id']
        author = f'weibo-author:{uid}'

    except Exception as e:
        return RespBody(status='parsererr', msg=f'acparsererr: {repr(e)}')

    data = RespBody.Data(
        title=f'{data["user"]["screen_name"]}的微博',
        udid=udid,
        # thumbnail_pic, bmiddle_pic, original_pic
        cover=data['bmiddle_pic'],
        desc=data['text'],
        ptime=get_ptime(created_at),
        author=[author],
        author_name=[data['user']['screen_name']],
        tname='DRAWING',
    )
    return RespBody(data=data)


def get_ptime(created_at: str) -> str:
    # Tue Jan 25 23:29:47 +0800 2022
    WEIBO_FORMAT = '%a %b %d %H:%M:%S %z %Y'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(created_at, WEIBO_FORMAT)
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-01-25 23:29:47 +0800
    return dt_struct.strftime(PTIME_FORMAT)
