use anyhow::{Context, Result, anyhow};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::Serialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub const KEY_ID_ENV: &str = "APP_STORE_CONNECT_KEY_ID";
pub const ISSUER_ID_ENV: &str = "APP_STORE_CONNECT_ISSUER_ID";
pub const KEY_PATH_ENV: &str = "APP_STORE_CONNECT_KEY_PATH";
pub const API_BASE: &str = "https://api.appstoreconnect.apple.com";

#[derive(Debug, Clone)]
pub struct AppStoreConnectAuth {
    pub key_id: String,
    pub issuer_id: String,
    pub key_path: String,
}

#[derive(Serialize)]
struct Claims<'a> {
    iss: &'a str,
    aud: &'a str,
    exp: usize,
}

impl AppStoreConnectAuth {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            key_id: read_required_env(KEY_ID_ENV)?,
            issuer_id: read_required_env(ISSUER_ID_ENV)?,
            key_path: read_required_env(KEY_PATH_ENV)?,
        })
    }

    pub fn token(&self) -> Result<String> {
        let key = fs::read(&self.key_path)
            .with_context(|| format!("failed to read private key at {}", self.key_path))?;
        let encoding_key =
            EncodingKey::from_ec_pem(&key).context("failed to parse App Store Connect .p8 key")?;
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.key_id.clone());
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("system clock is before UNIX_EPOCH")?
            .as_secs() as usize
            + 20 * 60;
        let claims = Claims {
            iss: &self.issuer_id,
            aud: "appstoreconnect-v1",
            exp,
        };
        encode(&header, &claims, &encoding_key).context("failed to sign JWT")
    }
}

fn read_required_env(name: &str) -> Result<String> {
    let value = std::env::var(name)
        .map_err(|_| anyhow!("missing required environment variable: {name}"))?;
    if value.trim().is_empty() {
        return Err(anyhow!("missing required environment variable: {name}"));
    }
    Ok(value)
}
