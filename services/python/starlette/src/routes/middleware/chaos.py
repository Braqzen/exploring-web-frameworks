from asyncio import sleep
from random import randrange
from starlette.requests import Request
from starlette.responses import Response
from starlette.middleware.base import BaseHTTPMiddleware, RequestResponseEndpoint

from routes.errors import send_error, AppErrors


class ChaosMiddleware(BaseHTTPMiddleware):
    async def dispatch(
        self, request: Request, call_next: RequestResponseEndpoint
    ) -> Response:
        config = request.app.state.config

        if config.latency.enabled and randrange(0, 101) < config.latency.rate:
            await sleep(randrange(500, 1501) / 1_000_000)
        if config.error.enabled and randrange(0, 101) < config.error.rate:
            return send_error(AppErrors.Internal)

        return await call_next(request)
