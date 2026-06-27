use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UniversalConfig {
    pub servers: HashMap<String, ServerConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Transport {
    Http,
    Stdio,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub transport: Transport,
    pub url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

pub fn read_universal_config(path: &Path) -> Result<UniversalConfig> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let config: UniversalConfig = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    for (name, server) in &config.servers {
        match server.transport {
            Transport::Http => {
                if server.url.is_none() {
                    anyhow::bail!("Server '{name}': http transport requires 'url'");
                }
            }
            Transport::Stdio => {
                if server.command.is_none() {
                    anyhow::bail!("Server '{name}': stdio transport requires 'command'");
                }
            }
        }
    }

    Ok(config)
}
