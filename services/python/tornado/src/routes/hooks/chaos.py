from asyncio import sleep
from random import randrange
from tornado.web import RequestHandler

from routes.errors import AppErrors, AppError


async def chaos_hook(_handler: RequestHandler) -> AppError | None:
    if randrange(0, 101) < 5:
        await sleep(randrange(500, 1501) / 1_000_000)
    if randrange(0, 101) < 5:
        return AppErrors.Internal
    return None
