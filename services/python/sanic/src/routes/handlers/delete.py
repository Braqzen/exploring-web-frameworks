import structlog
from uuid import UUID
from sanic import Request, empty
from sanic.response import JSONResponse, HTTPResponse

from app.state import AppState
from app.task import Task
from routes.errors import send_error, AppErrors


async def delete_handler(request: Request, id: UUID) -> HTTPResponse | JSONResponse:
    logger = structlog.get_logger()

    state: AppState = request.app.ctx.state

    async with request.app.ctx.tasks_lock:
        task: Task | None = state.tasks.pop(id, None)

        if task is None:
            logger.warn(
                "Task not found", id=str(id), method=request.method, path=request.path
            )
            return send_error(AppErrors.TaskNotFound)

    logger.info(
        "Removed task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return empty()
