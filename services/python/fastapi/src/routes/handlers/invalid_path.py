import structlog
from fastapi import Request

from routes.errors import AppErrors, send_error


async def invalid_path_handler(request: Request, exc: Exception):
    structlog.get_logger().warn(
        "Invalid path", method=request.method, path=request.url.path
    )

    return send_error(AppErrors.InvalidPath)
