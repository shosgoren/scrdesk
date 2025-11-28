FROM rust:1.75-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY shared ./shared
COPY scrdesk-policy-engine ./scrdesk-policy-engine

RUN cargo build --release --bin scrdesk-policy-engine

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/scrdesk-policy-engine /usr/local/bin/scrdesk-policy-engine

EXPOSE 8003

CMD ["scrdesk-policy-engine"]
