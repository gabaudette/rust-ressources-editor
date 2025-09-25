use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EditorData {
    pub name: String,

    #[serde(default)]
    pub version: Option<String>,

    #[serde(default)]
    pub metadata: Option<serde_json::Value>,

    #[serde(default)]
    pub ressources: Vec<serde_json::Value>,

    #[serde(default)]
    pub events: Vec<serde_json::Value>,

    #[serde(default)]
    pub locations: Vec<serde_json::Value>, 
}

impl Default for EditorData {
    fn default() -> Self {
        EditorData {
            name: "New Project".to_string(),
            version: Some("0.1.0".to_string()),
            metadata: None,
            ressources: vec![],
            events: vec![],
            locations: vec![],
        }
    }
}

pub fn load_editor<P: AsRef<Path>>(path: P) -> Result<EditorData> {
    let path = path.as_ref();
    let text =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let cfg = match ext.as_str() {
        "yaml" | "yml" => serde_yaml::from_str(&text).with_context(|| {
            "YAML parse error, check for required attribute here: <link_to_this_yaml_file_spec"
        })?,
        "json" => serde_json::from_str(&text).with_context(|| {
            "JSON parse error, check for required attribute here: <link_to_this_json_file_spec"
        })?,
        other => anyhow::bail!("Unsupported extension: {other}"),
    };

    Ok(cfg)
}
