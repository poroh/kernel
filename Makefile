

all:
	cargo +nightly build

run:
	qemu-system-x86_64 -kernel ./target/x86_64/debug/kernel -serial stdio
