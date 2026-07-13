import structlog
import asyncio
from hypercorn.asyncio import serve
from hypercorn.config import Config

from telemetry import Telemetry
from application import Application


class Server:
    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port

    def run(self, telemetry: Telemetry) -> None:
        logger = structlog.get_logger()
        application = Application()

        config = Config()
        config.bind = [f"{self.host}:{self.port}"]
        config.accesslog = None
        config.errorlog = None

        logger.info("Starting router", socket=f"{self.host}:{self.port}")

        try:
            asyncio.run(serve(application.app, config))
        finally:
            telemetry.shutdown()
