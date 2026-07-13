import structlog
from quart import request


async def log_middleware():
    structlog.get_logger().debug(
        "Incoming request", method=request.method, path=request.path
    )
