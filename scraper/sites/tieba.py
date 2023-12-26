import re
import ujson
from loguru import logger
from lxml import etree
from model import RespBody
from pytz import timezone
from utils.cache import with_cache
from utils.network import request_website
from .bilibili import get_ptime

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36'}


@with_cache(site='tieba', limit=0.2)
async def tiebadata(wid: str, udid: str) -> RespBody:
    wburl = f'https://tieba.baidu.com/p/{wid}'
    r = await request_website(wburl, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        test1 = page.xpath('/html/body/script')
        test2 = ''
        for item in test1:
            if item.text is None:
                continue
            if 'pb/widget/postList' in item.text:
                test2 = item.text
                break
        test2 = test2[test2.find('pb/widget/postList')+21:]
        test2 = test2[:test2.find('_.Module.use')-2]
        test2 = test2.replace("'",'"')
        print(test2[-100:])
        test2 = test2.replace('}, }', '}}')
        print(test2[-100:])
        test2 = test2.replace('},}', '}}')
        print(test2[21111:])
        json2 = ujson.loads(test2)[0]
        post_data = json2['firstPost']
        title = post_data['title']
        ctime = post_data['now_time']
        desc = post_data['content']
        m = re.match(r'<img.*src=\"(.+?)\".*?>', desc)
        img = m.group()
        cover = m.group(1)
        desc = desc.replace(img, '').replace('<br>', '\n').strip()
        thread_data = json2['thread']
        uid = thread_data['author_info']['user_id']
        author_name = thread_data['author_info']['user_name']
        

    except Exception as e:
        logger.exception(e)
        return RespBody(status='parsererr', msg=f'tiebaparsererr: {repr(e)}')

    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        desc=desc,
        ptime=get_ptime(ctime),
        author=[f'tieba-author:{uid}'],
        author_name=[author_name],
        tname='OTHER',
    )
    return RespBody(data=data)
