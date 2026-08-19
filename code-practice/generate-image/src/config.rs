use std::{env, fmt, path::PathBuf};

use thiserror::Error;

pub struct AppConfig {
    api_key: String,
    endpoint: String,
    model: String,
    size: String,
    quality: String,
}

impl fmt::Debug for AppConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AppConfig")
            .field("api_key", &"[REDACTED]")
            .field("endpoint", &self.endpoint)
            .field("model", &self.model)
            .field("size", &self.size)
            .field("quality", &self.quality)
            .finish()
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            api_key: required_env("API_KEY")?,
            endpoint: required_env("IMAGE_API_ENDPOINT")?,
            model: required_env("IMAGE_MODEL")?,
            size: required_env("IMAGE_SIZE")?,
            quality: required_env("IMAGE_QUALITY")?,
        })
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn size(&self) -> &str {
        &self.size
    }

    pub fn quality(&self) -> &str {
        &self.quality
    }

    pub fn output_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("poster.png")
    }

    pub fn dotenv_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".env")
    }
}

fn required_env(name: &'static str) -> Result<String, ConfigError> {
    validate_required_env(name, env::var(name))
}

fn validate_required_env(
    name: &'static str,
    value: Result<String, env::VarError>,
) -> Result<String, ConfigError> {
    let value = value.map_err(|error| match error {
        env::VarError::NotPresent => ConfigError::Missing { name },
        env::VarError::NotUnicode(_) => ConfigError::NotUnicode { name },
    })?;
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(ConfigError::Empty { name });
    }
    Ok(value)
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("环境变量 {name} 未配置")]
    Missing { name: &'static str },
    #[error("环境变量 {name} 不能为空")]
    Empty { name: &'static str },
    #[error("环境变量 {name} 不是有效的 UTF-8")]
    NotUnicode { name: &'static str },
}

#[cfg(test)]
mod tests {
    use std::env::VarError;

    use super::{ConfigError, validate_required_env};

    #[test]
    fn required_env_rejects_empty_values() {
        let name = "GENERATE_IMAGE_TEST_REQUIRED";
        assert!(matches!(
            validate_required_env(name, Ok("  ".into())),
            Err(ConfigError::Empty { name: error_name }) if error_name == name
        ));
    }

    #[test]
    fn required_env_trims_values() {
        let name = "GENERATE_IMAGE_TEST_REQUIRED";
        assert_eq!(
            validate_required_env(name, Ok(" value ".into())).expect("value should be present"),
            "value"
        );
    }

    #[test]
    fn required_env_rejects_missing_values() {
        let name = "GENERATE_IMAGE_TEST_REQUIRED";
        assert!(matches!(
            validate_required_env(name, Err(VarError::NotPresent)),
            Err(ConfigError::Missing { name: error_name }) if error_name == name
        ));
    }
}
