use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub git: GitConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub default_mode: String,
    pub check_interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub enable_worktree: bool,
    pub default_branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                default_mode: "claude".to_string(),
                check_interval: 5,
            },
            git: GitConfig {
                enable_worktree: false,
                default_branch: "main".to_string(),
            },
            storage: StorageConfig {
                database_path: default_database_path().to_string_lossy().to_string(),
            },
        }
    }
}

impl Config {
    pub async fn load() -> Result<Self> {
        let config_path = config_file_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }

    pub async fn save(&self) -> Result<()> {
        let config_path = config_file_path();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content).await?;
        Ok(())
    }

    pub fn database_path(&self) -> PathBuf {
        // Expand ~ to home directory
        if self.storage.database_path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                let path = self.storage.database_path.strip_prefix("~").unwrap();
                return home.join(path.trim_start_matches('/'));
            }
        }
        PathBuf::from(&self.storage.database_path)
    }
}

pub fn config_file_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "claude-scheduler") {
        proj_dirs.config_dir().join("config.toml")
    } else {
        PathBuf::from(".claude-scheduler/config.toml")
    }
}

pub fn default_database_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "claude-scheduler") {
        proj_dirs.data_dir().join("db.sqlite")
    } else {
        PathBuf::from(".claude-scheduler/db.sqlite")
    }
} 