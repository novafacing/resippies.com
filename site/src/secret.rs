use anyhow::{anyhow, Result};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use dotenvy_macro::dotenv;

pub fn secret() -> Result<Vec<u8>> {
    let encoded_secret: &str = dotenv!("SECRET_KEY").trim();
    BASE64_STANDARD_NO_PAD
        .decode(encoded_secret)
        .map_err(|e| anyhow!("Failed to decode secret: {}", e))
}
