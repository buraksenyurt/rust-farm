FROM rust:1.87.0 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/kivi-store /usr/local/bin/kivi-store
COPY .env .env

EXPOSE 5555

CMD ["kivi-store"]
