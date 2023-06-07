use std::{path::PathBuf, process::Command};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=static/tailwind.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    let manifest_dir = PathBuf::from(CARGO_MANIFEST_DIR);

    if !Command::new(manifest_dir.join("binaries/tailwindcss-linux-x64"))
        .arg("-i")
        .arg(manifest_dir.join("static/input.css"))
        .arg("-o")
        .arg(manifest_dir.join("static/style.css"))
        .status()
        .expect("failed to execute process")
        .success()
    {
        panic!("failed to execute ");
    }
}
