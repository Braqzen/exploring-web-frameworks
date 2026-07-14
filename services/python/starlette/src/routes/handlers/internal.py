import structlog
from starlette.requests import Request
from starlette.responses import Response

from routes.errors import AppErrors, send_error


async def internal_handler(request: Request, exception: Exception) -> Response:
    structlog.get_logger().error(
        "Internal server error",
        method=request.method,
        path=request.url.path,
        error=repr(exception),
    )

    return send_error(AppErrors.Internal)
