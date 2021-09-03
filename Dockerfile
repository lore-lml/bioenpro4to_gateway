# docker build -t bioenpro4to_gateway .

FROM rust:1.53 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/local/bin
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bioenpro4to_gateway ./app
COPY --from=builder /app/.env .
EXPOSE 8000

CMD ["./app"]