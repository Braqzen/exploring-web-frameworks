import structlog
from quart import request
from werkzeug.exceptions import RequestEntityTooLarge

from routes.errors import AppErrors, send_error


async def large_payload_handler(_error: RequestEntityTooLarge):
    structlog.get_logger().warn(
        "Invalid body JSON", method=request.method, path=request.path
    )
    return send_error(AppErrors.InvalidJsonBody)
