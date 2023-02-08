FROM rust:latest as builder
COPY . /app

WORKDIR /usr/src/app

RUN cargo build --release

# Path: Dockerfile
FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["./RUST_CLI_DIARY"]


