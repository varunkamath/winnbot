FROM rust:1.75

EXPOSE 8080
RUN mkdir -p /usr/src/winnbot
WORKDIR /usr/src/winnbot
COPY . .
RUN cargo install --path .

CMD [ "winn" ]