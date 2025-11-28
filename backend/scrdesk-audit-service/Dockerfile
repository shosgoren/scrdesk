FROM rust:1.75-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY shared ./shared
COPY scrdesk-audit-service ./scrdesk-audit-service

RUN cargo build --release --bin scrdesk-audit-service

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/scrdesk-audit-service /usr/local/bin/scrdesk-audit-service

EXPOSE 8005

CMD ["scrdesk-audit-service"]
