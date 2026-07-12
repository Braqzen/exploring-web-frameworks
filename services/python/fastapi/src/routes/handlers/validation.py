import structlog
from fastapi import Request
from fastapi.exceptions import RequestValidationError
from fastapi.responses import JSONResponse

from routes.errors import send_error, AppErrors


async def validation_handler(request: Request, exc: Exception) -> JSONResponse:
    if not isinstance(exc, RequestValidationError):
        raise exc

    logger = structlog.get_logger()

    for error in exc.errors():
        if error.get("loc") and error["loc"][0] == "path":
            logger.warn("Invalid path", method=request.method, path=request.url.path)
            return send_error(AppErrors.InvalidPath)

    logger.warn("Invalid body JSON", method=request.method, path=request.url.path)
    return send_error(AppErrors.InvalidJsonBody)
