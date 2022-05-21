from loguru import logger

from model import RespBody
from sites.acarticle import acadata
from sites.acfun import acdata
from sites.bilibili import bilidata
from sites.nicoseiga import nicoseigadata
from sites.nicovideo import nicovideodata
from sites.patchyvideo import patchydata
from sites.pixiv import pixdata
from sites.pixnovel import pixndata
from sites.thbwiki import thbdata
from sites.twitter import twidata
from sites.weibo import wbdata
from sites.youtube import ytbdata
from utils.match import (match_acarticle, match_acfun, match_bilibili,
                         match_mweibo, match_nicoseiga, match_nicovideo,
                         match_patchyvideo, match_pixiv, match_pixnovel,
                         match_thbwiki, match_twitter, match_youtube)

matcher_list = [
    (match_bilibili, bilidata),
    (match_pixiv, pixdata),
    (match_pixnovel, pixndata),
    (match_twitter, twidata),
    (match_youtube, ytbdata),
    (match_acfun, acdata),
    (match_acarticle, acadata),
    (match_nicoseiga, nicoseigadata),
    (match_nicovideo, nicovideodata),
    (match_thbwiki, thbdata),
    (match_patchyvideo, patchydata),
    (match_mweibo, wbdata),
]


async def get_data(url: str) -> RespBody:
    try:
        for matcher, praser in matcher_list:
            if wid := await matcher(url):
                resp: RespBody = await praser(wid)
                if resp.status == 'rematch':
                    resp = await get_data(resp.msg)
                return resp
        return RespBody(status='err', msg='no content found')
    except Exception as e:
        logger.exception(e)
        return RespBody(status='except', msg=repr(e))
