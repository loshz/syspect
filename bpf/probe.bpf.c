#include "vmlinux.h"

#include <bpf/bpf_helpers.h>

char __license[] SEC("license") = "Dual MIT/GPL";

#include "syscalls/sys_enter.bpf.c"
