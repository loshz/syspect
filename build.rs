use std::path::Path;

use libbpf_cargo::SkeletonBuilder;

fn main() {
    // Compile bpf code if changed.
    println!("cargo:rerun-if-changed=./bpf/");
    let skel = Path::new("./src/bpf/sys_enter.rs");
    SkeletonBuilder::new("./bpf/syscalls/sys_enter.bpf.c")
        .clang_args("-c -O2 -I./bpf")
        .generate(&skel)
        .unwrap();
}
