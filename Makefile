prog :=chsc

release :=--release --bin
target :=release

build:
	cargo build $(release) chsc

install:
	cp target/$(target)/$(prog) ./tmp/$(prog)

chsc: build install retest

help:
	@echo "usage: make $(prog)"