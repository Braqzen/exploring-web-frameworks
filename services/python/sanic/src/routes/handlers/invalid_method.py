import structlog
from sanic import Request
from sanic.response import JSONResponse
from sanic.exceptions import MethodNotAllowed

from routes.errors import AppErrors, send_error


async def invalid_method_handler(
    request: Request, _exception: MethodNotAllowed
) -> JSONResponse:
    structlog.get_logger().warn(
        "Invalid method", method=request.method, path=request.path
    )

    return send_error(AppErrors.InvalidMethod)
