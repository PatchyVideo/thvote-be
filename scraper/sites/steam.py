import datetime as dt

from loguru import logger
from lxml import etree
from model import RespBody
from utils.cache import with_cache
from utils.network import request_abroad_website

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36',
    'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
}


@with_cache(site='steam', limit=0.1)
async def steamdata(appid: str, udid: str) -> RespBody:
    url = f'https://store.steampowered.com/app/{appid}'
    r = await request_abroad_website(url, headers=header)
    if r.status_code == 302:
        return RespBody(status='err', msg=f'no content with {url}')
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        title = page.xpath('//*[@id="appHubAppName"]')[0].text
        cover = page.xpath('//*[@id="gameHeaderImageCtn"]/img')[0].attrib['src']
        cover = cover[:cover.find('?')]
        media = []
        show_list = page.xpath('//*[@id="highlight_player_area"]/div')[0]
        for item in show_list.xpath('//div[@class="highlight_player_item highlight_movie"]'):
            video = item.attrib.get('data-mp4-hd-source')
            if video:
                media.append(video[:video.find('?')])
        for item in show_list.xpath('//div/a[@class="highlight_screenshot_link"]'):
            image = item.attrib.get('href')
            if image:
                media.append(image[:image.find('?')])
        desc = page.xpath('/html/head/meta[@property="og:description"]')[0].attrib['content']
        author_name = page.xpath('//div[@id="developers_list"]/a')[0].text
        author = f'steam-author:{author_name}'
        time_str = page.xpath('//div[@class="release_date"]/div[@class="date"]')[0].text
        dt_struct = dt.datetime.strptime(time_str, '%Y 年 %m 月 %d 日')
        ptime = dt_struct.strftime('%Y-%m-%d %H:%M:%S +0800')
    except Exception as e:
        logger.exception(e)
        return RespBody(status='parsererr', msg=f'acaparsererr: {repr(e)}')

    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        media=media,
        desc=desc,
        ptime=ptime,
        author=[author],
        author_name=[author_name],
        tname='SOFTWARE',
    )
    return RespBody(data=data)
