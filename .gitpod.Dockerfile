FROM rust

RUN curl -o rustup-init.sh --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs && sh rustup-init.sh -y
