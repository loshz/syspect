BIN ?= /usr/bin
BPF_OUT ?= ./target/bpf
TARGET ?= x86_64-unknown-linux-gnu

.PHONY: install-rust-tools
install-rust-tools:
	rustup update
	rustup component add rustfmt clippy

.PHONY: lint
lint:
	# Format files in the current crate using rustfmt
	cargo fmt -- --check
	# Check all packages and tests in the current crate and fail on warnings
	cargo clippy --all --tests -- --no-deps -D warnings

.PHONY: test
test:
	cargo test --no-fail-fast

.PHONY: build
build:
	cargo build --target ${TARGET}

.PHONY: release
release:
	cargo build --release --target ${TARGET}

.PHONY: install
install: release
	sudo cp ./target/${TARGET}/release/lemurs ${BIN}

.PHONY: btf
btf:
	uname -rvmo
	bpftool btf dump file /sys/kernel/btf/vmlinux format c > bpf/vmlinux.h

.PHONY: compile-bpf
compile-bpf:
	mkdir -p ${BPF_OUT}
	clang -c -O2 -target bpf -I ./bpf/ -o ${BPF_OUT}/probe.bpf.o ./bpf/probe.bpf.c
