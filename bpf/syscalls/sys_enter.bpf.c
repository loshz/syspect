#include "vmlinux.h"

#include <bpf/bpf_helpers.h>

char __license[] SEC("license") = "Dual MIT/GPL";

struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u64);
    __uint(max_entries, 1);
} syscall_count SEC(".maps");

// This tracepoint is defined in:
// /sys/kernel/debug/tracing/events/raw_syscalls/sys_enter
SEC("tracepoint/raw_syscalls/sys_enter")
int sys_enter() {
    u32 key = 0;
    u64 init_val = 1, *count;

    count = bpf_map_lookup_elem(&syscall_count, &key);
    if (!count) {
        bpf_map_update_elem(&syscall_count, &key, &init_val, BPF_ANY);
        return 0;
    }
    __sync_fetch_and_add(count, 1);

    return 0;
}
