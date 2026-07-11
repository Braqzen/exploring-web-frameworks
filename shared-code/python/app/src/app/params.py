from pydantic import TypeAdapter
from pydantic.types import UUID4
from .task import Task, PatchedTask


def parse_id(id: str):
    return TypeAdapter(UUID4).validate_python(id)


def parse_task(body):
    return Task.model_validate(body)


def parse_patched_task(body):
    return PatchedTask.model_validate(body)
