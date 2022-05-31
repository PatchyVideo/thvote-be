import datetime as dt

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
    url = f'https://{get_cache("melon_proxy")}/detail/detail.php?product_id={wid}'
    r = await request_abroad_website(url, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        title = page.xpath('/html/head/title')[0].text
        if '年齢認証' in title:
            return RespBody(status='r18')
        if title.find('の通販・購入') != -1:
            title = title[:title.find('の通販・購入')]
        media =[f"https:{image.attrib['href']}" for image in page.xpath('//div[@id="thumbs"]/ul/li/div/a')]
        if not media:
            media=None
        cover = page.xpath('/html/head/meta[@property="og:image"]')[0].attrib['content']
        desc = page.xpath('/html/head/meta[@property="og:description"]')[0].attrib['content']
        author = page.xpath('//*[@id="title"]/div/div/div[1]/div/a')[0].attrib['href']
        find = author.find('circle_id')
        if find != -1:
            author = f'melonbooks-author:{author[find+10:]}'
        else:
            return RespBody(status='parsererr', msg=f'melonparsererr: no circle_id')
        author_name = page.xpath('//*[@id="title"]/div/div/div[1]/div/a')[0].text
        time_str = page.xpath('//*[@id="title"]/div/div/div[1]/div/em/span')[0].text
        dt_struct = dt.datetime.strptime(time_str, '%Y年%m月%d日')
        ptime = dt_struct.strftime('%Y-%m-%d %H:%M:%S +0800')
        status = 'success'
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
