prog :=chsc

release :=--release --bin
target :=release

build:
	cargo build $(release) chsc

install:
	cp target/$(target)/$(prog) ./tmp/$(prog)

retest: build install
	python3 test.py retest

chsc: build install retest

test: build install
	python3 test.py

help:
	@echo "usage: make $(prog)"