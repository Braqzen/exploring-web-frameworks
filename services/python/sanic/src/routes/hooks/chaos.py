from asyncio import sleep
from random import randrange
from sanic import Request
from sanic.response import JSONResponse

from routes.errors import send_error, AppErrors


async def chaos_hook(
    request: Request,
) -> JSONResponse | None:
    config = request.app.ctx.config

    if config.latency.enabled and randrange(0, 101) < config.latency.rate:
        await sleep(randrange(500, 1501) / 1_000_000)
    if config.error.enabled and randrange(0, 101) < config.error.rate:
        return send_error(AppErrors.Internal)
