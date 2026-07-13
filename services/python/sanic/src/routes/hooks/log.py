import structlog
from sanic import Request


async def log_hook(request: Request) -> None:
    structlog.get_logger().debug(
        "Incoming request", method=request.method, path=request.path
    )
