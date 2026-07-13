import structlog
from uuid import UUID
from quart import current_app, request
from pydantic import ValidationError

from app.state import AppState
from app.params import parse_id
from app.task import Task
from routes.errors import send_error, AppErrors


async def delete_handler(id: str):
    logger = structlog.get_logger()

    try:
        task_id: UUID = parse_id(id)
    except ValidationError:
        logger.warn("Invalid path", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidPath)

    state: AppState = current_app.extensions["state"]
    task: Task | None = state.tasks.pop(task_id, None)

    if task is None:
        logger.warn(
            "Task not found", id=str(task_id), method=request.method, path=request.path
        )
        return send_error(AppErrors.TaskNotFound)

    logger.info(
        "Removed task",
        id=str(task_id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return "", 204
