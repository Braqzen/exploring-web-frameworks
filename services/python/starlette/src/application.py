import asyncio
from starlette.applications import Starlette
from starlette.middleware import Middleware
from starlette.routing import Route

from app.state import AppState
from routes.middleware import LogMiddleware, ChaosMiddleware, BodySizeMiddleware
from routes.handlers import (
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
    internal_handler,
    invalid_method_handler,
    invalid_path_handler,
)

# TODO: make configurable?
MAX_BODY_SIZE: int = 64 * 1024


class Application:
    def __init__(self) -> None:
        self.app = Starlette(
            routes=[
                Route("/", endpoint=post_handler, methods=["POST"]),
                Route("/{id:uuid}", endpoint=get_handler, methods=["GET"]),
                Route("/{id:uuid}", endpoint=delete_handler, methods=["DELETE"]),
                Route("/{id:uuid}", endpoint=put_handler, methods=["PUT"]),
                Route("/{id:uuid}", endpoint=patch_handler, methods=["PATCH"]),
            ],
            middleware=[
                Middleware(LogMiddleware),
                Middleware(ChaosMiddleware),
                Middleware(BodySizeMiddleware, max_size=MAX_BODY_SIZE),
            ],
            exception_handlers={
                404: invalid_path_handler,
                405: invalid_method_handler,
                Exception: internal_handler,
            },
        )

        self.app.state.app_state = AppState()
        self.app.state.tasks_lock = asyncio.Lock()
