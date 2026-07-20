from asyncio import sleep
from fastapi import Request, Response
from random import randrange
from collections.abc import Awaitable, Callable

from routes.errors import send_error, AppErrors


async def chaos_middleware(
    request: Request,
    call_next: Callable[[Request], Awaitable[Response]],
) -> Response:
    config = request.app.state.config

    if config.latency.enabled and randrange(0, 101) < config.latency.rate:
        await sleep(randrange(500, 1501) / 1_000_000)
    if config.error.enabled and randrange(0, 101) < config.error.rate:
        return send_error(AppErrors.Internal)

    return await call_next(request)
