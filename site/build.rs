use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/routes/*.rs");
    println!("cargo:rerun-if-changed=src/components/*.rs");
    println!("cargo:rerun-if-changed=static/input.css");
    println!("cargo:rerun-if-changed=static/style.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    let manifest_dir = PathBuf::from(CARGO_MANIFEST_DIR);

    let output = Command::new(manifest_dir.join("binaries/tailwindcss-linux-x64"))
        .arg("-m")
        .arg("-i")
        .arg(manifest_dir.join("static/input.css"))
        .arg("-o")
        .arg(manifest_dir.join("static/style.min.css"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Could not run tailwindcss");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        panic!(
            "tailwindcss failed:\nstdout: {}\nstderr: {}",
            stdout, stderr
        );
    }

    if stdout.contains("No utility classes were detected")
        || stderr.contains("No utility classes were detected")
    {
        panic!("No utility classes were detected by tailwindcss");
    }

    let output = Command::new(manifest_dir.join("binaries/tailwindcss-linux-x64"))
        .arg("-i")
        .arg(manifest_dir.join("static/input.css"))
        .arg("-o")
        .arg(manifest_dir.join("static/style.css"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Could not run tailwindcss");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        panic!(
            "tailwindcss failed:\nstdout: {}\nstderr: {}",
            stdout, stderr
        );
    }

    if stdout.contains("No utility classes were detected")
        || stderr.contains("No utility classes were detected")
    {
        panic!("No utility classes were detected by tailwindcss");
    }
}
