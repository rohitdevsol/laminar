run:
	cargo run

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

clean:
	cargo clean

ping-test:
	cargo test --test ping_check -- --nocapture

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

audit:
	cargo audit

deny:
	cargo deny check

fix:
	./scripts/fix.sh

verify:
	./scripts/check.sh

ci:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-features
	cargo build --all-targets --all-features

watch:
	cargo watch -x run

watch-test:
	cargo watch -x test

watch-check:
	cargo watch -x "clippy --all-targets --all-features -- -D warnings"

tree:
	cargo tree

update:
	cargo update