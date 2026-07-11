import structlog
from flask import Flask
from os import environ
from application import create_app
from server import start_server

if __name__ == "__main__":
    structlog.configure(
        processors=[structlog.processors.JSONRenderer()],
        cache_logger_on_first_use=True,
    )

    socket = environ["SOCKET"]
    host, _, port = socket.partition(":")

    app: Flask = create_app()

    start_server(app, host, int(port))
