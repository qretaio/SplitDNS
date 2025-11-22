
build:
	cargo build

example:
	cargo build --example simple
	sudo ./target/debug/examples/simple

lint: check
	cargo clippy --fix --allow-dirty
	cargo test --doc --all
	cargo test --all
	cargo machete --fix

check: fmt
	cargo sort --workspace
	cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

fmt:
	cargo +nightly fmt --all
