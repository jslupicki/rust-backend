FROM rust:1.35

WORKDIR /usr/src/rust-backend
COPY . .

RUN cargo install --path .

CMD ["rust-backend"]

