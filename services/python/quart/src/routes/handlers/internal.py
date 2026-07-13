import structlog
from quart import request
from werkzeug.exceptions import InternalServerError

from routes.errors import AppErrors, send_error


async def internal_error_handler(error: InternalServerError):
    cause = error.original_exception or error

    structlog.get_logger().error(
        "Internal server error",
        method=request.method,
        path=request.path,
        error=repr(cause),
    )

    return send_error(AppErrors.Internal)
