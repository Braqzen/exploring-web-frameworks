import structlog
from starlette.requests import Request
from starlette.responses import Response

from routes.errors import AppErrors, send_error


async def invalid_path_handler(request: Request, _exception: Exception) -> Response:
    structlog.get_logger().warn(
        "Invalid path", method=request.method, path=request.url.path
    )

    return send_error(AppErrors.InvalidPath)
