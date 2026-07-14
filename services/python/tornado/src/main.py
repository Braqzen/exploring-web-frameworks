from os import environ
from telemetry import Telemetry
from server import Server


if __name__ == "__main__":
    log_level: str = environ.get("LOG_LEVEL", "info")
    socket: str = environ["SOCKET"]
    host, _, port = socket.partition(":")

    telemetry = Telemetry("tornado", log_level)
    telemetry.start()

    server = Server(host, int(port))
    # TODO: passing in telemetry is a bad abstraction, but it doesn't make sense in init
    server.run(telemetry)
