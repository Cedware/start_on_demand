FROM rust:1.76 as builder
WORKDIR /usr/src/start_on_demand
COPY . .
RUN cargo install --path ./server

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/start_on_demand /usr/local/bin/start_on_demand
CMD ["start_on_demand"]