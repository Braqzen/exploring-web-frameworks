import structlog
from uuid import uuid4, UUID
from sanic import Request, json
from sanic.response import JSONResponse
from pydantic import ValidationError

from app.params import parse_task
from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


async def post_handler(request: Request) -> JSONResponse:
    logger = structlog.get_logger()

    body = request.json
    if body is None:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        task: Task = parse_task(body)
    except ValidationError:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    id: UUID = uuid4()
    state: AppState = request.app.ctx.state

    async with request.app.ctx.tasks_lock:
        state.tasks[id] = task

    logger.info(
        "Inserted new task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return json({"id": str(id)}, status=201)
