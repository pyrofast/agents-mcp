use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::Value;

use crate::config::UniversalConfig;

pub mod claude;
pub mod copilot;
pub mod cursor;
pub mod opencode;
pub mod vscode;
pub mod windsurf;

pub trait AgentRenderer: Send + Sync {
    fn name(&self) -> &str;
    fn config_path(&self, project_root: &Path) -> PathBuf;

    fn render(&self, config: &UniversalConfig) -> Value;

    fn write(&self, config: &UniversalConfig, project_root: &Path) -> Result<()> {
        let rendered = self.render(config);
        let path = self.config_path(project_root);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create dirs for {}", path.display()))?;
        }
        let json = serde_json::to_string_pretty(&rendered)
            .context("Failed to serialize config")?;
        fs::write(&path, &json)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        Ok(())
    }

    fn exists(&self, project_root: &Path) -> bool {
        self.config_path(project_root).exists()
    }
}

fn transform_server(server: &crate::config::ServerConfig) -> Value {
    match server.transport {
        crate::config::Transport::Http => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".into(), Value::String("http".into()));
            if let Some(url) = &server.url {
                obj.insert("url".into(), Value::String(url.clone()));
            }
            if let Some(headers) = &server.headers {
                obj.insert("headers".into(), serde_json::to_value(headers).unwrap_or_default());
            }
            Value::Object(obj)
        }
        crate::config::Transport::Stdio => {
            let mut obj = serde_json::Map::new();
            if let Some(cmd) = &server.command {
                obj.insert("command".into(), Value::String(cmd.clone()));
            }
            if let Some(args) = &server.args {
                obj.insert("args".into(), serde_json::to_value(args).unwrap_or_default());
            }
            if let Some(env) = &server.env {
                obj.insert("env".into(), serde_json::to_value(env).unwrap_or_default());
            }
            Value::Object(obj)
        }
    }
}
