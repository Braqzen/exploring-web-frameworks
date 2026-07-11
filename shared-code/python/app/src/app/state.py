from dataclasses import dataclass, field
from uuid import UUID
from .task import Task


@dataclass
class AppState:
    tasks: dict[UUID, Task] = field(default_factory=dict)
