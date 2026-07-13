import structlog
import json
from uuid import UUID
from pydantic import ValidationError
from django.http import HttpRequest, HttpResponse, JsonResponse
from django.apps import apps
from typing import cast

from app.params import parse_id, parse_task
from app.task import Task
from routes.errors import send_error, AppErrors
from routes.apps import RoutesConfig


async def put_handler(request: HttpRequest, id: str) -> HttpResponse:
    logger = structlog.get_logger()

    try:
        task_id: UUID = parse_id(id)
    except ValidationError:
        logger.warn("Invalid path", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidPath)

    try:
        body = json.loads(request.body)
    except (json.JSONDecodeError, UnicodeDecodeError):
        body = None

    if body is None:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        new_task: Task = parse_task(body)
    except ValidationError:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    config = cast(RoutesConfig, apps.get_app_config("routes"))

    async with config.tasks_lock:
        task: Task | None = config.state.tasks.get(task_id)

        if task is None:
            logger.warn(
                "Task not found",
                id=str(task_id),
                method=request.method,
                path=request.path,
            )
            return send_error(AppErrors.TaskNotFound)

        config.state.tasks[task_id] = new_task

    logger.info(
        "Overwrote task",
        id=str(task_id),
        from_secret=len(task.secret),
        to_secret=len(new_task.secret),
        from_operation=task.operation.lower(),
        to_operation=new_task.operation.lower(),
        method=request.method,
    )

    return JsonResponse(new_task.model_dump(), status=200)
