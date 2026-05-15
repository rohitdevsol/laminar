#!/usr/bin/env bash

#  chmod +x scripts/fix.sh

set -e

echo "==> Formatting"
cargo fmt

echo "==> Applying clippy fixes"
cargo clippy --fix --allow-dirty --allow-staged

echo "==> Final clippy verification"
cargo clippy --all-targets --all-features -- -D warnings

echo "==> Audit"
cargo audit

echo "==> Deny"
cargo deny check

echo "==> All fixes applied"