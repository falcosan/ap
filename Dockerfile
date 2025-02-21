FROM rust:1.70 AS builder
WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ap /usr/local/bin/ap
EXPOSE 8888
CMD [ "ap" ]