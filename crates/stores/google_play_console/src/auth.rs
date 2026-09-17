use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::fs;

pub const CREDENTIALS_ENV: &str = "GOOGLE_PLAY_SERVICE_ACCOUNT_JSON";
pub const TOKEN_URI: &str = "https://oauth2.googleapis.com/token";
pub const ANDROID_PUBLISHER_SCOPE: &str = "https://www.googleapis.com/auth/androidpublisher";
pub const PUBLISHER_BASE: &str = "https://androidpublisher.googleapis.com";
pub const PUBLISHER_API: &str = "https://androidpublisher.googleapis.com/androidpublisher/v3";
pub const UPLOAD_BASE: &str = "https://androidpublisher.googleapis.com/upload/androidpublisher/v3";

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceAccountCredentials {
    pub client_email: String,
    pub private_key: String,
    pub token_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GooglePlayAuth {
    pub credentials: ServiceAccountCredentials,
}

#[derive(Debug, Serialize)]
struct AccessTokenClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: i64,
    exp: i64,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}

impl GooglePlayAuth {
    pub fn from_env() -> Result<Self> {
        let credentials_json = std::env::var(CREDENTIALS_ENV)
            .map_err(|_| anyhow!("missing required environment variable: {CREDENTIALS_ENV}"))?;
        if credentials_json.trim().is_empty() {
            return Err(anyhow!(
                "missing required environment variable: {CREDENTIALS_ENV}"
            ));
        }
        let credentials = parse_credentials(&credentials_json)?;
        Ok(Self { credentials })
    }

    pub async fn access_token(&self) -> Result<String> {
        let issued_at = Utc::now().timestamp();
        let claims = AccessTokenClaims {
            iss: &self.credentials.client_email,
            scope: ANDROID_PUBLISHER_SCOPE,
            aud: TOKEN_URI,
            iat: issued_at,
            exp: issued_at + 3600,
        };
        let jwt = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(self.credentials.private_key.as_bytes())
                .context("failed to parse service account private key")?,
        )
        .context("failed to sign service account JWT")?;

        let response = reqwest::Client::new()
            .post(TOKEN_URI)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", jwt.as_str()),
            ])
            .send()
            .await
            .context("failed to request Google OAuth access token")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "failed to get Google OAuth access token: {status}\n{body}"
            ));
        }

        let body: AccessTokenResponse = response
            .json()
            .await
            .context("failed to decode Google OAuth token response")?;
        Ok(body.access_token)
    }
}

fn parse_credentials(value: &str) -> Result<ServiceAccountCredentials> {
    match serde_json::from_str(value) {
        Ok(credentials) => Ok(credentials),
        Err(json_error) => {
            let content = fs::read_to_string(value).with_context(|| {
                format!(
                    "{CREDENTIALS_ENV} must contain service account JSON or a path to a JSON file; JSON parse error: {json_error}"
                )
            })?;
            serde_json::from_str(&content)
                .with_context(|| format!("failed to parse service account JSON file: {value}"))
        }
    }
}
