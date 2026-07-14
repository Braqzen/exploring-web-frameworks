import structlog
import json
from uuid import uuid4, UUID
from pydantic import ValidationError
from starlette.requests import Request
from starlette.responses import Response, JSONResponse

from app.params import parse_task
from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


async def post_handler(request: Request) -> Response:
    logger = structlog.get_logger()

    try:
        body = await request.json()
        task: Task = parse_task(body)
    except (json.JSONDecodeError, ValidationError):
        logger.warn("Invalid body JSON", method=request.method, path=request.url.path)
        return send_error(AppErrors.InvalidJsonBody)

    id: UUID = uuid4()
    state: AppState = request.app.state.app_state

    async with request.app.state.tasks_lock:
        state.tasks[id] = task

    logger.info(
        "Inserted new task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return JSONResponse({"id": str(id)}, status_code=201)
