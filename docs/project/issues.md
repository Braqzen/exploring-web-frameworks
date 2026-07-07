# Known Issues / Future Work

## Frameworks

1. HTTP status codes aren't identical across all frameworks
2. `Typescript`: artificial latency uses `setTimeout()` which doesn't work sub-ms so its latency is usually higher in dashboards

## Telemetry

1. Logs may be missing some parameters or may need to be added
2. Framework telemetry is limited to logging for non-rust. May expand later
3. Profiling is different in `Rust`/`Typescript` therefore they have seperate dashboards
   1. In those dashboards the provider dropdowns show all frameworks instead of `Rust`/`Typescript` only
