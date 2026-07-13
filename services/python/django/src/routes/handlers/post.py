import structlog
import json
from uuid import uuid4, UUID
from pydantic import ValidationError
from django.http import HttpRequest, HttpResponse, JsonResponse
from django.apps import apps
from typing import cast

from app.task import Task
from app.params import parse_task
from routes.errors import send_error, AppErrors
from routes.apps import RoutesConfig


async def post_handler(request: HttpRequest) -> HttpResponse:
    logger = structlog.get_logger()

    try:
        body = json.loads(request.body)
    except (json.JSONDecodeError, UnicodeDecodeError):
        body = None

    if body is None:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    try:
        task: Task = parse_task(body)
    except ValidationError:
        logger.warn("Invalid body JSON", method=request.method, path=request.path)
        return send_error(AppErrors.InvalidJsonBody)

    id: UUID = uuid4()

    config = cast(RoutesConfig, apps.get_app_config("routes"))

    async with config.tasks_lock:
        config.state.tasks[id] = task

    logger.info(
        "Inserted new task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return JsonResponse({"id": str(id)}, status=201)
