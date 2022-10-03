FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
#FROM rust:1.63.0 As builder
#
## Switch to work dir `app`
#WORKDIR /app
#
#RUN apt update && apt install lld clang -y
#
## Copy all files from our working environment to our Docker image
#COPY . .
#
#ENV SQLX_OFFLINE true
#
#RUN cargo build --release

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin email_news_subscription


# Runtime stage
#FROM rust:1.63.0-slim As runtime
FROM debian:bullseye-slim As runtime
WORKDIR /app

RUN apt-get update -y && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/email_news_subscription email_news_subscription
COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./email_news_subscription"]
