import asyncio
from tornado.web import Application as TornadoApp, RequestHandler

from app.config import Config
from app.state import AppState
from routes.handlers import (
    PostHandler,
    MultiMethodHandler,
    NotFoundHandler,
)

BYTES: int = 1024


class Application:
    def __init__(self) -> None:
        config = Config.new()
        self.MAX_BODY_SIZE = config.request_size_limit * BYTES
        self.app = TornadoApp(
            [
                (r"/", PostHandler),
                (r"/(?P<id>[^/]+)", MultiMethodHandler),
            ],
            state=AppState(),
            tasks_lock=asyncio.Lock(),
            config=config,
            default_handler_class=NotFoundHandler,
            log_function=_noop_log,
        )


def _noop_log(_handler: RequestHandler) -> None:
    pass
