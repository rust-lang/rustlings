#!/bin/bash

# Error out if any command fails
set -e

cargo run -- dev check
typos
cargo outdated -w --exit-code 1
cargo test --workspace --all-targets
