import asyncio
from sanic import Sanic
from sanic.exceptions import NotFound, MethodNotAllowed, PayloadTooLarge

from app.state import AppState
from routes.hooks import log_hook, chaos_hook
from routes.handlers import (
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
    internal_handler,
    invalid_method_handler,
    invalid_path_handler,
    large_payload_handler,
)

# TODO: make configurable?
MAX_BODY_SIZE: int = 64 * 1024


class Application:
    def __init__(self) -> None:
        self.app = Sanic("sanic", configure_logging=False)
        self.app.ctx.state = AppState()
        self.app.ctx.tasks_lock = asyncio.Lock()
        self.app.config.REQUEST_MAX_SIZE = MAX_BODY_SIZE

        self.app.on_request(chaos_hook)
        self.app.on_request(log_hook)

        self.app.error_handler.add(NotFound, invalid_path_handler)
        self.app.error_handler.add(MethodNotAllowed, invalid_method_handler)
        self.app.error_handler.add(Exception, internal_handler)
        self.app.exception(PayloadTooLarge)(large_payload_handler)

        self.app.post("/")(post_handler)
        self.app.get("/<id:uuid>")(get_handler)
        self.app.delete("/<id:uuid>")(delete_handler)
        self.app.put("/<id:uuid>")(put_handler)
        self.app.patch("/<id:uuid>")(patch_handler)
