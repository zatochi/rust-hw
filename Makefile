all:
	cargo clippy && cargo fmt --check && cargo test && cargo run --example app

arch:
	rm -f ../rust-2023-02-pavlovich-$(shell git rev-parse --abbrev-ref HEAD).zip && zip -r9 ../rust-2023-02-pavlovich-$(shell git rev-parse --abbrev-ref HEAD).zip . -x "./target/*" -x ".git*" -x ".idea/*" -x "*.swp"

