import structlog
import uvicorn
from fastapi import FastAPI

from telemetry import Telemetry
from application import create_app


class Server:
    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port

    def run(self, telemetry: Telemetry) -> None:
        logger = structlog.get_logger()
        app: FastAPI = create_app()

        config = uvicorn.Config(
            app,
            host=self.host,
            port=self.port,
            log_config=None,
            access_log=False,
        )
        server = uvicorn.Server(config)

        logger.info("Starting router", socket=f"{self.host}:{self.port}")

        try:
            server.run()
        finally:
            telemetry.shutdown()
