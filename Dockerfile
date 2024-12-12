FROM rust:1.83 AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
# Copy the whole project
COPY . .
# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
# Build the project
RUN cargo build --release --bin my_bot

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my_bot my_bot
EXPOSE 8080
ENTRYPOINT ["./my_bot"]
