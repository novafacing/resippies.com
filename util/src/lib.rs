use pwhash::{
    bcrypt::{hash, verify},
    error::Error,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;

pub fn uuid() -> Uuid {
    Uuid::now_v7()
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    hash(password)
}

pub fn validate_password(password: &str, hash: &str) -> bool {
    verify(password, hash)
}

pub fn code() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
