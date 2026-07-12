import structlog
from fastapi import Request
from fastapi.responses import JSONResponse

# TODO: make configurable?
MAX_BODY_SIZE = 64 * 1024


async def body_size_middleware(request: Request, call_next):
    if request.method in {"POST", "PUT", "PATCH"}:
        if len(await request.body()) > MAX_BODY_SIZE:
            structlog.get_logger().warn(
                "Invalid body JSON", method=request.method, path=request.url.path
            )

            return JSONResponse({"error": "Invalid body JSON"}, status_code=422)

    return await call_next(request)
