import structlog
from uuid import uuid4, UUID
from fastapi import Request
from typing import TypedDict

from app.task import Task
from app.state import AppState


class TaskId(TypedDict):
    id: str


async def post_handler(request: Request, task: Task) -> TaskId:
    logger = structlog.get_logger()

    id: UUID = uuid4()
    state: AppState = request.app.state.app_state

    async with request.app.state.tasks_lock:
        state.tasks[id] = task

    logger.info(
        "Inserted new task",
        id=str(id),
        secret=len(task.secret),
        operation=task.operation.lower(),
        method="POST",
    )

    return {"id": str(id)}
