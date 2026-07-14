from dataclasses import dataclass
from tornado.web import RequestHandler


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


def send_error(handler: RequestHandler, error: AppError) -> None:
    handler.set_status(error.status)
    handler.write({"error": error.message})
