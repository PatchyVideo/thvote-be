from typing import Tuple

from loguru import logger

from model import Data
from sites.acfun import acdata
from sites.acarticle import acadata
from sites.bilibili import bilidata
from sites.nicoseiga import nicoseigadata
from sites.nicovideo import nicovideodata
from sites.patchyvideo import patchydata
from sites.pixiv import pixdata
from sites.thbwiki import thbdata
from sites.twitter import twidata
from sites.youtube import ytbdata
from utils.match import (match_acarticle, match_acfun, match_bilibili, match_nicoseiga,
                         match_nicovideo, match_patchyvideo, match_pixiv,
                         match_thbwiki, match_twitter, match_youtube)

matcher_list = [
    (match_bilibili, bilidata),
    (match_pixiv, pixdata),
    (match_twitter, twidata),
    (match_youtube, ytbdata),
    (match_acfun, acdata),
    (match_acarticle,acadata),
    (match_nicoseiga, nicoseigadata),
    (match_nicovideo, nicovideodata),
    (match_thbwiki, thbdata),
    (match_patchyvideo, patchydata)
]


async def get_data(url: str) -> Tuple[str, str, Data]:
    try:
        for matcher, praser in matcher_list:
            if wid := await matcher(url):
                ret = await praser(wid)
                if ret[0] == 'rematch':
                    ret = await get_data(ret[1])
                return ret
        return 'err', 'no content found', Data()
    except Exception as e:
        logger.exception(e)
        return 'except', repr(e), Data()
