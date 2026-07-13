SECRET_KEY = "dev-only"
ROOT_URLCONF = "urls"
ALLOWED_HOSTS = ["*"]
DATA_UPLOAD_MAX_MEMORY_SIZE = 64 * 1024  # TODO: make configurable?
MIDDLEWARE = [
    "routes.middleware.log.LogMiddleware",
    "routes.middleware.chaos.ChaosMiddleware",
    "routes.middleware.head.RejectHeadMiddleware",
]
INSTALLED_APPS = [
    "routes.apps.RoutesConfig",
]
LOGGING = {
    "version": 1,
    "disable_existing_loggers": False,
    "handlers": {
        "null": {"class": "logging.NullHandler"},
    },
    "loggers": {
        "django": {"handlers": ["null"], "propagate": False},
        "django.request": {"handlers": ["null"], "propagate": False},
        "django.server": {"handlers": ["null"], "propagate": False},
    },
}
