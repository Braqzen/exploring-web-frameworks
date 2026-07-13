import structlog

from telemetry import Telemetry
from application import Application


class Server:
    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port

    def run(self, telemetry: Telemetry) -> None:
        logger = structlog.get_logger()
        application = Application()

        logger.info("Starting router", socket=f"{self.host}:{self.port}")

        try:
            application.app.run(
                host=self.host,
                port=self.port,
                access_log=False,
                motd=False,
                single_process=True,
            )
        finally:
            telemetry.shutdown()
