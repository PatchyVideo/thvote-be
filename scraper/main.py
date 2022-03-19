from fastapi import FastAPI

from logger import *
from model import *
from process import *

app = FastAPI()


@app.post("/api", response_model=RetBody, response_model_exclude_none=True)
async def api(item: ReqBody):
    status, msg, data = await get_data(item.url)
    return RetBody(status=status or 'ok', msg=msg or 'ok', data=data)
