FROM rust:1.75-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY scrdesk-core-server ./scrdesk-core-server
COPY scrdesk-auth-service ./scrdesk-auth-service
COPY scrdesk-device-manager ./scrdesk-device-manager
COPY scrdesk-policy-engine ./scrdesk-policy-engine
COPY scrdesk-relay-cluster ./scrdesk-relay-cluster
COPY scrdesk-audit-service ./scrdesk-audit-service
COPY scrdesk-notification-service ./scrdesk-notification-service
COPY scrdesk-billing-service ./scrdesk-billing-service
COPY scrdesk-admin-backend ./scrdesk-admin-backend
COPY scrdesk-update-server ./scrdesk-update-server
COPY scrdesk-logging ./scrdesk-logging
COPY scrdesk-analytics ./scrdesk-analytics
RUN cargo build --release --bin scrdesk-core-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/scrdesk-core-server /usr/local/bin/scrdesk-core-server
EXPOSE 8000
CMD ["scrdesk-core-server"]
