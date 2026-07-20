from random import randrange
from gevent import sleep
from flask import current_app

from app.config import Config
from routes.errors import send_error, AppErrors


def chaos_middleware():
    config: Config = current_app.extensions["config"]

    if config.latency.enabled and randrange(0, 101) < config.latency.rate:
        sleep(randrange(500, 1501) / 1_000_000)
    if config.error.enabled and randrange(0, 101) < config.error.rate:
        return send_error(AppErrors.Internal)

    return None
