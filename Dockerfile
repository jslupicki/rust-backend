FROM rust:1.85.0

WORKDIR /usr/src/rust-backend
COPY . .

RUN cargo install --path .

CMD ["rust-backend"]

