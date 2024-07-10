build:
	cargo build --bin chsc

release:
	cargo build --release --bin chsc

test: build
	./rere.py replay test.list

record: build
	./rere.py record test.list

install: release
	cp target/release/chsc ./chsc

chsc: release 

help:
	@echo "usage: make $(prog)"