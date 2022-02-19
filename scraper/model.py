from typing import List

from pydantic import BaseModel


class ReqBody(BaseModel):
    url: str


class Data(BaseModel):
    title: str = None
    udid: str = None
    cover: str = None
    media: List[str] = None
    desc: str = None
    ptime: str = None
    author: List[str] = None
    author_name: List[str] = None
    repost: bool = None


class RetBody(BaseModel):
    status: str = 'ok'
    msg: str = 'ok'
    data: Data
