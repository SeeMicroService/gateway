FROM rust:1.83 AS builder
WORKDIR /gateway
COPY . .
RUN cargo build --release --bin gateway

FROM debian:bookworm-slim
WORKDIR /gateway
RUN apt-get update && apt-get install -y libssl3 && apt-get clean
COPY --from=builder /gateway/target/release/gateway /usr/local/bin
CMD ["gateway"]
