FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY shared ./shared
COPY scrdesk-auth-service ./scrdesk-auth-service

# Build
RUN cargo build --release --bin scrdesk-auth-service

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/scrdesk-auth-service /usr/local/bin/scrdesk-auth-service

EXPOSE 8001

CMD ["scrdesk-auth-service"]
