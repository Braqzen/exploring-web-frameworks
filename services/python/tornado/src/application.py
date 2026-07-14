import asyncio
from tornado.web import Application as TornadoApp, RequestHandler

from app.state import AppState
from routes.handlers import (
    PostHandler,
    MultiMethodHandler,
    NotFoundHandler,
)

# TODO: make configurable?
MAX_BODY_SIZE: int = 64 * 1024


class Application:
    def __init__(self) -> None:
        self.MAX_BODY_SIZE = MAX_BODY_SIZE
        self.app = TornadoApp(
            [
                (r"/", PostHandler),
                (r"/(?P<id>[^/]+)", MultiMethodHandler),
            ],
            state=AppState(),
            tasks_lock=asyncio.Lock(),
            default_handler_class=NotFoundHandler,
            log_function=_noop_log,
        )


def _noop_log(_handler: RequestHandler) -> None:
    pass
