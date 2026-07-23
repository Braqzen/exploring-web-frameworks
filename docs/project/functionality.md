# Functionality

This section details the intended functionality of services.

> TODO: upon implementing a testing service empty tables will be filled in

## HTTP Methods and Routes

As defined in [api.md](./api.md) does the service accept/reject methods with the specified routes?

| Framework | Language | POST | GET | PUT | PATCH | DELETE | Unsupported |
| --------- | -------- | ---- | --- | --- | ----- | ------ | ----------- |
| Actix     | Rust     |      |     |     |       |        |             |
| Axum      | Rust     |      |     |     |       |        |             |
| Poem      | Rust     |      |     |     |       |        |             |
| Rocket    | Rust     |      |     |     |       |        |             |
| Salvo     | Rust     |      |     |     |       |        |             |
| Warp      | Rust     |      |     |     |       |        |             |
| Chi       | Go       |      |     |     |       |        |             |
| Echo      | Go       |      |     |     |       |        |             |
| Fiber     | Go       |      |     |     |       |        |             |
| Gin       | Go       |      |     |     |       |        |             |
| Express   | Ts       |      |     |     |       |        |             |
| Fastify   | Ts       |      |     |     |       |        |             |
| Hono      | Ts       |      |     |     |       |        |             |
| Koa       | Ts       |      |     |     |       |        |             |
| Elysia    | Ts       |      |     |     |       |        |             |
| FastAPI   | Python   |      |     |     |       |        |             |
| Starlette | Python   |      |     |     |       |        |             |
| Sanic     | Python   |      |     |     |       |        |             |
| Quart     | Python   |      |     |     |       |        |             |
| Flask     | Python   |      |     |     |       |        |             |
| Django    | Python   |      |     |     |       |        |             |
| Tornado   | Python   |      |     |     |       |        |             |
| Wisp      | Gleam    |      |     |     |       |        |             |

## Responses

As defined in [api.md](./api.md) does the service respond with the expected status code and body?

| Framework | Language | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | 10  | 11  | 12  | 13  |
| --------- | -------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Actix     | Rust     |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Axum      | Rust     |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Poem      | Rust     |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Rocket    | Rust     |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Salvo     | Rust     |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Warp      | Rust     |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Chi       | Go       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Echo      | Go       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Fiber     | Go       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Gin       | Go       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Express   | Ts       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Fastify   | Ts       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Hono      | Ts       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Koa       | Ts       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Elysia    | Ts       |     |     |     |     |     |     |     |     |     |     |     |     |     |
| FastAPI   | Python   |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Starlette | Python   |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Sanic     | Python   |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Quart     | Python   |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Flask     | Python   |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Django    | Python   |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Tornado   | Python   |     |     |     |     |     |     |     |     |     |     |     |     |     |
| Wisp      | Gleam    |     |     |     |     |     |     |     |     |     |     |     |     |     |

## Telemetry

> TODO: Need to define expectations in doc

Expected destination is OTel (logs, metrics, tracing) and Pyroscope (profiling).

| Framework | Language | Logging | Metrics | Profiling | Tracing |
| --------- | -------- | ------- | ------- | --------- | ------- |
| Actix     | Rust     | ✓       | ✗       | ✓         | ✗       |
| Axum      | Rust     | ✓       | ✗       | ✓         | ✗       |
| Poem      | Rust     | ✓       | ✗       | ✓         | ✗       |
| Rocket    | Rust     | ✓       | ✗       | ✓         | ✗       |
| Salvo     | Rust     | ✓       | ✗       | ✓         | ✗       |
| Warp      | Rust     | ✓       | ✗       | ✓         | ✗       |
| Chi       | Go       | ✓       | ✗       | ✓         | ✗       |
| Echo      | Go       | ✓       | ✗       | ✓         | ✗       |
| Fiber     | Go       | ✓       | ✗       | ✓         | ✗       |
| Gin       | Go       | ✓       | ✗       | ✓         | ✗       |
| Express   | Ts       | ✓       | ✗       | ✓         | ✗       |
| Fastify   | Ts       | ✓       | ✗       | ✓         | ✗       |
| Hono      | Ts       | ✓       | ✗       | ✓         | ✗       |
| Koa       | Ts       | ✓       | ✗       | ✓         | ✗       |
| Elysia    | Ts       | ✓       | ✗       | ✓         | ✗       |
| FastAPI   | Python   | ✓       | ✗       | ✓         | ✗       |
| Starlette | Python   | ✓       | ✗       | ✓         | ✗       |
| Sanic     | Python   | ✓       | ✗       | ✓         | ✗       |
| Quart     | Python   | ✓       | ✗       | ✓         | ✗       |
| Flask     | Python   | ✓       | ✗       | ✓         | ✗       |
| Django    | Python   | ✓       | ✗       | ✓         | ✗       |
| Tornado   | Python   | ✓       | ✗       | ✓         | ✗       |
| Wisp      | Gleam    | Stdout  | ✗       | ✗         | ✗       |

## Chaos Middleware

As defined in [simulation.md](./simulation.md) does the service sleep for the correct time range?

| Framework | Language | Latency |
| --------- | -------- | ------- |
| Actix     | Rust     | ✓       |
| Axum      | Rust     | ✓       |
| Poem      | Rust     | ✓       |
| Rocket    | Rust     | ✓       |
| Salvo     | Rust     | ✓       |
| Warp      | Rust     | ✓       |
| Chi       | Go       | ✓       |
| Echo      | Go       | ✓       |
| Fiber     | Go       | ✓       |
| Gin       | Go       | ✓       |
| Express   | Ts       | `1ms`   |
| Fastify   | Ts       | `1ms`   |
| Hono      | Ts       | `1ms`   |
| Koa       | Ts       | `1ms`   |
| Elysia    | Ts       | `1ms`   |
| FastAPI   | Python   | ✓       |
| Starlette | Python   | ✓       |
| Sanic     | Python   | ✓       |
| Quart     | Python   | ✓       |
| Flask     | Python   | ✓       |
| Django    | Python   | ✓       |
| Tornado   | Python   | ✓       |
| Wisp      | Gleam    | `1-2ms` |
