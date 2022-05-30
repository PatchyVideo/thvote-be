import datetime as dt

from loguru import logger
from lxml import etree
from model import RespBody
from utils.cache import with_cache
from utils.network import request_abroad_website

header = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36',
}

soft_tags = ['アクション', 'クイズ', 'アドベンチャー', 'ロールプレイング', 'テーブル', 'デジタルノベル', 'シミュレーション', 'タイピング', 'シューティング', 'パズル', 'その他ゲーム', 'ツール/アクセサリ']
image_tags = ['マンガ', '劇画', 'WEBTOON', 'CG・イラスト', '画像素材']
text_tags = ['ノベル', '官能小説']
video_tags = ['動画', 'ボイスコミック']
audio_tags = ['音楽', '音素材', 'ボイス・ASMR']


@with_cache(site='dlsite', limit=0.1)
async def dlsitedata(rjid: str, udid: str) -> RespBody:
    url = f'https://www.dlsite.com/home/work/=/product_id/{rjid}'
    r = await request_abroad_website(url, headers=header)
    html = r.content.decode('utf-8')
    try:
        page = etree.HTML(html)
        if page is None:
            return RespBody(status='r18')
        title = page.xpath('//*[@id="work_name"]')[0].text
        media = []
        cover = None
        for image in page.xpath('//div[@class="product-slider-data"]/div'):
            img = image.attrib.get('data-src')
            if img:
                media.append(img.replace('//','https://'))
            if 'img_main' in img:
                cover = image.attrib.get('data-thumb')
        if cover:
            cover = cover.replace('//','https://')
        else:
            if media:
                cover = media[0]
            else:
                media = None
        desc = page.xpath('/html/head/meta[@property="og:description"]')[0].attrib['content']
        maker = page.xpath('//span[@class="maker_name"]/a')[0]
        author = maker.attrib['href']
        author = author[author.find('maker_id')+9:].replace('.html','')
        author_name = maker.text
        table = page.xpath('//table[@id="work_outline"]')[0]
        heads = page.xpath('//table[@id="work_outline"]/tr/th/text()')
        status = 'ok'
        msg = ''
        tname = 'OTHER'
        for index, head in enumerate(heads):
            if head == '販売日':
                time_str = table.xpath(f'//tr[{index+1}]/td/a')[0].text
                time_str = time_str[:time_str.find('日')+1]
                continue
            if head == '年齢指定':
                age = table.xpath(f'//tr[{index+1}]/td/div/span')[0].text
                if age != '全年齢':
                    return RespBody(status='r18')
                continue
            if head == '作品形式':
                work_type = table.xpath(f'//tr[{index+1}]/td/div/a/span')[0].text
                if work_type in soft_tags:
                    tname = 'SOFTWARE'
                elif work_type in image_tags:
                    tname = 'DRAWING'
                elif work_type in text_tags:
                    tname = 'ARTICLE'
                elif work_type in video_tags:
                    tname = 'VIDEO'
                elif work_type in audio_tags:
                    tname = 'MUSIC'
                continue
            if head == 'ジャンル':
                tags = table.xpath(f'//tr[{index+1}]/td/div/a//text()')
                if '東方Project' not in tags:
                    status = 'warning'
                    msg='may not touhou'
                continue

        dt_struct = dt.datetime.strptime(time_str, '%Y年%m月%d日')
        ptime = dt_struct.strftime('%Y-%m-%d %H:%M:%S +0800')
    except Exception as e:
        logger.exception(e)
        return RespBody(status='parsererr', msg=f'dlsiteparsererr: {repr(e)}')

    data = RespBody.Data(
        title=title,
        udid=udid,
        cover=cover,
        media=media,
        desc=desc,
        ptime=ptime,
        author=[author],
        author_name=[author_name],
        tname=tname,
    )
    return RespBody(status=status, msg=msg, data=data)
