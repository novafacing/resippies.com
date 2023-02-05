use std::{fs::OpenOptions, io::Write, path::Path, process::Command};

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use rand::{thread_rng, Rng};

const DB_PATH: &str = "db/resippies.com.sqlite";

pub fn main() {
    // Only rerun if build script or .env is changed
    println!("cargo:rerun-if-changed=build.rs");
    // If the .env file is missing, create it
    // Check if the .env file exists
    if !Path::new(".env").exists() {
        // Create the .env file
        println!("cargo:rerun-if-changed=.env");
    }

    // Open the .env file and write the secret key to it
    let secret = thread_rng().gen::<[u8; 64]>();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/.env", env!("CARGO_MANIFEST_DIR")))
        .expect("Failed to open .env file");

    let encoded = BASE64_STANDARD_NO_PAD.encode(secret);

    writeln!(file, "SECRET_KEY={}", encoded).expect("Failed to write to .env file");
    writeln!(file, "DATABASE_URL=sqlite://{}", DB_PATH).expect("Failed to write to .env file");

    Command::new("sqlx")
        .arg("database")
        .arg("reset")
        .arg("-y")
        .output()
        .expect("Failed to reset database");
}
