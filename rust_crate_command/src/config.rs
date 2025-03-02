use serde::Deserialize;
use serde_yml::Value;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use thiserror::Error;

/// Error type for Rinf configuration loading.
#[derive(Debug, Error)]
pub enum RinfCommandError {
    #[error("Failed to read YAML file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yml::Error),

    #[error("Unknown key '{0}' in rinf config. Available keys are: {1}")]
    UnknownKey(String, String),

    #[error("Unknown error")]
    Other, // TODO: Remove
}

// TODO: Remove the message.

/// Rinf message configuration structure.
#[derive(Deserialize)]
pub struct RinfConfigMessage {
    pub input_dir: String,
    pub rust_output_dir: String,
    pub dart_output_dir: String,
    pub rust_serde: bool,
}

impl Display for RinfConfigMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "message:\
            \n  input_dir: {}\
            \n  rust_output_dir: {}\
            \n  dart_output_dir: {}\
            \n  rust_serde: {}",
            self.input_dir,
            self.rust_output_dir,
            self.dart_output_dir,
            self.rust_serde
        )
    }
}

impl Default for RinfConfigMessage {
    fn default() -> Self {
        Self {
            input_dir: "messages/".to_string(),
            rust_output_dir: "native/hub/src/messages/".to_string(),
            dart_output_dir: "lib/messages/".to_string(),
            rust_serde: false,
        }
    }
}

impl RinfConfigMessage {
    /// Constructs a `RinfConfigMessage` from a YAML map.
    pub fn from_yaml(
        yaml: &serde_yml::Mapping,
    ) -> Result<Self, RinfCommandError> {
        let valid_keys: HashSet<&str> = [
            "input_dir",
            "rust_output_dir",
            "dart_output_dir",
            "rust_serde",
        ]
        .into_iter()
        .collect();

        for key in yaml.keys() {
            if let Some(key_str) = key.as_str() {
                if !valid_keys.contains(key_str) {
                    return Err(RinfCommandError::UnknownKey(
                        key_str.to_string(),
                        valid_keys
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", "),
                    ));
                }
            }
        }

        Ok(Self {
            input_dir: yaml
                .get("input_dir")
                .and_then(Value::as_str)
                .unwrap_or("messages/")
                .to_string(),
            rust_output_dir: yaml
                .get("rust_output_dir")
                .and_then(Value::as_str)
                .unwrap_or("native/hub/src/messages/")
                .to_string(),
            dart_output_dir: yaml
                .get("dart_output_dir")
                .and_then(Value::as_str)
                .unwrap_or("lib/messages/")
                .to_string(),
            rust_serde: yaml
                .get("rust_serde")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        })
    }
}

/// Main Rinf configuration structure.
#[derive(Deserialize, Default)]
pub struct RinfConfig {
    pub message: RinfConfigMessage,
}

impl Display for RinfConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// TODO: Make `from_yaml` a trait method.

impl RinfConfig {
    /// Constructs a `RinfConfig` from a YAML map.
    pub fn from_yaml(
        yaml: &serde_yml::Mapping,
    ) -> Result<Self, RinfCommandError> {
        let valid_keys: HashSet<&str> = ["message"].into_iter().collect();

        for key in yaml.keys() {
            if let Some(key_str) = key.as_str() {
                if !valid_keys.contains(key_str) {
                    return Err(RinfCommandError::UnknownKey(
                        key_str.to_string(),
                        valid_keys
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", "),
                    ));
                }
            }
        }

        let message_yaml = yaml.get("message").and_then(Value::as_mapping);
        let message = match message_yaml {
            Some(map) => RinfConfigMessage::from_yaml(map)?,
            None => RinfConfigMessage::default(),
        };

        Ok(Self { message })
    }
}

/// Loads and verifies the rinf configuration from a YAML file.
pub fn load_verified_rinf_config(
    file_path: &str,
) -> Result<RinfConfig, RinfCommandError> {
    let content = fs::read_to_string(file_path)?;
    let yaml: Value = serde_yml::from_str(&content)?;
    let rinf_yaml = yaml
        .as_mapping()
        .ok_or(RinfCommandError::Other)?
        .get("rinf")
        .and_then(Value::as_mapping);
    let config = match rinf_yaml {
        Some(map) => RinfConfig::from_yaml(map)?,
        None => RinfConfig::default(),
    };
    Ok(config)
}
