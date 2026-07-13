import structlog
from django.http import HttpRequest, JsonResponse

from routes.errors import AppErrors, send_error


def invalid_path_handler(request: HttpRequest, exception: Exception) -> JsonResponse:
    structlog.get_logger().warn(
        "Invalid path", method=request.method, path=request.path
    )
    return send_error(AppErrors.InvalidPath)
