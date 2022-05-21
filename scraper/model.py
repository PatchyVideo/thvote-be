from typing import List

from pydantic import BaseModel


class ReqBody(BaseModel):
    url: str

class BaseResp(BaseModel):
    status: str = 'ok'
    msg: str = ''

class RespBody(BaseResp):

    class Data(BaseModel):
        title: str
        udid: str
        cover: str = None
        media: List[str] = None
        desc: str = None
        ptime: str = None
        author: List[str] = None
        author_name: List[str] = None
        tname: str = None
        repost: bool = None
    
    data: Data = None
