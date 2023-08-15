FROM rust:1.71.1-slim as builder
WORKDIR /usr/src/schemus
COPY . .
RUN cargo install --path ./schemus-native

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/schemus /usr/local/bin/schemus
ENTRYPOINT ["schemus"]
