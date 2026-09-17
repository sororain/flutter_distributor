use crate::auth::{API_BASE, AppStoreConnectAuth};
use anyhow::{Context as _, Result};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

#[derive(Clone)]
pub struct AppStoreConnectContext {
    pub api_base: &'static str,
    pub auth: AppStoreConnectAuth,
    pub http: reqwest::Client,
    pub client: crate::Client,
}

impl AppStoreConnectContext {
    pub fn from_env() -> Result<Self> {
        let auth = AppStoreConnectAuth::from_env()?;
        Self::new(auth)
    }

    pub fn new(auth: AppStoreConnectAuth) -> Result<Self> {
        let token = auth.token()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .context("failed to build Authorization header")?,
        );
        let http = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")?;
        let client = crate::Client::new_with_client(API_BASE, http.clone());
        Ok(Self {
            api_base: API_BASE,
            auth,
            http,
            client,
        })
    }

    pub fn url(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{}", self.api_base, path)
        }
    }
}
