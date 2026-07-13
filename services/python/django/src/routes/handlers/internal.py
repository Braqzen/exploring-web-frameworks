import structlog
import sys
from django.http import HttpRequest, HttpResponse

from routes.errors import AppErrors, send_error


def internal_error_handler(request: HttpRequest) -> HttpResponse:
    _, exc_value, _ = sys.exc_info()

    structlog.get_logger().error(
        "Internal server error",
        method=request.method,
        path=request.path,
        error=repr(exc_value) if exc_value is not None else None,
    )

    return send_error(AppErrors.Internal)
