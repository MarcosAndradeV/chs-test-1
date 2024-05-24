prog :=chsc

release :=--release --bin
target :=release

build:
	cargo build $(release) chsc

install:
	cp target/$(target)/$(prog) ./$(prog)

chsc: build install

help:
	@echo "usage: make $(prog)"