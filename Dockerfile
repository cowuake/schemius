FROM rust:slim as builder
WORKDIR /usr/src/schemius
COPY . .
RUN cargo install --path schemius-native

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/schemius /usr/local/bin/schemius
ENTRYPOINT ["schemius"]
