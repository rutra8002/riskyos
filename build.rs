use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/start.S");

    let out = std::env::var("OUT_DIR").unwrap();
    let obj = format!("{}/start.o", out);

    let status = Command::new("riscv64-linux-gnu-gcc")
        .args(["-c", "src/start.S", "-o", &obj])
        .status()
        .expect("failed to run riscv64-linux-gnu-gcc");

    assert!(status.success(), "assembling src/start.S failed");

    println!("cargo:rustc-link-arg={}", obj);
}