FROM rust:1.70.0-slim-buster AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Install dev tools
RUN cargo install sqlx-cli --no-default-features -F sqlite -F rustls
# Build application
COPY . .
ENV DATABASE_URL=sqlite://db.sqlite3
RUN touch db.sqlite3 && sqlx migrate run
RUN cargo build --release --bin sdgenbox

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
RUN apt-get update && apt-get install exiftool -y && apt-get clean
WORKDIR /app
COPY ./migrations /app/migrations
COPY ./static /app/static
COPY --from=builder /app/target/release/sdgenbox /usr/local/bin
CMD ["/usr/local/bin/sdgenbox"]
