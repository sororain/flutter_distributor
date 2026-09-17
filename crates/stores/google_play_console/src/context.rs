use crate::auth::{GooglePlayAuth, PUBLISHER_API, PUBLISHER_BASE, UPLOAD_BASE};
use anyhow::{Context as _, Result};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

#[derive(Clone)]
pub struct GooglePlayContext {
    pub api_base: &'static str,
    pub upload_base: &'static str,
    pub auth: GooglePlayAuth,
    pub http: reqwest::Client,
    pub client: crate::Client,
}

impl GooglePlayContext {
    pub async fn from_env() -> Result<Self> {
        let auth = GooglePlayAuth::from_env()?;
        Self::new(auth).await
    }

    pub async fn new(auth: GooglePlayAuth) -> Result<Self> {
        let access_token = auth.access_token().await?;
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {access_token}"))
                .context("failed to build Authorization header")?,
        );
        let http = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")?;
        // 生成的 Client 内部会在 baseurl 后追加 /androidpublisher/v3/...，
        // 所以 baseurl 只到根域名，避免路径重复。
        let client = crate::Client::new_with_client(PUBLISHER_BASE, http.clone());
        Ok(Self {
            api_base: PUBLISHER_API,
            upload_base: UPLOAD_BASE,
            auth,
            http,
            client,
        })
    }

    pub fn api_url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    pub fn upload_url(&self, path: &str) -> String {
        format!("{}{}", self.upload_base, path)
    }
}
