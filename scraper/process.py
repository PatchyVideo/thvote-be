from loguru import logger

from model import RespBody
from sites.acarticle import acadata
from sites.acfun import acdata
from sites.bilibili import bilidata
from sites.dizzylab import dizzydata
from sites.dlsite import dlsitedata
from sites.melon import melondata
from sites.nicoseiga import nicoseigadata
from sites.nicovideo import nicovideodata
from sites.patchyvideo import patchydata
from sites.pixiv import pixdata
from sites.pixnovel import pixndata
from sites.steam import steamdata
from sites.thbwiki import thbdata
from sites.twitter import twidata
from sites.weibo import wbdata
from sites.youtube import ytbdata
from utils.match import *

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
    (match_dizzy, dizzydata),
    (match_steam, steamdata),
    (match_dlsite, dlsitedata),
    (match_melon, melondata),
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
