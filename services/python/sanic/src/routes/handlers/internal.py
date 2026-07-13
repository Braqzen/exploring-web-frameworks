import structlog
from sanic import Request
from sanic.response import JSONResponse

from routes.errors import AppErrors, send_error


async def internal_handler(request: Request, exception: Exception) -> JSONResponse:
    structlog.get_logger().error(
        "Internal server error",
        method=request.method,
        path=request.path,
        error=repr(exception),
    )

    return send_error(AppErrors.Internal)
