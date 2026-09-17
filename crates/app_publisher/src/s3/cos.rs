use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};

use super::{S3PublishOptions, optional_value, parse_bool, required_value, upload_artifact};

pub struct CosPublisher;

const PUBLISHER_NAME: &str = "cos";
const ENV_COS_ACCESS_KEY: &str = "COS_ACCESS_KEY";
const ENV_COS_SECRET_KEY: &str = "COS_SECRET_KEY";
const ENV_COS_BUCKET: &str = "COS_BUCKET";
const ENV_COS_REGION: &str = "COS_REGION";
const ENV_COS_KEY_PREFIX: &str = "COS_KEY_PREFIX";
const ENV_COS_PUBLIC_BASE_URL: &str = "COS_PUBLIC_BASE_URL";
const ENV_COS_FORCE_PATH_STYLE: &str = "COS_FORCE_PATH_STYLE";

impl AppPublisher for CosPublisher {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        PUBLISHER_NAME
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn perform_publish(
        &self,
        config: &PublishConfig,
        on_progress: Option<&PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError> {
        let artifact_path = config
            .artifact_path
            .as_deref()
            .ok_or_else(|| PublishError::MissingArgument("artifact_path".to_string()))?;
        let options = build_options(config)?;
        upload_artifact(&options, artifact_path, on_progress)
    }
}

fn build_options(config: &PublishConfig) -> Result<S3PublishOptions, PublishError> {
    let access_key = required_value(
        config,
        &["access-key", "cos-access-key"],
        &[ENV_COS_ACCESS_KEY],
        "COS access key",
    )?;
    let secret_key = required_value(
        config,
        &["secret-key", "cos-secret-key"],
        &[ENV_COS_SECRET_KEY],
        "COS secret key",
    )?;
    let bucket = required_value(
        config,
        &["bucket", "cos-bucket"],
        &[ENV_COS_BUCKET],
        "COS bucket",
    )?;
    let region = required_value(
        config,
        &["region", "cos-region"],
        &[ENV_COS_REGION],
        "COS region",
    )?;
    let endpoint = optional_value(config, &["endpoint", "cos-endpoint"], &[])
        .unwrap_or_else(|| format!("cos.{region}.myqcloud.com"));
    let key_prefix = optional_value(
        config,
        &["savekey-prefix", "key-prefix", "cos-key-prefix"],
        &[ENV_COS_KEY_PREFIX],
    );
    let public_base_url = optional_value(
        config,
        &["public-base-url", "bucket-domain", "cos-public-base-url"],
        &[ENV_COS_PUBLIC_BASE_URL],
    );
    let force_path_style = optional_value(
        config,
        &["force-path-style", "cos-force-path-style"],
        &[ENV_COS_FORCE_PATH_STYLE],
    )
    .map(|v| parse_bool(&v))
    .transpose()?
    .unwrap_or(false);

    Ok(S3PublishOptions {
        endpoint,
        region,
        access_key,
        secret_key,
        session_token: None,
        bucket,
        key_prefix,
        public_base_url,
        force_path_style,
    })
}
