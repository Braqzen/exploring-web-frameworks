from routes.errors import send_error, AppErrors
from random import randrange
from gevent import sleep


def chaos_middleware():
    if randrange(0, 101) < 5:
        sleep(randrange(500, 1501) / 1_000_000)
    if randrange(0, 101) < 5:
        return send_error(AppErrors.Internal)

    return None
