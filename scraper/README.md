# scraper for THVote-be

# supported websites

check: done

blank: WIP

- [x] Bilibili (video)
- [x] Acfun (video)
- [x] YouTube (video)
- [x] Twitter
- [x] Pixiv (image)
- [x] Niconico (image)
- [x] Niconico (video)
- [x] THBWiki
- [x] PatchyVideo
- [ ] Acfun (article)
- [ ] Weibo (mobile)
- [ ] Pixiv (manga)
- [ ] Pixiv (novel)

not support: weibo (PC)

# usage

## request

|method|route|parameters|
|-|-|-|
|`POST`|`/api`|`{"url":"http://example.com"}`|

## response

### response body

|field|type|example|remark|
|-|-|-|-|
|`status`|`String`|`ok`|possible values: `ok`, `err`, `warning`, `apierr`, `parsererr`, `except`
|`msg`|`String`|`ok`|
|`data`|`Object`| none |refer below

meaning of the `status`:

`ok`: ok

`err`: something wrong with the request given (e.g. cannot match any content from the url).

`warning`: ok but something need to pay attention to (in `msg`, e.g. not touhou content).

`parsererr`: match the content but something wrong while try to parse the infomation (e.g. reach rate limit, elements change cause by website upgrade).

`apierr`: match the content but something wrong while try to get information from the third party api (e.g. reach rate limit, the api itself get wrong).

`except`: program itself throw an exception. detail in `msg`.

### `data` object

|field|type|example|remark|
|-|-|-|-|
|`title`|`String`|`bad apple原版高清1440*1080`|title of the content. |
|`udid`|`String`|`bilibili:24722`|unique identifier of content. format: `site:artwork_id`. |
|`cover`|`String`|`http://i2.hdslb.com/bfs/archive/2d494d24828b82410dcb8c3f320027de86e9141a.jpg`| no cover: `acfun`. |
|`media`|`Array[String]`|`["https://pbs.twimg.com/media/FLNEMPTVUAEAu7K.jpg"]`|list of content url(s). |
|`desc`|`String`|`sina 测试一下黑屏压制。已修复。`|
|`ptime`|`String`|`2010-09-07 21:30:02 +0800`|unified as `CST` (`Asia/Shanghai`). |
|`author`|`Array[String]`|`bilibili-author:45086`|list of unique identifier of author. format: `site-author:user_id`. |
|`author_name`|`Array[String]`|`僕の可愛い殿下`|list of display name of author. |
|`tname`|`String`|`VIDEO`|type of the content. possible values: `MUSIC`, `VIDEO`, `DRAWING`, `SOFTWARE`, `ARTICLE`, `CRAFT`, `OTHER`. |
|`repost`|`Boolean`|`true`|if the content is repost or not. |

# install & run

```cmd
git clone
cd scraper
poetry install
poetry run uvicorn main:app
```

# configure

edit `config.toml`

```toml
twiapi_auth = 'twiapi_auth'
# Twitter API Auth

pixiv_token = 'pixiv_token'
# pixiv_token: Pixiv's Refresh Tokens
# see:
# https://gist.github.com/ZipFile/c9ebedb224406f4f11845ab700124362
# or
# https://gist.github.com/upbit/6edda27cb1644e94183291109b8a5fde

pixiv_bad_tags = [
    'R-18','R-18G',    # add more...
]
# when artwork have the tag(s) above, status will set to warning (but data will provide normally)

ytbapi_key = 'ytbapi_key'
# API key for google's YouTube Data API v3
# see:
# https://console.developers.google.com/apis/api/youtube.googleapis.com

[redis_config]
    host = 'localhost'
    port = 6379
    db = 0
# use redis 5.x

[proxies]
    "all://" = 'http://127.0.0.1:10809'    # socks5/socks4/http (https not supported)
    # pixiv: to use a socks proxy on Windows, see:
    # https://github.com/aio-libs/aiohttp/issues/4536#issuecomment-579740877
    # or install package: aiohttp_socks
```