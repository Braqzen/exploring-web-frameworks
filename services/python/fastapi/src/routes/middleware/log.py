import structlog
from fastapi import Request


async def log_middleware(request: Request, call_next):
    structlog.get_logger().debug(
        "Incoming request", method=request.method, path=request.url.path
    )

    return await call_next(request)
