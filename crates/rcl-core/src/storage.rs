use std::path::Path;
use serde::{de::DeserializeOwned, Serialize};
use crate::error::Result;

pub struct StorageManager;

impl StorageManager {
    pub fn save_json<T: Serialize>(path: impl AsRef<Path>, data: &T) -> Result<()> {
        let content = serde_json::to_string_pretty(data)?;
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn load_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
        let content = std::fs::read_to_string(path)?;
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }

    pub fn save_toml<T: Serialize>(path: impl AsRef<Path>, data: &T) -> Result<()> {
        let content = toml::to_string_pretty(data)?;
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn load_toml<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
        let content = std::fs::read_to_string(path)?;
        let data = toml::from_str(&content)?;
        Ok(data)
    }
}
