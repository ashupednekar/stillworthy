use libbpf_cargo::SkeletonBuilder;
use std::env;
use std::path::PathBuf;

const SRC: &str = "src/bpf/syscall_counter.bpf.c";

fn main() {
    let mut out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    out.push("syscall_counter.skel.rs");

    SkeletonBuilder::new()
        .source(SRC)
        .build_and_generate(&out)
        .unwrap();

    println!("cargo:rerun-if-changed={}", SRC);
}
