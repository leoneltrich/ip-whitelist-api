# Stage 1: Builder - Using a specific Rust version on Alpine for consistency
FROM rustlang/rust:nightly-alpine3.22 AS builder

# Install necessary build-time dependencies for a statically linked binary
RUN apk add --no-cache musl-dev

# Set the working directory
WORKDIR /usr/src/app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to leverage Docker layer caching for dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
# Build dependencies to cache them
RUN cargo build --release

# Copy the actual source code
COPY ./src ./src

# Build the application, this will be faster as dependencies are cached
# We touch the src files to ensure cargo recompiles the main binary
RUN touch -c src/* && cargo build --release

# Stage 2: Final image - A minimal Alpine base
FROM alpine:3.22
# Install runtime dependencies. `ca-certificates` is often needed for making HTTPS requests.
RUN apk add --no-cache ca-certificates nftables

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/access_proxy_server /usr/local/bin/access_proxy_server

# Set the working directory for the final application
WORKDIR /app

# Expose the application port
EXPOSE 3000

# Set the command to run the application
CMD ["/usr/local/bin/access_proxy_server"]