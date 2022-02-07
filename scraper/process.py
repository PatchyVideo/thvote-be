from typing import Tuple

from model import Data
from sites.bilibili import bilidata, get_aid
from utils.match import match_bilibili


async def get_data(url: str) -> Tuple[str, str, Data]:
    try:
        if match_bilibili(url):
            aid = await get_aid(url)
            if aid is not None:
                return await bilidata(aid)
        return 'err', 'no content found', Data()
    except Exception as e:
        return 'except', repr(e), Data()
