use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};

use super::{S3PublishOptions, optional_value, parse_bool, required_value, upload_artifact};

pub struct QiniuPublisher;

const PUBLISHER_NAME: &str = "qiniu";
const ENV_QINIU_ACCESS_KEY: &str = "QINIU_ACCESS_KEY";
const ENV_QINIU_SECRET_KEY: &str = "QINIU_SECRET_KEY";
const ENV_QINIU_BUCKET: &str = "QINIU_BUCKET";
const ENV_QINIU_REGION: &str = "QINIU_REGION";
const ENV_QINIU_KEY_PREFIX: &str = "QINIU_KEY_PREFIX";
const ENV_QINIU_PUBLIC_BASE_URL: &str = "QINIU_PUBLIC_BASE_URL";
const ENV_QINIU_FORCE_PATH_STYLE: &str = "QINIU_FORCE_PATH_STYLE";
const DEFAULT_REGION: &str = "cn-east-1";

impl AppPublisher for QiniuPublisher {
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
        &["access-key", "qiniu-access-key"],
        &[ENV_QINIU_ACCESS_KEY],
        "Qiniu access key",
    )?;
    let secret_key = required_value(
        config,
        &["secret-key", "qiniu-secret-key"],
        &[ENV_QINIU_SECRET_KEY],
        "Qiniu secret key",
    )?;
    let bucket = required_value(
        config,
        &["bucket", "qiniu-bucket"],
        &[ENV_QINIU_BUCKET],
        "Qiniu bucket",
    )?;
    let region = optional_value(config, &["region", "qiniu-region"], &[ENV_QINIU_REGION])
        .unwrap_or_else(|| DEFAULT_REGION.to_string());
    let endpoint = optional_value(config, &["endpoint", "qiniu-endpoint"], &[])
        .unwrap_or_else(|| format!("s3-{region}.qiniucs.com"));
    let key_prefix = optional_value(
        config,
        &["savekey-prefix", "key-prefix", "qiniu-key-prefix"],
        &[ENV_QINIU_KEY_PREFIX],
    );
    let public_base_url = optional_value(
        config,
        &["public-base-url", "bucket-domain", "qiniu-public-base-url"],
        &[ENV_QINIU_PUBLIC_BASE_URL],
    );
    let force_path_style = optional_value(
        config,
        &["force-path-style", "qiniu-force-path-style"],
        &[ENV_QINIU_FORCE_PATH_STYLE],
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
