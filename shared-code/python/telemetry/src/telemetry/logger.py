import logging
import structlog
from opentelemetry.sdk.resources import SERVICE_NAME, Resource
from opentelemetry.sdk._logs import LoggerProvider, LoggingHandler
from opentelemetry.sdk._logs.export import BatchLogRecordProcessor
from opentelemetry._logs import set_logger_provider
from opentelemetry.exporter.otlp.proto.http._log_exporter import OTLPLogExporter


class TelemetryLogger:
    def __init__(self, service_name: str, log_level: str) -> None:
        self.service_name = service_name
        self.log_level = log_level
        self.provider = None

    def start(self) -> None:
        if self.provider is not None:
            return None

        resource = Resource.create({SERVICE_NAME: self.service_name})

        self.provider = LoggerProvider(resource=resource)
        self.provider.add_log_record_processor(
            BatchLogRecordProcessor(OTLPLogExporter())
        )
        set_logger_provider(self.provider)

        otel_handler = LoggingHandler(
            level=logging.NOTSET, logger_provider=self.provider
        )
        otel_handler.setFormatter(logging.Formatter("%(message)s"))

        logging.basicConfig(
            handlers=[otel_handler],
            level=getattr(logging, self.log_level.upper()),
            force=True,
        )

        structlog.configure(
            processors=[
                structlog.stdlib.filter_by_level,
                structlog.stdlib.add_log_level,
                structlog.stdlib.render_to_log_kwargs,
            ],
            logger_factory=structlog.stdlib.LoggerFactory(),
            wrapper_class=structlog.stdlib.BoundLogger,
            cache_logger_on_first_use=True,
        )

    def shutdown(self) -> None:
        if self.provider is not None:
            self.provider.shutdown()
