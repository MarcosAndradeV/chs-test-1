prog :=chsvm

release :=--release --bin
target :=release

build:
	cargo build $(release) chsvm

install:
	cp target/$(target)/$(prog) ./tmp/$(prog)

chsvm: build install

help:
	@echo "usage: make $(prog)"