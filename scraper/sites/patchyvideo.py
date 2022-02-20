import datetime as dt
from typing import Tuple

from model import Data
from pytz import timezone
from utils.cache import with_cache
from utils.network import request_abroad_api

api = 'https://patchyvideo.com/graphql'
gql = '''
query ($vid: String!) {
  getVideo(para: { vid: $vid, lang: "CHS" }) {
    item {
      title
      site
      url
      coverImage
      desc
      repostType
      thumbnailUrl
      uploadTime
      userSpaceUrls
    }
    tagByCategory(lang:"CHS") {
      key
      value
    }
  }
}
'''


@with_cache(site='patchyvideo')
async def patchydata(vid: str, udid: str) -> Tuple[str, str, Data]:
    resp = await request_abroad_api(api, json={
        'query': gql,
        'variables': {'vid': vid}
    })
    data = resp.get('data')
    if not data:
        return 'apierr', f'patchyapierr: {resp.get("errors")}', Data()

    item = data['getVideo']['item']
    tags = data['getVideo']['tagByCategory']
    site = item['site']
    if site in ['bilibili', 'nicovideo', 'youtube', 'twitter', 'acfun', 'weibo']:
        return 'rematch', item['url'], Data()

    authors = None
    for tag in tags:
        if tag['key'] == 'AUTHOR':
            authors = tag['value']
            break
    if item['repostType'] == 'original':
        repost = True
    else:
        repost = False

    # if site in []:

    return 'ok', 'ok', Data(
        title=item['title'],
        udid=udid,
        cover=item['coverImage'],
        desc=item['desc'],
        ptime=get_ptime(item['uploadTime']),
        author_name=authors,
        repost=repost
    )


def get_ptime(uploadTime: str) -> str:
    # 2022-02-14T13:20:28+00:00
    PATCHY_FORMAT = '%Y-%m-%dT%H:%M:%S%z'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(uploadTime, PATCHY_FORMAT)
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-02-14 21:20:28 +0800
    return dt_struct.strftime(PTIME_FORMAT)
