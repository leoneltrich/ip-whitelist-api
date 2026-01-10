# syntax=docker/dockerfile:1
FROM rust:alpine3.20 AS builder

WORKDIR /app

# Install build dependencies (musl-dev for static linking, openssl-dev)
RUN apk add --no-cache musl-dev openssl-dev pkgconfig curl

# Copy entire workspace
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build all binaries
RUN cargo build --release

# --- Auth Service Image ---
FROM alpine:3.20 AS auth_service
WORKDIR /app
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/auth_service .
CMD ["./auth_service"]

# --- Firewall Service Image ---
FROM alpine:3.20 AS firewall_service
WORKDIR /app
# Install nftables (even if using mock backend, good to have available)
RUN apk add --no-cache libgcc nftables
COPY --from=builder /app/target/release/firewall_service .
CMD ["./firewall_service"]