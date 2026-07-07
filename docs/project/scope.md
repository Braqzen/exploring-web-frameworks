## Project Scope

The repo consists of the following components:

- 1 Load Generator
- 1 Instance of each framework
- 6 Telemetry components
  - Grafana: UI to visualise telemetry
  - Alloy: collects and forwards telemetry to backends
  - Prometheus: stores metrics
  - Loki: stores logs
  - Tempo: stores traces
  - Pyroscope: stores profiles

The idea is to have something generate requests and try various programming languages and web-frameworks to see what they're like. This means telemetry is required to see what is happening internally and the API is simple and limited.

### Out of Scope

#### Database

There is no database because the concept of shared state can be represented through a HashMap or a database handle.\
Meaning, shutting down services loses state except telemetry which needs a command to specifically delete telemetry.

#### Framework Visualisations

There is data overlap in the generator and the frameworks because of the request that is sent between them.\
Reimplementing the same telemetry per framework is a lot of work therefore most of the telemetry is in the generator instead of being moved into or solely implemented in each framework.

#### Common Web-Server Functionality

The frameworks do not strictly follow any RFCs, they tend to but do not always implement the same functionality across all frameworks and they do not implement Auth or a variety of functionality that may be expected from a safe production-worthy setup.

#### Framework Implementation

There may be more than 1 approach to how a web-framework may be used. Instead of multiple implementations per framework 1 approach has been chosen and it may not be the idiomatic / "best" way of using that framework.
