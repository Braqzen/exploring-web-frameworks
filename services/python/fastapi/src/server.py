import structlog
import uvicorn

from telemetry import Telemetry
from application import Application


class Server:
    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port

    def run(self, telemetry: Telemetry) -> None:
        logger = structlog.get_logger()
        application = Application()

        config = uvicorn.Config(
            application.app,
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
