from operation import Operation
from pydantic import BaseModel


class Task(BaseModel):
    secret: str
    operation: Operation


class PatchedTask(BaseModel):
    operation: Operation
