from pydantic import BaseModel
from .operation import Operation


class Task(BaseModel):
    secret: str
    operation: Operation


class PatchedTask(BaseModel):
    operation: Operation
