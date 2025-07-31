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
            let current_dir = match std::env::current_dir() {
                Ok(dir) => dir,
                Err(_) => return Self::default(),
            };

            let config_path = current_dir.join("config.json");
            if !config_path.exists() {
                return Self::default();
            }

            let content = match std::fs::read_to_string(config_path) {
                Ok(content) => content,
                Err(_) => return Self::default(),
            };

            serde_json::from_str(&content).unwrap_or_else(|_| Self::default());
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