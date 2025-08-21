#!/bin/bash

# Error out if any command fails
set -e

typos
cargo upgrades

# Similar to CI
cargo clippy -- --deny warnings
cargo fmt --all --check
cargo test --workspace
cargo dev check --require-solutions

# MSRV
cargo +1.88 dev check --require-solutions
