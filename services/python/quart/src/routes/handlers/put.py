import structlog
from uuid import UUID
from quart import current_app, jsonify, request
from pydantic import ValidationError

from app.params import parse_id, parse_task
from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


async def put_handler(id: str):
    logger = structlog.get_logger()

    try:
        task_id: UUID = parse_id(id)
    except ValidationError:
        logger.warn("Invalid path", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidPath)

    body = await request.get_json(silent=True)

    if body is None:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        new_task: Task = parse_task(body)
    except ValidationError:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    state: AppState = current_app.extensions["state"]
    task: Task | None = state.tasks.get(task_id)

    if task is None:
        logger.warn(
            "Task not found", id=str(task_id), method=request.method, path=request.path
        )
        return send_error(AppErrors.TaskNotFound)

    state.tasks[task_id] = new_task

    logger.info(
        "Overwrote task",
        id=str(task_id),
        from_secret=len(task.secret),
        to_secret=len(new_task.secret),
        from_operation=task.operation.lower(),
        to_operation=new_task.operation.lower(),
        method=request.method,
    )

    return jsonify(new_task.model_dump()), 200
