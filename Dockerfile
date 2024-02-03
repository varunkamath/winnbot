FROM rust:1.75 as builder

WORKDIR /usr/src/winn
RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*
RUN pip3 install --break-system-packages cloudscraper
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y build-essential ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*
# RUN ln -s /usr/bin/python3 /usr/bin/python3.11 && ln -s /usr/bin/pip3 /usr/bin/pip3.11
RUN pip3 install --break-system-packages cloudscraper
COPY --from=builder /usr/local/cargo/bin/winn /usr/local/bin/winn
COPY --from=builder /usr/src/winn/src/commands/get_rank.py /usr/local/bin/get_rank.py
COPY entrypoint.sh /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["winn"]