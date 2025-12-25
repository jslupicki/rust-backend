FROM rust:1.88.0

WORKDIR /usr/src/rust-backend
COPY . .

RUN cargo install --path .

CMD ["rust-backend"]

