.PHONY: default build img

default: build img

build:
	cargo build --release

img:
	./target/release/rt > out.ppm
