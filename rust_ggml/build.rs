extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=ffi_ggml/ggml.c");
    cc::Build::new()
        .files(&[
            "ffi_ggml/ggml.c",
            "ffi_ggml/ggml-aarch64.c",
            "ffi_ggml/ggml-alloc.c",
            "ffi_ggml/ggml-quants.c",
        ])
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-function")
        .flag("-O3")
        .compile("ggml");
}
