import structlog
import signal
import asyncio
import logging
from tornado.httpserver import HTTPServer
from tornado.ioloop import IOLoop

from telemetry import Telemetry
from application import Application


class Server:
    def __init__(self, host: str, port: int) -> None:
        self.host = host
        self.port = port

        self._server = None
        self._loop = None

    def run(self, telemetry: Telemetry) -> None:
        logger = structlog.get_logger()
        application = Application()
        self._loop = IOLoop.current()

        logger.info("Starting router", socket=f"{self.host}:{self.port}")

        logging.getLogger("tornado.access").disabled = True
        logging.getLogger("tornado.application").disabled = True
        logging.getLogger("tornado.general").disabled = True

        self._server = HTTPServer(
            application.app, max_body_size=application.MAX_BODY_SIZE
        )
        self._server.listen(self.port, address=self.host)

        signal.signal(signal.SIGINT, self._handle_signal)
        signal.signal(signal.SIGTERM, self._handle_signal)

        try:
            self._loop.start()
        finally:
            telemetry.shutdown()

    def _handle_signal(self, signum: int, frame: object) -> None:
        asyncio.ensure_future(self._shutdown())

    async def _shutdown(self) -> None:
        if self._server is not None:
            self._server.stop()
            await self._server.close_all_connections()
        if self._loop is not None:
            self._loop.stop()
