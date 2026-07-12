import structlog
from fastapi import Request
from fastapi.responses import JSONResponse

from routes.errors import AppErrors, send_error


async def invalid_method_handler(request: Request, exc: Exception) -> JSONResponse:
    structlog.get_logger().warn(
        "Invalid method", method=request.method, path=request.url.path
    )

    return send_error(AppErrors.InvalidMethod)
