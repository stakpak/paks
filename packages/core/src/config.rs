use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub features: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "paks-project".to_string(),
            version: "0.1.0".to_string(),
            features: vec![],
        }
    }
}

impl Config {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.name, "paks-project");
        assert_eq!(config.version, "0.1.0");
    }

    #[test]
    fn test_new_config() {
        let config = Config::new("my-app");
        assert_eq!(config.name, "my-app");
    }
}
