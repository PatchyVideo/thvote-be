import datetime as dt
from datetime import timedelta
from typing import Tuple

from model import Data
from pytz import timezone
from utils.cache import get_cache, set_cache, with_cache
from utils.network import request_abroad_api


async def get_token() -> str:
    token = get_cache('twiapi_token')
    if token:
        return token
    api = 'https://api.twitter.com/1.1/guest/activate.json'
    header = {'authorization': get_cache('twiapi_auth')}
    resp = await request_abroad_api(api,
                                    headers=header,
                                    my_metod='post')
    token = resp['guest_token']
    set_cache('twiapi_token', token, timedelta(minutes=30))
    return token


@with_cache(site='twitter')
async def twidata(tid: str, udid: str) -> Tuple[str, str, Data]:
    api = f'https://api.twitter.com/1.1/statuses/show.json?id={tid}&tweet_mode=extended&include_entities=true'
    header = {
        'authorization': get_cache('twiapi_auth'),
        'x-guest-token': await get_token()
    }
    resp = await request_abroad_api(api, headers=header)
    if errors := resp.get('errors'):
        return 'apierr', f'twiapierr: {errors}'
    created_at = resp['created_at']
    uid = resp['user']['id_str']
    author = f'twitter-author:{uid}'
    raw_media = resp['entities']['media']
    return 'ok', 'ok', Data(
        title=f'{resp["user"]["name"]}的推文',
        udid=udid,
        desc=resp['full_text'],
        cover=raw_media[0]['media_url_https'],
        media=[m['media_url_https'] for m in raw_media],
        ptime=get_ptime(created_at),
        author=[author],
        author_name=[resp['user']['name']],
    )


def get_ptime(created_at: str) -> str:
    # Sun Feb 06 20:14:46 +0000 2022
    TWITTER_FORMAT = '%a %b %d %H:%M:%S %z %Y'
    PTIME_FORMAT = '%Y-%m-%d %H:%M:%S %z'
    dt_struct = dt.datetime.strptime(created_at, TWITTER_FORMAT)
    dt_struct = dt_struct.astimezone(timezone('Asia/Shanghai'))
    # 2022-02-07 04:14:46 +0800
    return dt_struct.strftime(PTIME_FORMAT)
