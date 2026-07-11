from flask import Flask
import structlog
from gunicorn.app.base import BaseApplication


def start_server(app: Flask, host: str, port: int) -> None:
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
