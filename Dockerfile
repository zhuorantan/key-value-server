FROM rust:slim as builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:stable-slim

ENV KV_SERVER_HOST=0.0.0.0
ENV KV_SERVER_FILE_PATH=/app/data.json

COPY --from=builder /app/target/release/key-value-server /
ENTRYPOINT ["/key-value-server"]
