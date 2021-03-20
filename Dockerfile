FROM rust:1.50

WORKDIR /usr/src/rust-backend
COPY . .

RUN cargo install --path .

CMD ["rust-backend"]

