FROM rust:1.73 AS builder
WORKDIR /usr/src/little-walk-auth
COPY . .
RUN rustup default nightly
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/little-walk-auth /usr/local/bin/little-walk-auth
CMD ["little-walk-auth"]