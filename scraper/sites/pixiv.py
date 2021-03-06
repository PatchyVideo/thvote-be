import datetime as dt

import ujson
from model import RespBody
from pixivpy_async import AppPixivAPI, PixivClient
from pytz import timezone
from utils.cache import get_cache, with_cache
from utils.parse import html_to_plain_text


@with_cache(site='pixiv', limit=0.2)
async def pixdata(pid: str, udid: str) -> RespBody:
    proxies = get_cache('proxies')
    if proxies:
        proxies = proxies.get('all://')
    async with PixivClient(proxy=proxies) as client:
        aapi = AppPixivAPI(client=client)
        await aapi.login(refresh_token=get_cache('pixiv_token'))
        info = await aapi.illust_detail(pid)
        if info.get('error'):
            return RespBody(status='apierr', msg=f'pixapierr: {ujson.encode(info["error"])}')
        data = info['illust']
        uid = data['user']['id']
        author = f'pixiv-author:{uid}'
        if data['meta_single_page']:
            media = [data['meta_single_page']['original_image_url']]
        elif data['meta_pages']:
            media = [x['image_urls']['original'] for x in data['meta_pages']]

        desc = html_to_plain_text(data['caption'])
        is_touhou = False
        bad = ''
        bad_tags = get_cache('pixiv_bad_tags')
        for tag in data['tags']:
            if not is_touhou:
                if '東方' in tag['name']:
                    is_touhou = True
            if bad_tags and not bad:
                if tag['name'] in bad_tags:
                    bad = tag['name']
            if is_touhou and bad:
                break
        status = 'ok'
        msg = ''
        if not is_touhou:
            status = 'warning'
            msg += 'may not touhou. '
        if bad:
            return RespBody(status='r18', msg=f'bad tag: {bad}')

        data = RespBody.Data(
            title=data['title'],
            udid=udid,
            cover=data['image_urls']['square_medium'].replace('pximg.net','pixiv.re'),
            media=media,
            desc=desc,
            ptime=get_ptime(data['create_date']),
            author=[author],
            author_name=[data['user']['name']],
            tname='DRAWING',
        )
        return RespBody(status=status, msg=msg, data=data)


def get_ptime(create_date: str) -> str:
    # 2022-02-08T21:02:00+09:00
    PIXIV_FORMAT = '%Y-%m-%dT%H:%M:%S+09:00'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(create_date, PIXIV_FORMAT)
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-02-08 20:02:00 +0800
    return dt_struct.strftime(PTIME_FORMAT)
