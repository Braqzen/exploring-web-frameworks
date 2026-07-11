from os import environ
from application import create_app
from server import start_server

if __name__ == "__main__":
    socket = environ["SOCKET"]
    host, _, port = socket.partition(":")

    app = create_app()

    start_server(app, host, int(port))
