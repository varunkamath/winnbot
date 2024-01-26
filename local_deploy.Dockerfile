FROM rust:1.75 as builder
WORKDIR /usr/src/winn
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y build-essential ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/winn/env_vars /usr/local/bin/env_vars
COPY --from=builder /usr/local/cargo/bin/winn /usr/local/bin/winn
COPY entrypoint.sh /app/entrypoint.sh
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["winn"]