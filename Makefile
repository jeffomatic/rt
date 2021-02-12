.PHONY: default build img

default: build img

build:
	cargo build

img:
	./target/debug/rt > out.ppm
