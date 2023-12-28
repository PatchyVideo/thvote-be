import datetime as dt
import re

from loguru import logger
from lxml import etree
from model import RespBody
from utils.cache import get_cache, with_cache
from utils.network import request_abroad_website

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36',
}


@with_cache(site='melonbooks', limit=0.2)
async def melondata(wid: str, udid: str) -> RespBody:
    melon_proxy = get_cache("melon_proxy") or 'www.melonbooks.co.jp'
    url = f'https://{melon_proxy}/detail/detail.php?product_id={wid}'
    r = await request_abroad_website(url, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        title = page.xpath('/html/head/title')[0].text
        if '年齢認証' in title:
            return RespBody(status='r18')
        if title.find('の通販・購入') != -1:
            title = title[:title.find('の通販・購入')]
        title = title.strip()
        media =[f"https:{image.attrib['href']}" for image in page.xpath('//div[@id="thumbs"]/ul/li/div/a')]
        if not media:
            media=None
        cover = page.xpath('/html/head/meta[@property="og:image"]')[0].attrib['content']
        desc = page.xpath('/html/head/meta[@property="og:description"]')[0].attrib['content']
        author = page.xpath('//*[@id="contents"]/div[2]/div[1]/p/a')[0].attrib['href']
        m = re.match(r'.*?_id=(\d+)', author)
        if m:
            author = f'melonbooks-author:{m.group(1)}'
        else:
            return RespBody(status='parsererr', msg=f'melonparsererr: no circle_id or maker_id')
        author_name = page.xpath('//*[@id="contents"]/div[2]/div[1]/p/a')[0].text
        time_test = page.xpath('//*[@id="title"]/div/div/div[1]/div/em/span')
        if not time_test:
            time_test = page.xpath('//*[@id="contents"]/div[2]/div[2]/div[2]/div[3]/div[5]')
        time_str = time_test[0].text
        time_str = time_str.replace('発売日：', '').strip()
            
        dt_struct = dt.datetime.strptime(time_str, '%Y年%m月%d日')
        ptime = dt_struct.strftime('%Y-%m-%d %H:%M:%S +0800')
        status = 'ok'
        msg = ''
        # tags = [tag.text for tag in page.xpath('//*[@id="title"]/div/div[2]/div[1]/div/span/span')]
    except Exception as e:
        logger.exception(e)
        return RespBody(status='parsererr', msg=f'melonparsererr: {repr(e)}')

    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        media=media,
        desc=desc,
        ptime=ptime,
        author=[author],
        author_name=[author_name],
        tname=None,
    )
    return RespBody(status=status, msg=msg, data=data)
