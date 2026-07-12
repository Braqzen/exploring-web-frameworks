import structlog
from uuid import UUID
from fastapi import Request
from fastapi.responses import JSONResponse

from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


async def put_handler(
    request: Request, id: UUID, new_task: Task
) -> JSONResponse | Task:
    logger = structlog.get_logger()

    state: AppState = request.app.state.app_state

    async with request.app.state.tasks_lock:
        task: Task | None = state.tasks.get(id)

        if task is None:
            logger.warn(
                "Task not found", id=str(id), method="PUT", path=request.url.path
            )
            return send_error(AppErrors.TaskNotFound)

        state.tasks[id] = new_task

    logger.info(
        "Overwrote task",
        id=str(id),
        from_secret=len(task.secret),
        to_secret=len(new_task.secret),
        from_operation=task.operation.lower(),
        to_operation=new_task.operation.lower(),
        method="PUT",
    )

    return new_task
