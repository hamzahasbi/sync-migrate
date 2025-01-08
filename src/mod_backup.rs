use serde_derive::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Deserialize)]
#[derive(serde::Serialize)]
pub struct BackupConfig {
    pub omit_folders: Vec<String>,
    pub backup_paths: Vec<String>,
}

impl BackupConfig {
    pub fn load(config_path: &str) -> Result<Self> {
        let config_str = fs::read_to_string(config_path)?;
        let config: BackupConfig = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn default_config() -> Self {
        BackupConfig {
            omit_folders: vec![
                ".bundle".to_string(),
                ".cache".to_string(),
                ".local".to_string(),
                ".docker".to_string(),
                ".npm".to_string(),
                ".drush".to_string(),
                ".orbstack".to_string(),
                ".ssh".to_string(),
                ".gem".to_string(),
                ".gitconfig".to_string(),
                ".minikube".to_string(),
                ".mitmproxy".to_string(),
                ".proxyman-data".to_string(),
                ".supermaven".to_string(),
                ".redis-insight".to_string(),
                ".rbenv".to_string(),
                ".bun".to_string(),
                ".bundle".to_string(),
                ".composer".to_string(),
                ".th-client".to_string(),
                ".pdepend".to_string(),
                ".yarnrc".to_string(),
                ".CFUserTextEncoding".to_string(),
                ".npm".to_string(),
                ".wget-hsts".to_string(),
                ".DS_Store".to_string(),
                ".Trash".to_string(),
                ".node_repl_history".to_string(),
                ".gk".to_string(),
                ".sao".to_string(),
            ],
            backup_paths: vec![
                // "~/Documents".to_string(),
            ],
        }
    }

    pub fn create_default_config(config_path: &str) -> Result<()> {
        let config = Self::default_config();
        let toml_string = toml::to_string_pretty(&config)?;
        fs::write(config_path, toml_string)?;
        Ok(())
    }
}
