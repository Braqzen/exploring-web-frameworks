import structlog
from fastapi import Request, Response
from fastapi.responses import JSONResponse
from collections.abc import Awaitable, Callable

# TODO: make configurable?
MAX_BODY_SIZE: int = 64 * 1024


async def body_size_middleware(
    request: Request,
    call_next: Callable[[Request], Awaitable[Response]],
) -> Response:
    if request.method in {"POST", "PUT", "PATCH"}:
        if len(await request.body()) > MAX_BODY_SIZE:
            structlog.get_logger().warn(
                "Invalid body JSON", method=request.method, path=request.url.path
            )

            return JSONResponse({"error": "Invalid body JSON"}, status_code=422)

    return await call_next(request)
