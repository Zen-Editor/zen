use serde::{Deserialize, Serialize};

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
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(current_dir) = std::env::current_dir() {
                let config_path = current_dir.join("config.json");
                if config_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(config_path) {
                        if let Ok(config) = serde_json::from_str(&content) {
                            return config;
                        }
                    }
                }
            }
        }

        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let json = serde_json::to_string_pretty(self)?;
            std::fs::write("config.json", json)?;
        }
        Ok(())
    }
}