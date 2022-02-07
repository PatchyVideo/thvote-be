from pydantic import BaseModel


class ReqBody(BaseModel):
    url: str


class Data(BaseModel):
    title: str = None
    udid: str = None
    content: str = None
    desc: str = None
    ptime: str = None
    author: str = None


class RetBody(BaseModel):
    status: str = 'ok'
    msg: str = 'ok'
    data: Data
