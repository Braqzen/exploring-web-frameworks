import structlog
from uuid import UUID
from sanic import Request, json
from sanic.response import JSONResponse
from pydantic import ValidationError

from app.params import parse_patched_task
from app.state import AppState
from app.task import PatchedTask, Task
from app.operation import Operation
from routes.errors import send_error, AppErrors


async def patch_handler(request: Request, id: UUID) -> JSONResponse:
    logger = structlog.get_logger()

    body = request.json
    if body is None:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        patched_task: PatchedTask = parse_patched_task(body)
    except ValidationError:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    state: AppState = request.app.ctx.state

    async with request.app.ctx.tasks_lock:
        task: Task | None = state.tasks.get(id)

        if task is None:
            logger.warn(
                "Task not found", id=str(id), method=request.method, path=request.path
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
        method=request.method,
    )

    return json(task.model_dump())
