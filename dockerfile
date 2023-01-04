FROM rust:1.63

WORKDIR /usr/src/tiktunnel
COPY . .

RUN cargo install --path .

CMD ["tiktunnel"]