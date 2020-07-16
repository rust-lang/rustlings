FROM rust

RUN curl -O --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs && sh rustup-init.sh -y
