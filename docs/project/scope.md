# Project Scope

The scope is limited to the following:

- **Load Generator**: Something must send requests to various web-frameworks
- **Web-Frameworks**: Many frameworks in many languages but only 1 implementation of each framework
- **Telemetry**: We must be able to observe what each service is doing

### Frameworks

The frameworks should attempt to follow the same behaviour as closely as the framework, ecosystem tooling and language enables them.

Frameworks have different designs and features so ideally they showcase their uniqueness while being as simple/minimal as possible.

### Telemetry

Telemetry in different languages may not be equally mature (or exist) or may differ. We ought to keep it uniform to display the same information when possible.

Most telemetry should be in the generator and later may be moved into frameworks depending on available tooling.

Use OTeL as much as possible however if there is a popular library then attempt to bridge the library into OTeL format.

Keep the telemetry to a minimum because even a few points can be interpreted and visualised in many ways (as the repo already has).

Use the Grafana stack:

- **UI**: Grafana
- **Collector**: Grafana Alloy
- **Log Storage**: Loki
- **Metric Storage**: Prometheus
- **Trace Storage**: Tempo
- **Profile Storage**: Pyroscope

## Out of Scope

### Database

There will not be a database because the concept of shared state can be represented through a HashMap or a database handle in a framework.

### Testing

There will be no tests. There is consideration for OpenAPI validation which acts as a constraint.

### Web RFCs

The frameworks do not subscribe to any particular RFC but attempt to use the same status codes, http methods, bodies.

### Production Functionality

The frameworks are not meant to be "off-the-shelf" backends therefore common functionality like authentication, authorization, caching, etc. will not be implemented.

### Benchmarking

The code is not meant to be performant and there are other projects that specialise in isolating and comparing the performance of web-frameworks.

### Alerts

Telemetry captures warnings and errors however no alerts will be integrated.

### Cloud Infrastructure

The project is intended to run locally and thus no cloud infra configuration/deployment/3rd party services will be added.
