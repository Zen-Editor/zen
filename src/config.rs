use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub default_theme: String,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            default_theme: "Dark".to_string(),
        }
    }
}

impl EditorConfig {
    pub fn load() -> Self {
        let config_path = "config.json";
        if Path::new(config_path).exists() {
            if let Ok(json) = std::fs::read_to_string(config_path) {
                if let Ok(config) = serde_json::from_str(&json) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("config.json", json)?;
        Ok(())
    }
}