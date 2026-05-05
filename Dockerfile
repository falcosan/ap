# syntax=docker/dockerfile:1.7

FROM rust:1-slim-bookworm AS builder
WORKDIR /app
ENV CARGO_TERM_COLOR=always \
    CARGO_NET_RETRY=10 \
    RUSTFLAGS="-C strip=symbols"

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=ap-target,target=/app/target,sharing=locked \
    cargo build --release --locked && \
    cp /app/target/release/ap /usr/local/bin/ap

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder --chown=nonroot:nonroot /usr/local/bin/ap /app/ap
COPY --chown=nonroot:nonroot src/static ./src/static

USER nonroot
ENV PORT=8080 \
    RUST_LOG=info \
    RUST_BACKTRACE=1
EXPOSE 8080
ENTRYPOINT ["/app/ap"]
