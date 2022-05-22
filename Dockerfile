FROM rust as builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc

ARG KV_SERVER_HOST=0.0.0.0
ARG KV_SERVER_PORT
ARG KV_SERVER_FILE_PATH=/app/data.json

COPY --from=builder /app/target/release/key-value-server /
ENTRYPOINT ["./key-value-server"]
