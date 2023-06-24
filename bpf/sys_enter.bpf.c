#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>

#define TASK_COMM_LEN 16

struct map_key {
    char proc_name[TASK_COMM_LEN];
    pid_t pid;
} __attribute__((packed));

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 8192);
    __type(key, struct map_key);
    __type(value, u64);
} syscall_count SEC(".maps");

SEC("tracepoint/raw_syscalls/sys_enter")
int sys_enter(struct trace_event_raw_sys_enter *args)
{
    struct task_struct *task = (struct task_struct *) bpf_get_current_task();

    /* Setup key */
    struct map_key key;
    __builtin_memcpy(key.proc_name, BPF_CORE_READ(task, comm), sizeof(task->comm));
    key.pid = BPF_CORE_READ(task, pid);

    /* Update count */
    int *count = bpf_map_lookup_elem(&syscall_count, &key);
    if (!count) {
        u64 zero = 0;
        bpf_map_update_elem(&syscall_count, &key, &zero, 0);
        /* bpf_printk("(%s, %d) = 0", key.proc_name, key.syscall_nr); */
    } else {
        (*count)++;
        /* bpf_printk("(%s, %d) = %d", key.proc_name, key.syscall_nr, *count); */
    }

    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";
