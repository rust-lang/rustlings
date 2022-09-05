# Pull base image.
FROM rust:latest

# Install rustlings
# WORKDIR /temp
# RUN curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash

WORKDIR /temp
RUN apt-get update && apt-get install -y git
RUN git clone -b 5.1.1 --depth 1 https://github.com/rust-lang/rustlings
WORKDIR /temp/rustlings
RUN cargo install --force --path .
RUN rm -rf /temp/rustlings