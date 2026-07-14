import structlog
import json
from uuid import UUID
from pydantic import ValidationError
from starlette.requests import Request
from starlette.responses import Response, JSONResponse

from app.params import parse_task
from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


async def put_handler(request: Request) -> Response:
    logger = structlog.get_logger()

    id: UUID = request.path_params["id"]

    try:
        body = await request.json()
        new_task: Task = parse_task(body)
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

        state.tasks[id] = new_task

    logger.info(
        "Overwrote task",
        id=str(id),
        from_secret=len(task.secret),
        to_secret=len(new_task.secret),
        from_operation=task.operation.lower(),
        to_operation=new_task.operation.lower(),
        method=request.method,
    )

    return JSONResponse(new_task.model_dump())
