import structlog
import json
from uuid import UUID
from pydantic import ValidationError
from tornado.escape import json_decode

from app.params import parse_id, parse_patched_task, parse_task
from app.state import AppState
from app.task import Task, PatchedTask
from app.operation import Operation
from routes.errors import AppErrors, send_error
from routes.handlers.base import BaseHandler


class MultiMethodHandler(BaseHandler):
    async def delete(self, id: str):
        logger = structlog.get_logger()

        try:
            task_id: UUID = parse_id(id)
        except ValidationError:
            send_error(self, AppErrors.InvalidPath)
            return

        state: AppState = self.settings["state"]

        async with self.settings["tasks_lock"]:
            task: Task | None = state.tasks.pop(task_id, None)

            if task is None:
                logger.warn(
                    "Task not found",
                    id=str(task_id),
                    method=self.request.method,
                    path=self.request.path,
                )
                send_error(self, AppErrors.TaskNotFound)
                return

        logger.info(
            "Removed task",
            id=str(task_id),
            secret=len(task.secret),
            operation=task.operation.lower(),
            method=self.request.method,
        )

        self.set_status(204)

    async def get(self, id: str):
        logger = structlog.get_logger()

        try:
            task_id: UUID = parse_id(id)
        except ValidationError:
            send_error(self, AppErrors.InvalidPath)
            return

        state: AppState = self.settings["state"]

        async with self.settings["tasks_lock"]:
            task: Task | None = state.tasks.get(task_id)

            if task is None:
                logger.warn(
                    "Task not found",
                    id=str(task_id),
                    method=self.request.method,
                    path=self.request.path,
                )
                send_error(self, AppErrors.TaskNotFound)
                return

        logger.info(
            "Retrieved task",
            id=str(task_id),
            secret=len(task.secret),
            operation=task.operation.lower(),
            method=self.request.method,
        )

        self.set_status(200)
        self.write(task.model_dump())

    async def patch(self, id: str):
        logger = structlog.get_logger()

        try:
            task_id: UUID = parse_id(id)
        except ValidationError:
            send_error(self, AppErrors.InvalidPath)
            return

        body = self.request.body
        if not body:
            logger.warn(
                "Invalid body JSON", method=self.request.method, path=self.request.path
            )
            send_error(self, AppErrors.InvalidJsonBody)
            return

        try:
            body = json_decode(body)
            patched_task: PatchedTask = parse_patched_task(body)
        except (json.JSONDecodeError, ValidationError):
            logger.warn(
                "Invalid body JSON", method=self.request.method, path=self.request.path
            )
            send_error(self, AppErrors.InvalidJsonBody)
            return

        state: AppState = self.settings["state"]

        async with self.settings["tasks_lock"]:
            task: Task | None = state.tasks.get(task_id)

            if task is None:
                logger.warn(
                    "Task not found",
                    id=str(task_id),
                    method=self.request.method,
                    path=self.request.path,
                )
                send_error(self, AppErrors.TaskNotFound)
                return

            previous_operation: Operation = task.operation

            state.tasks[task_id].operation = patched_task.operation

        logger.info(
            "Patched task",
            id=str(task_id),
            secret=len(task.secret),
            from_operation=previous_operation.lower(),
            to_operation=patched_task.operation.lower(),
            method=self.request.method,
        )

        self.set_status(200)
        self.write(task.model_dump())

    async def put(self, id: str):
        logger = structlog.get_logger()

        try:
            task_id: UUID = parse_id(id)
        except ValidationError:
            send_error(self, AppErrors.InvalidPath)
            return

        body = self.request.body
        if not body:
            logger.warn(
                "Invalid body JSON", method=self.request.method, path=self.request.path
            )
            send_error(self, AppErrors.InvalidJsonBody)
            return

        try:
            body = json_decode(body)
            new_task: Task = parse_task(body)
        except (json.JSONDecodeError, ValidationError):
            logger.warn(
                "Invalid body JSON", method=self.request.method, path=self.request.path
            )
            send_error(self, AppErrors.InvalidJsonBody)
            return

        state: AppState = self.settings["state"]

        async with self.settings["tasks_lock"]:
            task: Task | None = state.tasks.get(task_id)

            if task is None:
                logger.warn(
                    "Task not found",
                    id=str(task_id),
                    method=self.request.method,
                    path=self.request.path,
                )
                send_error(self, AppErrors.TaskNotFound)
                return

            state.tasks[task_id] = new_task

        logger.info(
            "Overwrote task",
            id=str(task_id),
            from_secret=len(task.secret),
            to_secret=len(new_task.secret),
            from_operation=task.operation.lower(),
            to_operation=new_task.operation.lower(),
            method=self.request.method,
        )

        self.set_status(200)
        self.write(new_task.model_dump())
