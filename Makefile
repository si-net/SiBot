main:
	cargo run

debug:
	RUST_LOG=debug cargo run

build:
	cargo build

release:
	cargo build
	cp target/debug/chat-bot ~/bin/si-net
