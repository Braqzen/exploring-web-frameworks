from flask import Blueprint, request
from routes.errors import send_error, AppErrors
from routes.handlers import (
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
)

bp = Blueprint("flask", __name__)


def register_routes():
    bp.before_request(reject_head)

    bp.post("/")(post_handler)
    bp.get("/<id>")(get_handler)
    bp.delete("/<id>")(delete_handler)
    bp.put("/<id>")(put_handler)
    bp.patch("/<id>")(patch_handler)


def reject_head():
    if request.method == "HEAD":
        return send_error(AppErrors.InvalidMethod)
    return None
