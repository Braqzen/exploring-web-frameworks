import structlog
from fastapi import Request, Response
from collections.abc import Awaitable, Callable


async def log_middleware(
    request: Request,
    call_next: Callable[[Request], Awaitable[Response]],
) -> Response:
    structlog.get_logger().debug(
        "Incoming request", method=request.method, path=request.url.path
    )

    return await call_next(request)
