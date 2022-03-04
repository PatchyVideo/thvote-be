import re

from .biliutils import bvid_converter
from .network import get_redirect_url


async def match_bilibili(text: str) -> str:
    if match_b23url := re.match(r'.*((?:http|https)://(?:(?:bili(?:22|23|33|2233).cn)|(?:b23.tv))/[A-Za-z0-9]+)', text, re.DOTALL):
        # 短链接
        b23url = match_b23url.group(1)
        text = await get_redirect_url(b23url)
    if match_aid := re.match(r'.*(?<![A-Za-z0-9])(?:AV|av)(\d+)', text, re.DOTALL):
        # av号
        return match_aid.group(1)
    if match_bvid := re.match(r'.*(?<![A-Za-z0-9])(BV[A-Za-z0-9]{10})(?![A-Za-z0-9])', text, re.DOTALL):
        # bv号
        return await bvid_converter(bvid=match_bvid.group(1))


async def match_twitter(text: str) -> str:
    if match_normal := re.match(r'.*twitter\.com/[^/]+/status/(\d+)', text):
        return match_normal.group(1)


async def match_pixiv(text: str) -> str:
    if match_normal := re.match(r'.*(?:pixiv|pixivdl).net/(?:(?:(?:artworks|i)/)|member_illust.php?.*id=)([0-9]+)', text):
        return match_normal.group(1)


async def match_youtube(text: str) -> str:
    if match_normal := re.match(r'.*(?:youtu.be/|youtube.com/watch\?v=)([-\w]+)', text):
        return match_normal.group(1)


async def match_acfun(text: str) -> str:
    if match_normal := re.match(r'.*acfun.cn/v/(?:ac|\?ac=)(\d+)', text):
        return match_normal.group(1)


async def match_acarticle(text: str) -> str:
    if match_normal := re.match(r'.*acfun.cn/a/(?:ac|\?ac=)(\d+)', text):
        return match_normal.group(1)


async def match_nicoseiga(text: str) -> str:
    if match_normal := re.match(r'.*seiga/im(\d+)', text):
        return match_normal.group(1)


async def match_nicovideo(text: str) -> str:
    if match_normal := re.match(r'.*nicovideo.jp/watch/sm(\d+)', text):
        return match_normal.group(1)


async def match_thbwiki(text: str) -> str:
    if match_short := re.match(r'.*thwiki.cc/-/\w+', text):
        return match_short.group()
    if match_normal := re.match(r'.*thwiki.cc/([\u4e00-\u9fa5A-Za-z0-9%]+)', text):
        return match_normal.group(1)


async def match_patchyvideo(text: str) -> str:
    if match_normal := re.match(r'.*(?:thvideo.tv|patchyvideo.com)/#/video\?id=(\w+)', text):
        return match_normal.group(1)
    if match_dev := re.match(r'.*platinum.vercel.app/video/(\w+)', text):
        return match_dev.group(1)
