import structlog
from flask import request
from werkzeug.exceptions import HTTPException

from routes.errors import AppErrors, send_error


def invalid_path_handler(_error: HTTPException):
    structlog.get_logger().warn(
        "Invalid path", method=request.method, path=request.path
    )
    return send_error(AppErrors.InvalidPath)
