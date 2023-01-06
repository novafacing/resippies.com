use anyhow::{Context, Result};
use pwhash::{
    sha512_crypt::{hash_with, verify},
    HashSetup,
};

const HASH_PARAMS: HashSetup = HashSetup {
    salt: None,
    rounds: Some(10000),
};

pub fn hash_password(password: &str) -> Result<String> {
    Ok(
        hash_with(HASH_PARAMS, password)
            .context("Password could not be hashed for some reason.")?,
    )
}

pub fn validate_password(password: &str, hash: &str) -> bool {
    verify(password, hash)
}
