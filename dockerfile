FROM rustlang/rust:nightly-bookworm-slim

COPY * /home-core

WORKDIR /home-core

RUN apt install -y sqlcipher && \
    cargo build -r