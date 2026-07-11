from routes.middleware import log_middleware, chaos_middleware
from flask import Flask
from app.state import AppState
from routes.router import register_routes, bp
from routes.handlers import (
    invalid_path_handler,
    invalid_method_handler,
    large_payload_handler,
    internal_error_handler,
)


# TODO: make configurable?
MAX_BODY_SIZE = 64 * 1024


def create_app() -> Flask:
    app = Flask(__name__, static_folder=None)

    app.config["MAX_CONTENT_LENGTH"] = MAX_BODY_SIZE
    app.config["PROVIDE_AUTOMATIC_OPTIONS"] = False
    app.extensions["state"] = AppState()

    app.before_request(log_middleware)
    app.before_request(chaos_middleware)

    app.register_error_handler(404, invalid_path_handler)
    app.register_error_handler(405, invalid_method_handler)
    app.register_error_handler(413, large_payload_handler)
    app.register_error_handler(500, internal_error_handler)

    register_routes()
    app.register_blueprint(bp)

    return app
