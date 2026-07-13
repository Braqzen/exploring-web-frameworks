import structlog
from uuid import uuid4, UUID
from quart import current_app, jsonify, request
from pydantic import ValidationError

from app.task import Task
from app.params import parse_task
from app.state import AppState
from routes.errors import send_error, AppErrors


async def post_handler():
    logger = structlog.get_logger()

    body = await request.get_json(silent=True)

    if body is None:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        task: Task = parse_task(body)
    except ValidationError:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    id: UUID = uuid4()
    state: AppState = current_app.extensions["state"]

    state.tasks[id] = task

    logger.info(
        "Inserted new task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return jsonify({"id": str(id)}), 201
