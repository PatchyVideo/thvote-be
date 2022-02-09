from typing import Tuple

from model import Data
from sites.bilibili import bilidata
from sites.pixiv import pixdata
from sites.twitter import twidata
from sites.youtube import ytbdata
from utils.match import (match_bilibili, match_pixiv, match_twitter,
                         match_youtube)

matcher_list = [
    (match_bilibili, bilidata),
    (match_pixiv, pixdata),
    (match_twitter, twidata),
    (match_youtube, ytbdata),
]


async def get_data(url: str) -> Tuple[str, str, Data]:
    try:
        for matcher, praser in matcher_list:
            if wid := await matcher(url):
                return await praser(wid)
        return 'err', 'no content found', Data()
    except Exception as e:
        return 'except', repr(e), Data()
