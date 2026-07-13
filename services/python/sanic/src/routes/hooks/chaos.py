from asyncio import sleep
from random import randrange
from sanic import Request
from sanic.response import JSONResponse

from routes.errors import send_error, AppErrors


async def chaos_hook(
    _request: Request,
) -> JSONResponse | None:
    if randrange(0, 101) < 5:
        await sleep(randrange(500, 1501) / 1_000_000)
    if randrange(0, 101) < 5:
        return send_error(AppErrors.Internal)
