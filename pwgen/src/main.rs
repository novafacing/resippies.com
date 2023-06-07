use anyhow::{ensure, Context, Result};
use pwhash::bcrypt::{hash, verify};
use std::io::stdin;

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password).context("Password could not be hashed for some reason.")?)
}

pub fn validate_password(password: &str, hash: &str) -> bool {
    verify(password, hash)
}

fn main() -> Result<()> {
    // Read line from stdin
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let hashed = hash_password(&password)?;
    println!("{}", hashed);
    ensure!(
        validate_password(&password, &hashed),
        "Password does not match"
    );
    Ok(())
}
