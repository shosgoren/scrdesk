FROM rust:1.75-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml ./
COPY shared ./shared
COPY scrdesk-relay-cluster ./scrdesk-relay-cluster
RUN cargo build --release --bin scrdesk-relay-cluster

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/scrdesk-relay-cluster /usr/local/bin/scrdesk-relay-cluster
EXPOSE 21116 21117
CMD ["scrdesk-relay-cluster"]
