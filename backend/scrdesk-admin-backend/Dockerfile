FROM rust:1.75-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml ./
COPY shared ./shared
COPY scrdesk-admin-backend ./scrdesk-admin-backend
RUN cargo build --release --bin scrdesk-admin-backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/scrdesk-admin-backend /usr/local/bin/scrdesk-admin-backend
EXPOSE 8006
CMD ["scrdesk-admin-backend"]
