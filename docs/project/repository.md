# Repository Structure

The repository consists of the following directories:

- [**docker**](../../docker/): All files to configure, build and run services (except .dockerignore)
- [**docs**](../../docs/): All documentation
- [**generator**](../../generator/): The generator service has its own directory but it imports telemetry defined in [**shared-code**](../../shared-code/rust-telemetry/)
- [**services**](../../services/): Contains the code for web-frameworks split by language and they import telemetry from their relevant language in [**shared-code**](../../shared-code/)
- [**shared-code**](../../shared-code/): Contains common code across languages
- [**telemetry**](../../telemetry/): Various files to configure the telemetry services and store pre-made dashboards

There is also a [justfile](../../justfile) which is used to abbreviate commands for building and running images.
