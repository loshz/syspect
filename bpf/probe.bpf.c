#include <bpf/bpf_helpers.h>

#include "vmlinux.h"

char __license[] SEC("license") = "Dual MIT/GPL";

#include "syscalls/sys_enter.bpf.c"
