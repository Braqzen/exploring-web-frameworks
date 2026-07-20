from flask import Flask, Blueprint, request

from app.config import Config
from app.state import AppState
from routes.middleware import log_middleware, chaos_middleware
from routes.errors import send_error, AppErrors
from routes.handlers import (
    invalid_path_handler,
    invalid_method_handler,
    large_payload_handler,
    internal_error_handler,
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
)

BYTES: int = 1024


class Application:
    def __init__(self) -> None:
        self.app = Flask(__name__, static_folder=None)
        bp = Blueprint("flask", __name__)

        config = Config.new()
        self.app.config["MAX_CONTENT_LENGTH"] = config.request_size_limit * BYTES
        self.app.config["PROVIDE_AUTOMATIC_OPTIONS"] = False
        self.app.extensions["config"] = config
        self.app.extensions["state"] = AppState()

        self.app.before_request(log_middleware)
        self.app.before_request(chaos_middleware)

        self.app.register_error_handler(404, invalid_path_handler)
        self.app.register_error_handler(405, invalid_method_handler)
        self.app.register_error_handler(413, large_payload_handler)
        self.app.register_error_handler(500, internal_error_handler)

        bp.before_request(reject_head)

        bp.post("/")(post_handler)
        bp.get("/<id>")(get_handler)
        bp.delete("/<id>")(delete_handler)
        bp.put("/<id>")(put_handler)
        bp.patch("/<id>")(patch_handler)

        self.app.register_blueprint(bp)


def reject_head():
    if request.method == "HEAD":
        return send_error(AppErrors.InvalidMethod)
    return None
