import structlog
from uuid import UUID
from sanic import Request, json
from sanic.response import JSONResponse
from pydantic import ValidationError

from app.params import parse_task
from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


async def put_handler(request: Request, id: UUID) -> JSONResponse:
    logger = structlog.get_logger()

    body = request.json
    if body is None:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        new_task: Task = parse_task(body)
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

    return json(new_task.model_dump())
