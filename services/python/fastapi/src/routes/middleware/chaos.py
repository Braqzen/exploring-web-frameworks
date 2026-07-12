from asyncio import sleep
from fastapi import Request, Response
from random import randrange
from collections.abc import Awaitable, Callable

from routes.errors import send_error, AppErrors


async def chaos_middleware(
    request: Request,
    call_next: Callable[[Request], Awaitable[Response]],
) -> Response:
    if randrange(0, 101) < 5:
        await sleep(randrange(500, 1501) / 1_000_000)
    if randrange(0, 101) < 5:
        return send_error(AppErrors.Internal)

    return await call_next(request)
