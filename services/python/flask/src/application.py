from routes.router import register_routes, bp
from flask import Flask
from app.state import AppState


# TODO: make configurable?
MAX_BODY_SIZE = 64 * 1024


def create_app() -> Flask:
    app = Flask(__name__)

    app.config["MAX_CONTENT_LENGTH"] = MAX_BODY_SIZE
    app.extensions["state"] = AppState()

    register_routes()
    app.register_blueprint(bp)

    return app
