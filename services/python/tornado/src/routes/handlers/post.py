import structlog
import json
from uuid import uuid4, UUID
from pydantic import ValidationError
from tornado.escape import json_decode

from app.params import parse_task
from app.task import Task
from app.state import AppState
from routes.errors import AppErrors, send_error
from routes.handlers.base import BaseHandler


class PostHandler(BaseHandler):
    async def post(self):
        logger = structlog.get_logger()

        body = self.request.body
        if not body:
            logger.warn(
                "Invalid body JSON", method=self.request.method, path=self.request.path
            )
            send_error(self, AppErrors.InvalidJsonBody)
            return

        try:
            body = json_decode(body)
            task: Task = parse_task(body)
        except (json.JSONDecodeError, ValidationError):
            logger.warn(
                "Invalid body JSON", method=self.request.method, path=self.request.path
            )
            send_error(self, AppErrors.InvalidJsonBody)
            return

        id: UUID = uuid4()
        state: AppState = self.settings["state"]

        async with self.settings["tasks_lock"]:
            state.tasks[id] = task

        logger.info(
            "Inserted new task",
            id=str(id),
            secret=len(task.secret),
            operation=task.operation.lower(),
            method=self.request.method,
        )

        self.set_status(201)
        self.write({"id": str(id)})
