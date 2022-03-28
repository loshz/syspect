use std::process::Command;

fn main() {
    // Compile bpf code if changed.
    println!("cargo:rerun-if-changed=./bpf/");
    Command::new("make").arg("compile-bpf").status().unwrap();
}
