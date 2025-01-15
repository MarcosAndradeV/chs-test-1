build:
	cargo build --bin chs

release:
	cargo build --release --bin chs

chsc: release

help:
	@echo "usage: make $(prog)"
