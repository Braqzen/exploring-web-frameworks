import structlog
from uuid import UUID
from sanic import Request, json
from sanic.response import JSONResponse

from app.state import AppState
from app.task import Task
from routes.errors import send_error, AppErrors


async def get_handler(request: Request, id: UUID) -> JSONResponse:
    logger = structlog.get_logger()

    state: AppState = request.app.ctx.state

    async with request.app.ctx.tasks_lock:
        task: Task | None = state.tasks.get(id)

        if task is None:
            logger.warn(
                "Task not found", id=str(id), method=request.method, path=request.path
            )
            return send_error(AppErrors.TaskNotFound)

    logger.info(
        "Retrieved task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return json(task.model_dump())
