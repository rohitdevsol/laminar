#!/usr/bin/env bash

# try running chmod +x scripts/check.sh to make it executable

set -e

echo "Formatting..."
cargo fmt --all -- --check

echo "Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Tests..."
cargo test --all-features

echo "Build..."
cargo build --all-targets --all-features

echo "Audit..."
cargo audit

echo "Deny..."
cargo deny check

echo "All checks passed."