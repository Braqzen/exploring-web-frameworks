import asyncio
from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.exceptions import RequestValidationError

from app.state import AppState
from routes.middleware import log_middleware, chaos_middleware, body_size_middleware
from routes.handlers import (
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
    internal_handler,
    invalid_method_handler,
    invalid_path_handler,
    validation_handler,
)


class Application:
    def __init__(self) -> None:
        self.app = FastAPI(lifespan=lifespan)

        self.app.middleware("http")(body_size_middleware)
        self.app.middleware("http")(chaos_middleware)
        self.app.middleware("http")(log_middleware)

        self.app.add_exception_handler(RequestValidationError, validation_handler)
        self.app.add_exception_handler(404, invalid_path_handler)
        self.app.add_exception_handler(405, invalid_method_handler)
        self.app.add_exception_handler(Exception, internal_handler)

        # TODO: response_model=None, unidiomatic bcs we return JSONResponse and 2nd type
        self.app.post("/", status_code=201)(post_handler)
        self.app.get("/{id}", status_code=200, response_model=None)(get_handler)
        self.app.delete("/{id}", status_code=204, response_model=None)(delete_handler)
        self.app.put("/{id}", status_code=200, response_model=None)(put_handler)
        self.app.patch("/{id}", status_code=200, response_model=None)(patch_handler)


@asynccontextmanager
async def lifespan(app: FastAPI):
    app.state.app_state = AppState()
    app.state.tasks_lock = asyncio.Lock()
    yield
