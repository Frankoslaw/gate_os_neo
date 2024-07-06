fn main() {
    match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => {
            println!("cargo:rustc-link-arg=-Tlinkers/x86_64.ld");
            println!("cargo:rerun-if-changed=linkers/x86_64.ld");
        }
        "aarch64" => {
            println!("cargo:rustc-link-arg=-Tlinkers/aarch64.ld");
            println!("cargo:rerun-if-changed=linkers/aarch64.ld");
        }
        "riscv64" => {
            println!("cargo:rustc-link-arg=-Tlinkers/riscv64.ld");
            println!("cargo:rerun-if-changed=linkers/riscv64.ld");
        }
        _ => panic!("Unsupported target"),
    };
}
