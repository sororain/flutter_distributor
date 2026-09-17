use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};

use super::{S3PublishOptions, optional_value, parse_bool, required_value, upload_artifact};

pub struct OssPublisher;

const PUBLISHER_NAME: &str = "oss";
const ENV_OSS_ACCESS_KEY: &str = "OSS_ACCESS_KEY";
const ENV_OSS_SECRET_KEY: &str = "OSS_SECRET_KEY";
const ENV_OSS_BUCKET: &str = "OSS_BUCKET";
const ENV_OSS_REGION: &str = "OSS_REGION";
const ENV_OSS_KEY_PREFIX: &str = "OSS_KEY_PREFIX";
const ENV_OSS_PUBLIC_BASE_URL: &str = "OSS_PUBLIC_BASE_URL";
const ENV_OSS_FORCE_PATH_STYLE: &str = "OSS_FORCE_PATH_STYLE";

impl AppPublisher for OssPublisher {
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
        &["access-key", "oss-access-key"],
        &[ENV_OSS_ACCESS_KEY],
        "OSS access key",
    )?;
    let secret_key = required_value(
        config,
        &["secret-key", "oss-secret-key"],
        &[ENV_OSS_SECRET_KEY],
        "OSS secret key",
    )?;
    let bucket = required_value(
        config,
        &["bucket", "oss-bucket"],
        &[ENV_OSS_BUCKET],
        "OSS bucket",
    )?;
    let region = required_value(
        config,
        &["region", "oss-region"],
        &[ENV_OSS_REGION],
        "OSS region",
    )?;
    let endpoint = optional_value(config, &["endpoint", "oss-endpoint"], &[])
        .unwrap_or_else(|| format!("oss-{region}.aliyuncs.com"));
    let key_prefix = optional_value(
        config,
        &["savekey-prefix", "key-prefix", "oss-key-prefix"],
        &[ENV_OSS_KEY_PREFIX],
    );
    let public_base_url = optional_value(
        config,
        &["public-base-url", "bucket-domain", "oss-public-base-url"],
        &[ENV_OSS_PUBLIC_BASE_URL],
    );
    let force_path_style = optional_value(
        config,
        &["force-path-style", "oss-force-path-style"],
        &[ENV_OSS_FORCE_PATH_STYLE],
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
