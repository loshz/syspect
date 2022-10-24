#include "vmlinux.h"

#include <bpf/bpf_helpers.h>

struct {
  __uint(type, BPF_MAP_TYPE_ARRAY);
  __type(key, u32);
  __type(value, u64);
  __uint(max_entries, 1000);
} syscall_count SEC(".maps");

// This tracepoint is defined in:
// /sys/kernel/debug/tracing/events/raw_syscalls/sys_enter
SEC("tracepoint/raw_syscalls/sys_enter")
int sys_enter(struct bpf_raw_tracepoint_args *ctx) {
  struct pt_regs *regs;
  const char *key;
  u64 count = 1;

  // Get value from CPU register.
  regs = (struct pt_regs *)ctx->args[0];
  bpf_probe_read(&key, sizeof(key), &regs->si);

  // Write key=program, value=count to the map.
  bpf_map_update_elem(&syscall_count, &key, &count, BPF_ANY);

  return 0;
}
