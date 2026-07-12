from asyncio import sleep
from fastapi import Request
from random import randrange

from routes.errors import send_error, AppErrors


async def chaos_middleware(request: Request, call_next):
    if randrange(0, 101) < 5:
        await sleep(randrange(500, 1501) / 1_000_000)
    if randrange(0, 101) < 5:
        return send_error(AppErrors.Internal)

    return await call_next(request)
