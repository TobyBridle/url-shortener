FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app-backend

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app-backend/recipe.json recipe.json

# The following command accepts --release
RUN cargo chef cook --release --recipe-path recipe.json 
# Build application
COPY . /app-backend/
RUN cargo build --release --bin shortener-backend

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app-backend
COPY --from=builder /app-backend/target/release/shortener-backend /usr/local/bin
# For some reason, within the container, rocket does not seem to take Rocket.toml into account
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
RUN apt update
RUN apt-get install -y libmariadb-dev libsqlite3-dev libpq-dev
ENTRYPOINT ["/usr/local/bin/shortener-backend"]