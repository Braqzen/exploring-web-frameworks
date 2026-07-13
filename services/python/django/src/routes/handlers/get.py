import structlog
from uuid import UUID
from pydantic import ValidationError
from django.http import HttpRequest, HttpResponse, JsonResponse
from django.apps import apps
from typing import cast

from app.params import parse_id
from app.task import Task
from routes.errors import send_error, AppErrors
from routes.apps import RoutesConfig


async def get_handler(request: HttpRequest, id: str) -> HttpResponse:
    logger = structlog.get_logger()

    try:
        task_id: UUID = parse_id(id)
    except ValidationError:
        logger.warn("Invalid path", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidPath)

    config = cast(RoutesConfig, apps.get_app_config("routes"))
    async with config.tasks_lock:
        task: Task | None = config.state.tasks.get(task_id)

    if task is None:
        logger.warn(
            "Task not found", id=str(task_id), method=request.method, path=request.path
        )
        return send_error(AppErrors.TaskNotFound)

    logger.info(
        "Retrieved task",
        id=str(task_id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return JsonResponse(task.model_dump(), status=200)
