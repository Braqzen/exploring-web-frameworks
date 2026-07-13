from gevent import monkey

monkey.patch_all()

from os import environ  # noqa: E402
from telemetry import Telemetry  # noqa: E402
from server import Server  # noqa: E402


if __name__ == "__main__":
    log_level: str = environ.get("LOG_LEVEL", "info")
    socket: str = environ["SOCKET"]
    host, _, port = socket.partition(":")

    telemetry = Telemetry("flask", log_level)
    telemetry.start()

    server = Server(host, int(port))
    # TODO: passing in telemetry is a bad abstraction, but it doesn't make sense in init
    server.run(telemetry)
