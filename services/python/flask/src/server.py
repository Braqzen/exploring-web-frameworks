import structlog
from functools import partial
from flask import Flask
from gunicorn.app.base import BaseApplication
from telemetry import Telemetry
from application import create_app


class Server:
    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port

    def run(self, telemetry: Telemetry) -> None:
        logger = structlog.get_logger()
        app: Flask = create_app()

        server = GunicornApp(
            app,
            {
                "bind": f"{self.host}:{self.port}",
                "workers": 1,
                "worker_class": "gevent",
                "worker_connections": 1000,
                "accesslog": None,
                "errorlog": "/dev/null",
                "worker_exit": partial(worker_exit, telemetry),
                "on_exit": partial(on_exit, telemetry),
            },
        )

        logger.info("Starting router", socket=f"{self.host}:{self.port}")

        server.run()


class GunicornApp(BaseApplication):
    def __init__(self, application: Flask, options: dict) -> None:
        self.application = application
        self.options = options
        super().__init__()

    def load_config(self) -> None:
        for key, value in self.options.items():
            if key in self.cfg.settings and value is not None:
                self.cfg.set(key.lower(), value)

    def load(self):
        return self.application


def worker_exit(telemetry: Telemetry, server, worker) -> None:
    telemetry.shutdown()


def on_exit(telemetry: Telemetry, server) -> None:
    telemetry.shutdown()
