import structlog
from fastapi import Request, Response
from collections.abc import Awaitable, Callable

from routes.errors import send_error, AppErrors

BYTES: int = 1024


async def body_size_middleware(
    request: Request,
    call_next: Callable[[Request], Awaitable[Response]],
) -> Response:
    if request.method in {"POST", "PUT", "PATCH"}:
        if (
            len(await request.body())
            > request.app.state.config.request_size_limit * BYTES
        ):
            structlog.get_logger().warn(
                "Invalid body JSON", method=request.method, path=request.url.path
            )

            return send_error(AppErrors.InvalidJsonBody)

    return await call_next(request)
