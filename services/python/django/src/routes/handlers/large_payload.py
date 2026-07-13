import structlog
from django.core.exceptions import RequestDataTooBig
from django.http import HttpRequest, JsonResponse

from routes.errors import AppErrors, send_error


def large_payload_handler(
    request: HttpRequest, exception: RequestDataTooBig
) -> JsonResponse:
    structlog.get_logger().warn(
        "Invalid body JSON", method=request.method, path=request.path
    )
    return send_error(AppErrors.InvalidJsonBody)
