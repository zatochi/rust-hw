all:
	cargo clippy && cargo fmt --check && cargo run
