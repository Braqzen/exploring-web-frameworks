from asyncio import Lock
from types import ModuleType
from django.apps import AppConfig

from app.state import AppState


class RoutesConfig(AppConfig):
    name = "routes"

    def __init__(self, app_name: str, app_module: ModuleType) -> None:
        super().__init__(app_name, app_module)
        self.state = AppState()
        self.tasks_lock = Lock()
