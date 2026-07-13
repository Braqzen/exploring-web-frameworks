import structlog
from uuid import UUID
from flask import current_app, jsonify, request
from pydantic import ValidationError

from app.state import AppState
from app.params import parse_id, parse_patched_task
from app.task import PatchedTask, Task
from app.operation import Operation
from routes.errors import send_error, AppErrors


def patch_handler(id: str):
    logger = structlog.get_logger()

    try:
        task_id: UUID = parse_id(id)
    except ValidationError:
        logger.warn("Invalid path", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidPath)

    body = request.get_json(silent=True)

    if body is None:
        logger.warn("Invalid body JSON", method="PATCH", path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        patched_task: PatchedTask = parse_patched_task(body)
    except ValidationError:
        logger.warn("Invalid body JSON", method="PATCH", path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    state: AppState = current_app.extensions["state"]
    task: Task | None = state.tasks.get(task_id)

    if task is None:
        logger.warn(
            "Task not found", id=str(task_id), method="PATCH", path=request.path
        )
        return send_error(AppErrors.TaskNotFound)

    previous_operation: Operation = task.operation

    state.tasks[task_id].operation = patched_task.operation

    logger.info(
        "Patched task",
        id=str(task_id),
        secret=len(task.secret),
        from_operation=previous_operation.lower(),
        to_operation=patched_task.operation.lower(),
        method="PATCH",
    )

    return jsonify(state.tasks[task_id].model_dump()), 200
