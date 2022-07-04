#include "vmlinux.h"

#include <bpf/bpf_helpers.h>

char __license[] SEC("license") = "Dual MIT/GPL";

#include "tracepoints/sys_enter.bpf.c"
