# syntax=docker/dockerfile:experimental


FROM rust:1.46.0-buster
ENV HOME=/home/root

WORKDIR $HOME/app

ARG database_url="required"

RUN apt-get -y update && apt-get -y install gcc-arm-linux-gnueabihf
RUN rustup target add arm-unknown-linux-gnueabihf
RUN mkdir -p /source /.cargo /app && \
    echo "[target.arm-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"" > /.cargo/config

ADD src src
ADD Cargo.lock .
ADD Cargo.toml .



RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/home/root/app/target \
    DATABASE_URL="$database_url" cargo build --target arm-unknown-linux-gnueabihf --release