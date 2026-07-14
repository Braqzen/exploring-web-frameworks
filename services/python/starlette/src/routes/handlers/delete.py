import structlog
from uuid import UUID
from starlette.requests import Request
from starlette.responses import Response

from app.state import AppState
from app.task import Task
from routes.errors import send_error, AppErrors


async def delete_handler(request: Request) -> Response:
    logger = structlog.get_logger()

    id: UUID = request.path_params["id"]
    state: AppState = request.app.state.app_state

    async with request.app.state.tasks_lock:
        task: Task | None = state.tasks.pop(id, None)

        if task is None:
            logger.warn(
                "Task not found",
                id=str(id),
                method=request.method,
                path=request.url.path,
            )
            return send_error(AppErrors.TaskNotFound)

    logger.info(
        "Removed task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method=request.method,
    )

    return Response(status_code=204)
