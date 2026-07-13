import structlog
from flask import request


def log_middleware():
    structlog.get_logger().debug(
        "Incoming request", method=request.method, path=request.path
    )
