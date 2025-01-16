build:
	cargo build --bin chs

release:
	cargo build --release --bin chs

test:
	cargo test

chsc: release

help:
	@echo "usage: make $(prog)"
