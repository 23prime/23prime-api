#!/bin/bash -eu

echo "# Format ================" 
cargo fmt --all -- --check

echo "# Lint ================" 
cargo clippy --all-targets --all-features -- -D warnings -A clippy::needless_return

echo "# Test ================" 
cargo test --all -- --nocapture
