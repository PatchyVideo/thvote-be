from fastapi import FastAPI

from logger import *
from model import *
from process import *

app = FastAPI()


@app.post("/api", response_model=RespBody, response_model_exclude_none=True)
async def api(item: ReqBody):
    return await get_data(item.url)
