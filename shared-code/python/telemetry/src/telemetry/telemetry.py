import logging
import structlog
from opentelemetry.sdk.resources import SERVICE_NAME, Resource
from opentelemetry.sdk._logs import LoggerProvider, LoggingHandler
from opentelemetry.sdk._logs.export import BatchLogRecordProcessor
from opentelemetry._logs import set_logger_provider
from opentelemetry.exporter.otlp.proto.http._log_exporter import OTLPLogExporter


def init_telemetry(service_name: str, log_level: str) -> LoggerProvider:
    resource = Resource.create({SERVICE_NAME: service_name})

    provider = LoggerProvider(resource=resource)
    provider.add_log_record_processor(BatchLogRecordProcessor(OTLPLogExporter()))
    set_logger_provider(provider)

    otel_handler = LoggingHandler(level=logging.NOTSET, logger_provider=provider)
    otel_handler.setFormatter(logging.Formatter("%(message)s"))

    logging.basicConfig(
        handlers=[otel_handler],
        level=getattr(logging, log_level.upper()),
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

    return provider
