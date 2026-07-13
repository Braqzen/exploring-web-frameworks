import structlog
from sanic import Request
from sanic.response import JSONResponse
from sanic.exceptions import NotFound

from routes.errors import AppErrors, send_error


async def invalid_path_handler(request: Request, _exception: NotFound) -> JSONResponse:
    structlog.get_logger().warn(
        "Invalid path", method=request.method, path=request.path
    )

    return send_error(AppErrors.InvalidPath)
