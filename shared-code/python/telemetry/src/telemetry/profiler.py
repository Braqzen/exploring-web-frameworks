import pyroscope
from os import environ


class TelemetryProfiler:
    def __init__(self, service_name: str) -> None:
        self.service_name = service_name

    def start(self) -> None:
        pyroscope.configure(
            application_name=self.service_name,
            server_address=environ["PYROSCOPE_URL"],
            sample_rate=100,
            oncpu=True,
            tags={"lang": "python"},
        )

    def shutdown(self) -> None:
        pyroscope.shutdown()
