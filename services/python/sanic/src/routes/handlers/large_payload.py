import structlog
from sanic import Request
from sanic.response import JSONResponse
from sanic.exceptions import PayloadTooLarge

from routes.errors import AppErrors, send_error


async def large_payload_handler(
    request: Request, _exception: PayloadTooLarge
) -> JSONResponse:
    structlog.get_logger().warn(
        "Invalid body JSON", method=request.method, path=request.path
    )

    return send_error(AppErrors.InvalidJsonBody)
