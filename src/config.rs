use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
    pub family: FamilyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// GitHub token for Copilot API access
    pub github_token: Option<String>,
    /// GitHub Copilot API endpoint (default: https://api.githubcopilot.com)
    pub copilot_api_url: String,
    /// Model to use (default: gpt-4o)
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyConfig {
    pub adults_count: u8,
    pub kids_count: u8,
    pub infant_count: u8,
    pub cuisine: String,
    pub region: String,
    pub protein_preferences: Vec<String>,
}

impl Default for AiConfig {
    fn default() -> Self {
        AiConfig {
            github_token: None,
            copilot_api_url: "https://api.githubcopilot.com".to_string(),
            model: "gpt-4o".to_string(),
        }
    }
}

impl Default for FamilyConfig {
    fn default() -> Self {
        FamilyConfig {
            adults_count: 4,
            kids_count: 2,
            infant_count: 1,
            cuisine: "South Indian".to_string(),
            region: "Chennai".to_string(),
            protein_preferences: vec![
                "egg".to_string(),
                "fish".to_string(),
                "chicken".to_string(),
                "mutton".to_string(),
            ],
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ai: AiConfig::default(),
            family: FamilyConfig::default(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("food-planner")
            .join("config.toml")
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        println!("Configuration saved to: {}", path.display());
        Ok(())
    }

    pub fn get_token(&self) -> Option<String> {
        // First check config file, then environment variable
        self.ai
            .github_token
            .clone()
            .or_else(|| std::env::var("GITHUB_TOKEN").ok())
    }
}
