from typing import Tuple

from model import Data
from sites.acfun import acdata
from sites.bilibili import bilidata
from sites.nicoseiga import nicoseigadata
from sites.nicovideo import nicovideodata
from sites.pixiv import pixdata
from sites.thbwiki import thbdata
from sites.twitter import twidata
from sites.youtube import ytbdata
from utils.match import (match_acfun, match_bilibili, match_nicoseiga,
                         match_nicovideo, match_pixiv, match_thbwiki,
                         match_twitter, match_youtube)

matcher_list = [
    (match_bilibili, bilidata),
    (match_pixiv, pixdata),
    (match_twitter, twidata),
    (match_youtube, ytbdata),
    (match_acfun, acdata),
    (match_nicoseiga, nicoseigadata),
    (match_nicovideo, nicovideodata),
    (match_thbwiki, thbdata),
]


async def get_data(url: str) -> Tuple[str, str, Data]:
    # try:
        for matcher, praser in matcher_list:
            if wid := await matcher(url):
                return await praser(wid)
        return 'err', 'no content found', Data()
    # except Exception as e:
    #     return 'except', repr(e), Data()
