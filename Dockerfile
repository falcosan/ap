FROM rust:bookworm AS builder
 
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/ap /app/ap
EXPOSE 8888
CMD ["/ap/app"]