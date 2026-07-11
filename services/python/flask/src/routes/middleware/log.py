from flask import request
import structlog


def log_middleware():
    logger = structlog.get_logger()

    logger.debug("Incoming request", method=request.method, path=request.path)
