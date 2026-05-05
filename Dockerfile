# syntax=docker/dockerfile:1.7
FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ap /app/ap
COPY src/static ./src/static
ENV PORT=8080
EXPOSE 8080
CMD ["/app/ap"]
