from flask import Blueprint
from routes.middleware import log_middleware, chaos_middleware
from routes.handlers import (
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
)

bp = Blueprint("flask", __name__)


def register_routes():
    bp.before_request(log_middleware)
    bp.before_request(chaos_middleware)

    bp.post("/")(post_handler)
    bp.get("/<id>")(get_handler)
    bp.delete("/<id>")(delete_handler)
    bp.put("/<id>")(put_handler)
    bp.patch("/<id>")(patch_handler)
