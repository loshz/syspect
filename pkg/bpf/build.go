package bpf

//go:generate go run github.com/cilium/ebpf/cmd/bpf2go -cflags "-g -O2 -Wall -Werror" sys_enter ../../bpf/sys_enter.bpf.c
