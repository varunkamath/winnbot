FROM rust:1.75 as builder
RUN mkdir -p /usr/src/winnbot
WORKDIR /usr/src/winnbot
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/cargo/bin/winn /usr/local/bin/winn
CMD ["winn"]