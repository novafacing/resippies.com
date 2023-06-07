use anyhow::{Context, Result};
use pwhash::bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> Result<String> {
    hash(password).context("Password could not be hashed for some reason.")
}

pub fn validate_password(password: &str, hash: &str) -> bool {
    verify(password, hash)
}
