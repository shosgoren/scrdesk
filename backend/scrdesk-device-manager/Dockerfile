FROM rust:1.75-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY shared ./shared
COPY scrdesk-device-manager ./scrdesk-device-manager

RUN cargo build --release --bin scrdesk-device-manager

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/scrdesk-device-manager /usr/local/bin/scrdesk-device-manager

EXPOSE 8002

CMD ["scrdesk-device-manager"]
