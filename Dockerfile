FROM rust:bookworm AS builder

# Create a new empty shell project
WORKDIR /app
RUN cargo new --bin ap
WORKDIR /app/ap

# Copy manifests first
COPY Cargo.lock Cargo.toml ./

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs
RUN rm target/release/deps/ap*

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

FROM debian:bookworm-slim AS runner

# Install only required runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/ap/target/release/ap ./ap

CMD ["./ap"]