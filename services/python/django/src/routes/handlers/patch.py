import structlog
import json
from uuid import UUID
from pydantic import ValidationError
from django.http import HttpRequest, HttpResponse, JsonResponse
from django.apps import apps
from typing import cast

from app.params import parse_id, parse_patched_task
from app.task import PatchedTask, Task
from app.operation import Operation
from routes.errors import send_error, AppErrors
from routes.apps import RoutesConfig


async def patch_handler(request: HttpRequest, id: str) -> HttpResponse:
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
        patched_task: PatchedTask = parse_patched_task(body)
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

        previous_operation: Operation = task.operation

        config.state.tasks[task_id].operation = patched_task.operation

    logger.info(
        "Patched task",
        id=str(task_id),
        secret=len(task.secret),
        from_operation=previous_operation.lower(),
        to_operation=patched_task.operation.lower(),
        method=request.method,
    )

    return JsonResponse(task.model_dump(), status=200)
