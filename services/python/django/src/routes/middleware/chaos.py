from collections.abc import Awaitable, Callable
from random import randrange
from asyncio import sleep

from inspect import iscoroutinefunction
from asgiref.sync import markcoroutinefunction
from django.conf import settings
from django.http import HttpRequest, HttpResponse

from routes.errors import send_error, AppErrors


class ChaosMiddleware:
    async_capable = True
    sync_capable = False

    def __init__(
        self, get_response: Callable[[HttpRequest], Awaitable[HttpResponse]]
    ) -> None:
        self.get_response = get_response
        if iscoroutinefunction(self.get_response):
            markcoroutinefunction(self)

    async def __call__(self, request: HttpRequest) -> HttpResponse:
        config = settings.APP_CONFIG

        if config.latency.enabled and randrange(0, 101) < config.latency.rate:
            await sleep(randrange(500, 1501) / 1_000_000)
        if config.error.enabled and randrange(0, 101) < config.error.rate:
            return send_error(AppErrors.Internal)
        return await self.get_response(request)
