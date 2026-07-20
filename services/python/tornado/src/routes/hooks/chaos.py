from asyncio import sleep
from random import randrange
from tornado.web import RequestHandler

from routes.errors import AppErrors, AppError


async def chaos_hook(handler: RequestHandler) -> AppError | None:
    config = handler.settings["config"]

    if config.latency.enabled and randrange(0, 101) < config.latency.rate:
        await sleep(randrange(500, 1501) / 1_000_000)
    if config.error.enabled and randrange(0, 101) < config.error.rate:
        return AppErrors.Internal
    return None
