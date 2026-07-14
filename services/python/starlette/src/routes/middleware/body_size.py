import structlog
from starlette.middleware.base import BaseHTTPMiddleware, RequestResponseEndpoint
from starlette.responses import Response
from starlette.requests import Request
from starlette.types import ASGIApp

from routes.errors import send_error, AppErrors


class BodySizeMiddleware(BaseHTTPMiddleware):
    def __init__(self, app: ASGIApp, max_size: int):
        super().__init__(app)
        self.max_size = max_size

    async def dispatch(
        self, request: Request, call_next: RequestResponseEndpoint
    ) -> Response:
        if request.method in {"POST", "PUT", "PATCH"}:
            if len(await request.body()) > self.max_size:
                structlog.get_logger().warn(
                    "Invalid body JSON", method=request.method, path=request.url.path
                )

                return send_error(AppErrors.InvalidJsonBody)

        return await call_next(request)
