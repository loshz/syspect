BIN ?= /usr/bin
BPF_OUT ?= ./target/bpf
TARGET ?= x86_64-unknown-linux-gnu

.PHONY: install-rust-tools audit lint test build release install btf compile-btf

install-rust-tools:
	rustup update
	rustup component add rustfmt clippy
	cargo install --locked cargo-deny

audit:
	cargo deny check

lint:
	# Format files in the current crate using rustfmt
	cargo fmt -- --check
	# Check all packages and tests in the current crate and fail on warnings
	cargo clippy --all --tests -- --no-deps -D warnings

test:
	cargo test --no-fail-fast

build:
	cargo build --target ${TARGET}

release:
	cargo build --release --target ${TARGET}

install: release
	sudo cp ./target/${TARGET}/release/lemurs ${BIN}

btf:
	uname -rvmo
	bpftool btf dump file /sys/kernel/btf/vmlinux format c > bpf/vmlinux.h

compile-bpf:
	mkdir -p ${BPF_OUT}
	clang -c -O2 -target bpf -I ./bpf/ -o ${BPF_OUT}/probe.bpf.o ./bpf/probe.bpf.c
