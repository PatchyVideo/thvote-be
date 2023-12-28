from utils.cache import get_cache


async def bvid_converter(bvid=None, aid=None) -> str:
    '''aid和bvid互转'''
    if bvid is None and aid is None:
        return None
    table = 'fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF'
    tr = {}
    for i in range(58):
        tr[table[i]] = i
    s = [11, 10, 3, 8, 4, 6]
    xor = 177451812
    add = 8728348608

    if bvid:
        r = 0
        for i in range(6):
            r += tr[bvid[s[i]]]*58**i
        return (r-add) ^ xor

    if aid:
        if type(aid) is str:
            if aid.isdigit():
                aid = int(aid)
            else:
                return None
        else:
            if type(aid) is not int:
                return None
        aid = (aid ^ xor)+add
        r = list('BV1  4 1 7  ')
        for i in range(6):
            r[s[i]] = table[aid//58**i % 58]
        return ''.join(r)


def get_header() -> dict:
    return {
        'origin': 'https://space.bilibili.com',
        'referer': 'https://www.bilibili.com/read/cv2/',
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36',
        'accept-language': 'en,zh-CN;q=0.9,zh;q=0.8',
    }


def get_cookies() -> dict:
    return {
        'buvid3': get_cache('bilibili_config')['bili_jct'],
        'b_nut': get_cache('bilibili_config')['DedeUserID'],
    }
