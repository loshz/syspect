#[cfg(target_os = "linux")]
fn main() {
    use std::env;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;

    // Build a path of the bpf dirs.
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("bpf");
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // List of bpf probes to compile.
    let probes = vec!["raw_syscalls"];

    // Attempt to collect a list of files with the .bpf.c extension from the src dir.
    probes.into_iter().for_each(|dir| {
        let path = PathBuf::from(&src_dir).join(dir);

        fs::read_dir(&path)
            .unwrap()
            // Filter out dir entries that couldn't be read.
            .filter_map(|res| res.ok())
            // Map dir entries to paths.
            .map(|dir_entry| dir_entry.path())
            // Filter out paths that don't have suffix.
            .filter_map(|path| path.to_string_lossy().ends_with(".bpf.c").then_some(path))
            // Build the bpf skeletion for each src probe.
            .for_each(|src| {
                let mut out = PathBuf::from(&out_dir).join(src.file_name().unwrap());
                out.set_extension("rs");

                libbpf_cargo::SkeletonBuilder::new()
                    .source(&src)
                    .clang_args([
                        OsStr::new("-I"),
                        vmlinux::include_path_root()
                            .join(env::consts::ARCH)
                            .as_os_str(),
                    ])
                    .build_and_generate(&out)
                    .unwrap();
            });
    });

    println!("cargo:rerun-if-changed={}", src_dir.display());
}

#[cfg(not(target_os = "linux"))]
fn main() {
    // panic!("only able to build on linux, kernel header files required");
}
