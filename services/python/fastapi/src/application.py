import asyncio
from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.exceptions import RequestValidationError

from app.state import AppState
from routes.middleware import log_middleware, chaos_middleware, body_size_middleware
from routes.router import register_routes
from routes.handlers import (
    internal_handler,
    invalid_method_handler,
    invalid_path_handler,
    validation_handler,
)


def create_app() -> FastAPI:
    app = FastAPI(lifespan=lifespan)

    app.middleware("http")(body_size_middleware)
    app.middleware("http")(log_middleware)
    app.middleware("http")(chaos_middleware)

    app.add_exception_handler(RequestValidationError, validation_handler)
    app.add_exception_handler(404, invalid_path_handler)
    app.add_exception_handler(405, invalid_method_handler)
    app.add_exception_handler(Exception, internal_handler)

    register_routes(app)

    return app


@asynccontextmanager
async def lifespan(app: FastAPI):
    app.state.app_state = AppState()
    app.state.tasks_lock = asyncio.Lock()
    yield
