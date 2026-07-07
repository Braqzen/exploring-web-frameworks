## Known Issues / Future Work

1. HTTP status codes aren't identical across all frameworks
2. Logs may be missing some parameters or may need to be added
3. Typescript: artificial latency uses setTimeout() which doesn't work sub-ms so its latency is usually higher
4. Frameworks use fewer telemetry than the base Rust/Generator. May split generator telemetry into Generator and Framework instead of current mostly Generator focused view and thus implement more telemetry in frameworks
5. Grafana framework directory has Rust and Typescript profiling as seperate dashboards because profiling is inherently different\
   5.1 Thus provider dropdown is limited and allows all providers instead of limiting to only rust web frameworks in the Rust Profiling and Typescript services in Typescript Profiling
