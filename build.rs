use std::path::Path;
use std::process::Command;

use libbpf_cargo::SkeletonBuilder;

fn main() {
    // Compile eBPF code if changed.
    println!("cargo:rerun-if-changed=./bpf/tracepoints/");

    // Regenerate vmlinux.h
    Command::new("make").args(&["btf"]).status().unwrap();

    let sys_enter_skel = Path::new("./src/bpf/sys_enter.rs");
    SkeletonBuilder::new("./bpf/tracepoints/sys_enter.bpf.c")
        .clang_args("-c -O2 -I./bpf")
        .generate(&sys_enter_skel)
        .unwrap();
}
