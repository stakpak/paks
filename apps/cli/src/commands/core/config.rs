//! Configuration management for paks CLI
//!
//! Config file location: ~/.paks/config.toml

use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Default agent to use when --agent is not specified
    #[serde(default)]
    pub default_agent: Option<String>,

    /// Default registry to use when --registry is not specified
    #[serde(default)]
    pub default_registry: Option<String>,

    /// Configured agents with their skills directories (ordered, stakpak first)
    #[serde(default)]
    pub agents: IndexMap<String, AgentConfig>,

    /// Configured registries
    #[serde(default)]
    pub registries: IndexMap<String, RegistryConfig>,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Display name for the agent
    pub name: String,

    /// Path to the skills directory
    pub skills_dir: PathBuf,

    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
}

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Registry URL
    pub url: String,

    /// Optional API token (stored separately in keychain ideally)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl Config {
    /// Get the config file path
    pub fn path() -> Result<PathBuf> {
        let paks_dir = dirs::home_dir()
            .context("Could not determine home directory")?
            .join(".paks");
        Ok(paks_dir.join("config.toml"))
    }

    /// Load config from disk, or return default if not exists
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default_with_builtin_agents());
        }

        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {}", path.display()))?;

        let mut config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config from {}", path.display()))?;

        // Merge built-in agents (user config takes precedence)
        let builtin = Self::builtin_agents();
        for (key, agent) in builtin {
            config.agents.entry(key).or_insert(agent);
        }

        Ok(config)
    }

    /// Save config to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config to {}", path.display()))?;

        Ok(())
    }

    /// Get built-in agent configurations (stakpak first, then alphabetical)
    fn builtin_agents() -> IndexMap<String, AgentConfig> {
        let mut agents = IndexMap::new();

        // Stakpak agent - MUST be first
        agents.insert(
            "stakpak".to_string(),
            AgentConfig {
                name: "Stakpak".to_string(),
                skills_dir: dirs::home_dir()
                    .map(|h| h.join(".stakpak").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.stakpak/skills")),
                description: Some("Stakpak agent".to_string()),
            },
        );

        agents.insert(
            "claude-code".to_string(),
            AgentConfig {
                name: "Claude Code".to_string(),
                skills_dir: dirs::home_dir()
                    .map(|h| h.join(".claude").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.claude/skills")),
                description: Some("Anthropic's Claude Code agent".to_string()),
            },
        );

        agents.insert(
            "cursor".to_string(),
            AgentConfig {
                name: "Cursor".to_string(),
                skills_dir: dirs::home_dir()
                    .map(|h| h.join(".cursor").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.cursor/skills")),
                description: Some("Cursor AI editor".to_string()),
            },
        );

        agents.insert(
            "vscode".to_string(),
            AgentConfig {
                name: "VS Code".to_string(),
                skills_dir: dirs::home_dir()
                    .map(|h| h.join(".vscode").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.vscode/skills")),
                description: Some("VS Code with GitHub Copilot".to_string()),
            },
        );

        agents.insert(
            "copilot".to_string(),
            AgentConfig {
                name: "GitHub Copilot".to_string(),
                skills_dir: dirs::home_dir()
                    .map(|h| h.join(".copilot").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.copilot/skills")),
                description: Some("GitHub Copilot CLI".to_string()),
            },
        );

        agents.insert(
            "goose".to_string(),
            AgentConfig {
                name: "Goose".to_string(),
                skills_dir: dirs::config_dir()
                    .map(|c| c.join("goose").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.config/goose/skills")),
                description: Some("Block's Goose agent".to_string()),
            },
        );

        agents.insert(
            "opencode".to_string(),
            AgentConfig {
                name: "OpenCode".to_string(),
                skills_dir: dirs::config_dir()
                    .map(|c| c.join("opencode").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.config/opencode/skills")),
                description: Some("OpenCode AI agent".to_string()),
            },
        );

        agents
    }

    /// Get the default skills directory when no agent is specified
    /// This is ~/.agents/skills
    pub fn default_skills_dir() -> PathBuf {
        dirs::home_dir()
            .map(|h| h.join(".agents").join("skills"))
            .unwrap_or_else(|| PathBuf::from("~/.agents/skills"))
    }

    /// Create default config with built-in agents (no default agent until user sets one)
    fn default_with_builtin_agents() -> Self {
        Self {
            default_agent: None,
            default_registry: None,
            agents: Self::builtin_agents(),
            registries: IndexMap::new(),
        }
    }

    /// Get agent config by name
    pub fn get_agent(&self, name: &str) -> Option<&AgentConfig> {
        self.agents.get(name)
    }

    /// Get the default agent config
    pub fn get_default_agent(&self) -> Option<&AgentConfig> {
        self.default_agent
            .as_ref()
            .and_then(|name| self.agents.get(name))
    }

    /// Get the auth token for the default registry
    pub fn get_auth_token(&self) -> Option<&str> {
        // First check default registry
        if let Some(default_reg) = &self.default_registry
            && let Some(reg) = self.registries.get(default_reg)
            && reg.token.is_some()
        {
            return reg.token.as_deref();
        }
        // Fall back to "stakpak" registry
        self.registries
            .get("stakpak")
            .and_then(|r| r.token.as_deref())
    }

    /// Set the auth token for the default registry
    pub fn set_auth_token(&mut self, token: String) {
        let registry_name = self
            .default_registry
            .clone()
            .unwrap_or_else(|| "stakpak".to_string());

        if let Some(reg) = self.registries.get_mut(&registry_name) {
            reg.token = Some(token);
        } else {
            self.registries.insert(
                registry_name,
                RegistryConfig {
                    url: "https://apiv2.stakpak.dev".to_string(),
                    token: Some(token),
                },
            );
        }
    }

    /// Clear the auth token for the default registry
    pub fn clear_auth_token(&mut self) {
        let registry_name = self
            .default_registry
            .clone()
            .unwrap_or_else(|| "stakpak".to_string());

        if let Some(reg) = self.registries.get_mut(&registry_name) {
            reg.token = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_builtin_agents() {
        let config = Config::default_with_builtin_agents();
        assert!(config.agents.contains_key("claude-code"));
        assert!(config.agents.contains_key("cursor"));
        assert!(config.agents.contains_key("vscode"));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default_with_builtin_agents();
        let toml_str = toml::to_string_pretty(&config).unwrap_or_default();
        let parsed: Config = toml::from_str(&toml_str).unwrap_or_default();
        assert_eq!(config.default_agent, parsed.default_agent);
    }
}
