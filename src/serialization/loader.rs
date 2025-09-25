use std::{fs, path::Path};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path = path.as_ref();
    let text = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let cfg = match ext.as_str() {
        "yaml" | "yml" => serde_yaml::from_str(&text)
            .with_context(|| "YAML parse error, check for required attribute here: <link_to_this_yaml_file_spec")?,
        "json" => serde_json::from_str(&text)
            .with_context(|| "JSON parse error, check for required attribute here: <link_to_this_json_file_spec")?,
        other => anyhow::bail!("Unsupported extension: {other}"),
    };

    Ok(cfg)
}
