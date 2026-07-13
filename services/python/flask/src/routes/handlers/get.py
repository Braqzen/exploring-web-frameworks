import structlog
from uuid import UUID
from flask import current_app, jsonify, request
from pydantic import ValidationError

from app.params import parse_id
from app.state import AppState
from app.task import Task
from routes.errors import send_error, AppErrors


def get_handler(id: str):
    logger = structlog.get_logger()

    try:
        task_id: UUID = parse_id(id)
    except ValidationError:
        logger.warn("Invalid path", method="GET", path=request.path)
        return send_error(AppErrors.InvalidPath)

    state: AppState = current_app.extensions["state"]
    task: Task | None = state.tasks.get(task_id)

    if task is None:
        logger.warn("Task not found", id=str(task_id), method="GET", path=request.path)
        return send_error(AppErrors.TaskNotFound)

    logger.info(
        "Retrieved task",
        id=str(task_id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method="GET",
    )

    return jsonify(task.model_dump()), 200
