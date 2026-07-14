import structlog
import json
from uuid import UUID
from pydantic import ValidationError
from starlette.requests import Request
from starlette.responses import Response, JSONResponse

from app.params import parse_patched_task
from app.state import AppState
from app.task import PatchedTask, Task
from app.operation import Operation
from routes.errors import send_error, AppErrors


async def patch_handler(request: Request) -> Response:
    logger = structlog.get_logger()

    id: UUID = request.path_params["id"]

    try:
        body = await request.json()
        patched_task: PatchedTask = parse_patched_task(body)
    except (json.JSONDecodeError, ValidationError):
        logger.warn("Invalid body JSON", method=request.method, path=request.url.path)
        return send_error(AppErrors.InvalidJsonBody)

    state: AppState = request.app.state.app_state

    async with request.app.state.tasks_lock:
        task: Task | None = state.tasks.get(id)

        if task is None:
            logger.warn(
                "Task not found",
                id=str(id),
                method=request.method,
                path=request.url.path,
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

    return JSONResponse(task.model_dump())
