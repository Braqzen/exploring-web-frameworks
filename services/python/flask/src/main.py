from gevent import monkey

monkey.patch_all()

from flask import Flask
from os import environ
from application import create_app
from server import start_server
from telemetry.telemetry import init_telemetry

if __name__ == "__main__":
    log_level = environ.get("LOG_LEVEL", "info")

    provider = init_telemetry("flask", log_level)

    socket = environ["SOCKET"]
    host, _, port = socket.partition(":")

    app: Flask = create_app()

    start_server(app, host, int(port), provider)
