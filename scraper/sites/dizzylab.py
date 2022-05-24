import datetime as dt
from loguru import logger

from lxml import etree
from model import RespBody
from utils.cache import with_cache
from utils.network import request_website

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36'}


@with_cache(site='dizzylab', limit=0.2)
async def dizzydata(wid: str, udid: str) -> RespBody:
    url = f'https://www.dizzylab.net/d/{wid}/'
    r = await request_website(url, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        pagetitle = page.xpath('/html/head/title')[0].text
        if '出错了' in pagetitle:
            return RespBody(status='err', msg=f'error when request {url}')
        title = page.xpath('//div[@class="col"]/h1')[0].text
        cover = page.xpath('/html/head/link[@rel="shortcut icon"]')[0].attrib['href']

        media = [x.attrib['data-audio'] for x in page.xpath('//ul[@class="playlist--list"]/li')]
        if len(media) == 1 and not media[0]:
            media = None
        desc = page.xpath('/html/head/meta[@name="description"]')[0].attrib['content']
        author_name = page.xpath('//div[@class="col"]/h4[1]/a')[0].text.replace('@ ','')
        author = f'dizzylab-author:{author_name}'
        time_str = page.xpath('//div[@class="col"]/p[@class="text-left"]')[1].text
        time_s = time_str.find('发布于')
        time_e = time_str.find('日')
        ptime = None
        if time_s != -1 and time_e != -1:
            time_str = time_str[time_s+3:time_e+1]
            dt_struct = dt.datetime.strptime(time_str, '%Y年%m月%d日')
            ptime = dt_struct.strftime('%Y-%m-%d %H:%M:%S +0800')
    except Exception as e:
        logger.exception(e)
        return RespBody(status='parsererr', msg=f'dizzyaparsererr: {repr(e)}')

    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        media=media,
        desc=desc,
        ptime=ptime,
        author=[author],
        author_name=[author_name],
        tname='MUSIC',
    )
    return RespBody(data=data)
