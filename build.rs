use std::process::Command;

use libbpf_cargo::SkeletonBuilder;

fn main() {
    // Compile eBPF code if changed.
    println!("cargo:rerun-if-changed=./bpf/tracepoints/");

    // Regenerate vmlinux.h
    Command::new("make").args(["btf"]).status().unwrap();

    SkeletonBuilder::new()
        .source("./bpf/tracepoints/sys_enter.bpf.c")
        .debug(true)
        .clang_args("-c -O2 -I./bpf")
        .build_and_generate("./src/bpf/sys_enter.rs")
        .unwrap();
}
