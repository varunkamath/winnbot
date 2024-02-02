FROM rust:1.75 as builder
WORKDIR /usr/src/winn
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y build-essential ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
# Install Python 3.12.1, and the pip package cloudscraper
RUN apt-get update && apt-get install -y python3.12 python3-pip
RUN pip3 install cloudscraper
# Symlink python3.12 to python3 and pip3.12 to pip3 and python3 to python and pip3 to pip
RUN ln -s /usr/bin/python3.12 /usr/bin/python3 && ln -s /usr/bin/pip3.12 /usr/bin/pip3 && ln -s /usr/bin/python3 /usr/bin/python && ln -s /usr/bin/pip3 /usr/bin/pip
# COPY --from=builder /usr/src/winn/src/auto/data/data.txt /usr/local/cargo/bin/data/data.txt
COPY --from=builder /usr/local/cargo/bin/winn /usr/local/bin/winn
COPY entrypoint.sh /app/entrypoint.sh
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["winn"]