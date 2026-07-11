import structlog
from flask import request
from werkzeug.exceptions import HTTPException
from routes.errors import AppErrors, send_error


def invalid_method_handler(_error: HTTPException):
    structlog.getLogger().warn(
        "Invalid method", method=request.method, path=request.path
    )
    return send_error(AppErrors.InvalidMethod)
