import structlog
from fastapi import Request

from routes.errors import AppErrors, send_error


async def internal_handler(request: Request, exc: Exception):
    structlog.get_logger().error(
        "Internal server error",
        method=request.method,
        path=request.url.path,
        error=repr(exc),
    )

    return send_error(AppErrors.Internal)
