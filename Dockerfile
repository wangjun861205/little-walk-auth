FROM rust:1.73 AS builder
WORKDIR /usr/src/little-walk-auth
COPY . .
RUN rustup default nightly
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/little-walk-auth /usr/local/bin/little-walk-auth
CMD ["little-walk-auth"]