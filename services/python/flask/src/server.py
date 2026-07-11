import structlog
from functools import partial
from flask import Flask
from gunicorn.app.base import BaseApplication
from opentelemetry.sdk._logs import LoggerProvider


def start_server(app: Flask, host: str, port: int, provider: LoggerProvider) -> None:
    logger = structlog.get_logger()
    logger.info("Starting router", socket=f"{host}:{port}")

    server = GunicornApp(
        app,
        {
            "bind": f"{host}:{port}",
            "workers": 1,
            "worker_class": "gevent",
            "worker_connections": 1000,
            "accesslog": None,
            "errorlog": "/dev/null",
            "worker_exit": partial(worker_exit, provider),
            "on_exit": partial(on_exit, provider),
        },
    )

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


def worker_exit(provider: LoggerProvider, server, worker) -> None:
    try:
        provider.shutdown()
    except Exception:
        structlog.get_logger().exception("Failed to shut down telemetry provider")


def on_exit(provider: LoggerProvider, server) -> None:
    try:
        provider.shutdown()
    except Exception:
        structlog.get_logger().exception("Failed to shut down telemetry provider")
