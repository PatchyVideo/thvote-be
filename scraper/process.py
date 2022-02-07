from typing import Tuple

from model import Data
from sites.bilibili import bilidata, get_aid
from sites.twitter import get_tid, twidata
from utils.match import match_bilibili, match_twitter


async def get_data(url: str) -> Tuple[str, str, Data]:
    try:
        if await match_bilibili(url):
            aid = await get_aid(url)
            if aid is not None:
                return await bilidata(aid)
        if await match_twitter(url):
            tid = await get_tid(url)
            if tid is not None:
                return await twidata(tid)
        return 'err', 'no content found', Data()
    except Exception as e:
        return 'except', repr(e), Data()
