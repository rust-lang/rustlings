#!/bin/bash

# Error out if any command fails
set -e

typos
cargo outdated -w --exit-code 1
cargo test --workspace --all-targets
