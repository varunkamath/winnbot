FROM rust:1.75 as builder
WORKDIR /usr/src/winnbot
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/winnbot /usr/local/bin/winnbot
CMD ["winnbot"]