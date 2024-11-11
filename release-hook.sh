#!/bin/bash

# Error out if any command fails
set -e

typos
cargo upgrades

# Similar to CI
cargo clippy -- --deny warnings
cargo fmt --all --check
cargo test --workspace --all-targets
cargo run -- dev check --require-solutions

# MSRV
cargo +1.80 run -- dev check --require-solutions
