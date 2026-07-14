import structlog
from tornado.web import RequestHandler, Finish

from routes.errors import AppErrors, send_error, AppError
from routes.hooks import log_hook, chaos_hook


class BaseHandler(RequestHandler):
    async def prepare(self) -> None:
        await log_hook(self)
        error: AppError | None = await chaos_hook(self)
        if error is not None:
            send_error(self, error)
            raise Finish()

    def write_error(self, status_code: int, **kwargs) -> None:
        if status_code == 404:
            structlog.get_logger().warn(
                "Invalid path",
                method=self.request.method,
                path=self.request.path,
            )
            send_error(self, AppErrors.InvalidPath)
            return

        elif status_code == 405:
            structlog.get_logger().warn(
                "Invalid method",
                method=self.request.method,
                path=self.request.path,
            )
            send_error(self, AppErrors.InvalidMethod)
            return

        elif status_code == 500:
            exc = kwargs.get("exc_info")
            structlog.get_logger().error(
                "Internal server error",
                method=self.request.method,
                path=self.request.path,
                error=repr(exc[1]) if exc else None,
            )
            send_error(self, AppErrors.Internal)
            return

        send_error(self, AppError(status_code, "error"))
