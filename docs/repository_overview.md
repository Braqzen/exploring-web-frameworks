## Repository Structure

If you'd like to poke around the repository then there are 4 primary directories to look at:

- **docker**: configure, build and run services
- **generator**: a service that sends requests to the frameworks
- **services**: contains the APIs/frameworks to send requests to
- **telemetry**: components to capture telemetry from the generator, frameworks and dashboards for visualisations

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

## API

This section describes the data sent to the web-frameworks.

### Payload

The generator creates the following json

```json
{
    secret: String,
    operation: String
}
```

The secret is a randomly generated string and the operation is an enum of 4 valid variants and 1 variant which the servers reject.

```rust
enum Operation {
    Transform,
    Merge,
    Sort,
    Compute,
    Filter // Frameworks reject
}
```

Note that frameworks do no work and the payload is meaningless. The frameworks only store, update and delete payloads from their memory.

### HTTP Methods

The generator sends 5 valid HTTP methods and 1 invalid method (although the frameworks reject everything that isn't the 5 valid methods).

The valid methods are: POST, GET, PUT, PATCH, DELETE\
The invalid method is HEAD

## Data Flow

1. Generator randomly selects 1 framework to send a request to
2. Generator randomly selects a HTTP method to send
3. Generator performs method specific logic to send
4. The selected framework inspects the request and either accepts (and stores the information) or rejects the request

## Randomisation

The generator and frameworks have some chaos built-in to make the visualisations interesting specifically by sending invalid requests, hanging (sleeping upon receiving a request) or outright rejecting requests.

The following does not outline the happy path it only outlines the chaos.

### Generator

- The generator periodically randomises its probability of which HTTP methods and operations to send
- It may send a HEAD HTTP method (all frameworks reject it)
- It may send an invalid Operation (filter, frameworks do not support it)
- It may send an invalid payload identifier (random string or valid but random UUID) instead of the ID associated with the payload
- It may generate a very long secret which causes the servers to reject it because of maximum allowed payload size

### Frameworks

- They may sleep to artificially increase request latency
- They may reject the request with an internal server error response (no reason, purely random)

## Configuration

Currently only the generator has minimal configuration which filters out frameworks so it won't send requests to them.

You may edit the [config](../generator/config.json) but need to rebuild the generator image every time a change is made.

## Visualisations

There are a lot of dashboards from a minimal amount of metrics and misc telemetry. Most of the dashboards are attributed to the generator.

Since the payload is meaningless a lot of the panels are meaningless but you may imagine sending real data and creating dashboards to understand a variety of business and engineering needs.

Moreover, the panels may be grouped differently, some may be useless and removed and perhaps other visualisations may be more useful but again the payload is meaningless in this project.
