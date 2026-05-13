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