# scraper for touhou-vote

## support websites

check: done

blank: WIP

- [x] Bilibili
- [x] Acfun
- [x] YouTube
- [x] Twitter
- [x] Pixiv
- [x] Niconico (image)
- [x] Niconico (video)
- [ ] Weibo
- [ ] THBWiki
- [ ] PatchyVideo
- [ ] Tieba

# install & run

```
git clone
poetry install
poetry run uvicorn main:app
```

## configure

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
# API key for Google's YouTube Data API v3
# see:
# https://console.developers.google.com/apis/api/youtube.googleapis.com

[redis_config]
    host = 'localhost'
    port = 6379
    db = 0
# use redis 5.x

[proxies]
    "all://" = 'http://127.0.0.1:10809'    # socks5/socks4/http (https not supported)
    # pixiv: to use a socks proxy on windows, see:
    # https://github.com/aio-libs/aiohttp/issues/4536#issuecomment-579740877
    # or install package: aiohttp_socks
```