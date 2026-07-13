import structlog
from collections.abc import Awaitable, Callable
from inspect import iscoroutinefunction
from asgiref.sync import markcoroutinefunction
from django.http import HttpRequest, HttpResponse


class LogMiddleware:
    async_capable = True
    sync_capable = False

    def __init__(
        self, get_response: Callable[[HttpRequest], Awaitable[HttpResponse]]
    ) -> None:
        self.get_response = get_response
        if iscoroutinefunction(self.get_response):
            markcoroutinefunction(self)

    async def __call__(self, request: HttpRequest) -> HttpResponse:
        structlog.get_logger().debug(
            "Incoming request", method=request.method, path=request.path
        )
        return await self.get_response(request)
