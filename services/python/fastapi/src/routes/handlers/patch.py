import structlog
from uuid import UUID
from fastapi import Request
from fastapi.responses import JSONResponse

from app.state import AppState
from app.task import PatchedTask, Task
from app.operation import Operation
from routes.errors import send_error, AppErrors


async def patch_handler(
    request: Request, id: UUID, patched_task: PatchedTask
) -> JSONResponse | Task:
    logger = structlog.get_logger()

    state: AppState = request.app.state.app_state

    async with request.app.state.tasks_lock:
        task: Task | None = state.tasks.get(id)

        if task is None:
            logger.warn(
                "Task not found", id=str(id), method="PATCH", path=request.url.path
            )
            return send_error(AppErrors.TaskNotFound)

        previous_operation: Operation = task.operation

        state.tasks[id].operation = patched_task.operation

    logger.info(
        "Patched task",
        id=str(id),
        secret=len(task.secret),
        from_operation=previous_operation.lower(),
        to_operation=patched_task.operation.lower(),
        method="PATCH",
    )

    return task
