import structlog
from tornado.web import RequestHandler


async def log_hook(handler: RequestHandler) -> None:
    structlog.get_logger().debug(
        "Incoming request",
        method=handler.request.method,
        path=handler.request.path,
    )
