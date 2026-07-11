import structlog
from .logger import TelemetryLogger
from .profiler import TelemetryProfiler


class Telemetry:
    def __init__(self, service_name: str, log_level: str):
        self.logger = TelemetryLogger(service_name, log_level)
        self.profiler = TelemetryProfiler(service_name)

    def start(self) -> None:
        self.logger.start()
        self.profiler.start()

    def shutdown(self) -> None:
        try:
            self.profiler.shutdown()
        except Exception:
            structlog.get_logger().exception("Profiler shutdown failed")

        try:
            self.logger.shutdown()
        except Exception:
            structlog.get_logger().exception("Logger shutdown failed")
