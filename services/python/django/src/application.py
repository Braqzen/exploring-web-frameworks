import os
from django.core.handlers.asgi import ASGIHandler
from django.core.asgi import get_asgi_application

os.environ.setdefault("DJANGO_SETTINGS_MODULE", "settings")


class Application:
    def __init__(self) -> None:
        self.app: ASGIHandler = get_asgi_application()
