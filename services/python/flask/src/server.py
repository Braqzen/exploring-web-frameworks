from flask import Flask
from werkzeug.serving import make_server


def start_server(app: Flask, host: str, port: int) -> None:
    server = make_server(host, port, app)

    def terminate(_signum: int, _frame: object) -> None:
        print("Received terminate signal")
        raise SystemExit(0)

    try:
        print(f"Starting router on {host}:{port}")
        server.serve_forever()
    except KeyboardInterrupt:
        print("Received interrupt signal")
    finally:
        server.server_close()
