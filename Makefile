BIN ?= /usr/bin
BPF_OUT ?= ./target/bpf
TARGET ?= x86_64-unknown-linux-gnu

.PHONY: build release install btf bpf

build:
	cargo build --target ${TARGET}

release:
	cargo build --release --target ${TARGET}

install: release
	sudo cp ./target/${TARGET}/release/syspect ${BIN}

btf:
	bpftool btf dump file /sys/kernel/btf/vmlinux format c > ./bpf/vmlinux.h

bpf: btf
	mkdir -p ${BPF_OUT}
	clang -c -O2 -target bpf -I ./bpf/ -o ${BPF_OUT}/probe.bpf.o ./bpf/probe.bpf.c
