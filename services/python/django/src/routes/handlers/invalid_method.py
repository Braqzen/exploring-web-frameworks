import structlog
from django.http import HttpRequest, HttpResponse

from routes.errors import AppErrors, send_error


def invalid_method_handler(request: HttpRequest, _error=None) -> HttpResponse:
    structlog.get_logger().warn(
        "Invalid method", method=request.method, path=request.path
    )
    return send_error(AppErrors.InvalidMethod)
