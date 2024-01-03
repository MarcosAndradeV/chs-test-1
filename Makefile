prog :=chsvm

release :=--release --bin
target :=release

build:
	cargo build $(release) chsvm

install:
	cp target/$(target)/$(prog) ./tmp/$(prog)

retest: build install
	python3 test.py retest

chsvm: build install retest

test: build install
	python3 test.py

help:
	@echo "usage: make $(prog)"