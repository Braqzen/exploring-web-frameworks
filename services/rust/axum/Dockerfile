FROM rust:1.91-bookworm AS chef
RUN cargo install --locked cargo-chef

FROM chef AS planner
WORKDIR /app
COPY services/rust/axum .
COPY shared-code /shared-code
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY shared-code /shared-code
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json
COPY services/rust/axum .
COPY shared-code /shared-code
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --bin axum

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/axum /app/axum

CMD ["/app/axum"]
