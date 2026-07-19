# Exploring Web Frameworks

## Overview

This repository attempts to create the same API in different web-frameworks across different programming languages.

The goal is to explore what a "web-framework" is, how different frameworks implement and expose common design concepts/functionality, inherent benefits provided to the framework by its programming language, and to explore each language's ecosystem to develop a minimal prod-adjacent server.

For information about the technical aspects of the project [click here](./docs/project/overview.md).\
For information about which language/frameworks are/may be implemented [click here](./docs/framework_list.md).\
For a breakdown of what a framework is [click here](./docs/framework.md).\
For a web-framework tier-list [click here](./docs/tier-list.md).

This repository is incomplete and thus a work-in-progress. The stages are:

- **Stage 1**: Rough implementation of many frameworks and surface level writeups
- **Stage 2**: Refinement, larger refactoring and API parity
- **Stage 3**: Further reading into each framework for deeper understanding, idiomatic paradigms and refinement of writeups

The repo is currently mostly in **Stage 1**.

## Demo

You may run the project and click around in Grafana's local dashboards to see what the system is doing.

When the project is running give it 1-2 minutes to generate some activity and collect telemetry before exploring the dashboards otherwise missing data may make panels and filters unpopulated (clicking out of the dashboard, waiting a bit and returning fixes it).

### Pre-requisites

The project is run through `docker/docker-compose` and optionally you may install `just` or run the docker commands manually which are in the [justfile](./justfile).

### Quickstart

Building for the first time takes a while because there are a lot of services.

#### Build Docker Images

Build everything in 1 command

```shell
just
```

To build a specific service read the commands in the [justfile](./justfile).

#### Start the services

This will start the services (and pull additional images from DockerHub when running the command for the first time).

```shell
just run
```

Once everything is running click the following link to open Grafana in your browser and explore the dashboards: http://localhost:3000/dashboards

#### Stop the services

Stop everything and retain telemetry state

```shell
just stop
```

Stop everything and delete all previous telemetry

```shell
just clean
```

#### Configuration

You may alter the behaviour of the generator through its [config](./configs/generator.json).

After making a change to the config you must rebuild the generator before running it.

```shell
just build-generator
```
