from dataclasses import dataclass
from fastapi.responses import JSONResponse


@dataclass(frozen=True)
class AppError:
    status: int
    message: str


class AppErrors:
    TaskNotFound = AppError(404, "Task not found")
    InvalidPath = AppError(404, "Invalid path")
    InvalidMethod = AppError(405, "Invalid method")
    InvalidJsonBody = AppError(422, "Invalid body JSON")
    Internal = AppError(500, "Internal server error")


def send_error(error: AppError) -> JSONResponse:
    return JSONResponse({"error": error.message}, status_code=error.status)
