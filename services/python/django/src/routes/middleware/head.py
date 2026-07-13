from collections.abc import Awaitable, Callable
from inspect import iscoroutinefunction
from asgiref.sync import markcoroutinefunction
from django.http import HttpRequest, HttpResponse

from routes.errors import send_error, AppErrors


class RejectHeadMiddleware:
    async_capable = True
    sync_capable = False

    def __init__(
        self, get_response: Callable[[HttpRequest], Awaitable[HttpResponse]]
    ) -> None:
        self.get_response = get_response
        if iscoroutinefunction(self.get_response):
            markcoroutinefunction(self)

    async def __call__(self, request: HttpRequest) -> HttpResponse:
        if request.method == "HEAD":
            return send_error(AppErrors.InvalidMethod)

        return await self.get_response(request)
