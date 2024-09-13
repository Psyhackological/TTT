use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDefaultConfig {
    #[serde(rename = "PRIORITY")]
    pub priority: u8,

    #[serde(rename = "AUTHOR")]
    pub author: String,

    #[serde(rename = "TIMEZONE")]
    pub timezone: String,

    #[serde(rename = "DURATION")]
    pub duration: u16,

    #[serde(rename = "DURATION_UNIT")]
    pub duration_unit: String,
}

impl TryFrom<&Path> for TaskDefaultConfig {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self> {
        let toml_content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file at path: {}", path.display()))?;

        let config: Self =
            toml::from_str(&toml_content).with_context(|| "Failed to parse TOML content.")?;

        Ok(config)
    }
}
