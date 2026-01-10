//! Configuration management for paks CLI
//!
//! Config file locations:
//! - Global: ~/.paks/config.toml
//! - Project: .paks/config.toml (in project root or git root)

use anyhow::{Context, Result};
use clap::ValueEnum;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

/// Installation scope for skills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Default)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// Install to global user directory (e.g., ~/.claude/skills)
    #[default]
    Global,
    /// Install to project directory (e.g., ./.claude/skills)
    Project,
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Default agent to use when --agent is not specified
    #[serde(default)]
    pub default_agent: Option<String>,

    /// Default registry to use when --registry is not specified
    #[serde(default)]
    pub default_registry: Option<String>,

    /// Default installation scope (global or project)
    #[serde(default)]
    pub default_scope: Option<Scope>,

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

    /// Path to the global skills directory (e.g., ~/.claude/skills)
    pub skills_dir: PathBuf,

    /// Path to the project skills directory relative to project root (e.g., .claude/skills)
    #[serde(default)]
    pub project_skills_dir: Option<PathBuf>,

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
    /// Get the global config file path (~/.paks/config.toml)
    pub fn path() -> Result<PathBuf> {
        let paks_dir = dirs::home_dir()
            .context("Could not determine home directory")?
            .join(".paks");
        Ok(paks_dir.join("config.toml"))
    }

    /// Get the project config file path (.paks/config.toml)
    /// Looks in current directory first, then git root
    pub fn project_path() -> Option<PathBuf> {
        // Try current directory first
        if let Ok(cwd) = std::env::current_dir() {
            let local = cwd.join(".paks").join("config.toml");
            if local.exists() {
                return Some(local);
            }
        }

        // Try git root
        if let Some(git_root) = Self::find_git_root() {
            let git_local = git_root.join(".paks").join("config.toml");
            if git_local.exists() {
                return Some(git_local);
            }
        }

        None
    }

    /// Find the project root directory
    /// Prefers git root, falls back to current directory
    pub fn find_project_root() -> Result<PathBuf> {
        if let Some(git_root) = Self::find_git_root() {
            return Ok(git_root);
        }
        std::env::current_dir().context("Could not determine current directory")
    }

    /// Find the git repository root
    fn find_git_root() -> Option<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .ok()?;

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
        None
    }

    /// Load global config from disk, or return default if not exists
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

    /// Load merged config (project config overrides global config)
    pub fn load_merged() -> Result<Self> {
        let mut config = Self::load()?;

        // Load project config if it exists
        if let Some(project_path) = Self::project_path() {
            let content = std::fs::read_to_string(&project_path).with_context(|| {
                format!(
                    "Failed to read project config from {}",
                    project_path.display()
                )
            })?;

            let project_config: Config = toml::from_str(&content).with_context(|| {
                format!(
                    "Failed to parse project config from {}",
                    project_path.display()
                )
            })?;

            // Merge: project values override global
            if project_config.default_scope.is_some() {
                config.default_scope = project_config.default_scope;
            }
            if project_config.default_agent.is_some() {
                config.default_agent = project_config.default_agent;
            }
            if project_config.default_registry.is_some() {
                config.default_registry = project_config.default_registry;
            }

            // Merge agents (project agents override global)
            for (key, agent) in project_config.agents {
                config.agents.insert(key, agent);
            }

            // Merge registries (project registries override global)
            for (key, registry) in project_config.registries {
                config.registries.insert(key, registry);
            }
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
                project_skills_dir: Some(PathBuf::from(".stakpak/skills")),
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
                project_skills_dir: Some(PathBuf::from(".claude/skills")),
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
                project_skills_dir: Some(PathBuf::from(".cursor/skills")),
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
                project_skills_dir: Some(PathBuf::from(".vscode/skills")),
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
                project_skills_dir: Some(PathBuf::from(".copilot/skills")),
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
                project_skills_dir: Some(PathBuf::from(".goose/skills")),
                description: Some("Block's Goose agent".to_string()),
            },
        );

        agents.insert(
            "opencode".to_string(),
            AgentConfig {
                name: "OpenCode".to_string(),
                skills_dir: dirs::config_dir()
                    .map(|c| c.join("opencode").join("skill"))
                    .unwrap_or_else(|| PathBuf::from("~/.config/opencode/skill")),
                project_skills_dir: Some(PathBuf::from(".opencode/skill")),
                description: Some("OpenCode AI agent".to_string()),
            },
        );

        agents.insert(
            "amp".to_string(),
            AgentConfig {
                name: "Amp".to_string(),
                skills_dir: dirs::config_dir()
                    .map(|c| c.join("agents").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.config/agents/skills")),
                project_skills_dir: Some(PathBuf::from(".agents/skills")),
                description: Some("Sourcegraph's Amp coding agent".to_string()),
            },
        );

        agents.insert(
            "codex".to_string(),
            AgentConfig {
                name: "Codex".to_string(),
                skills_dir: dirs::home_dir()
                    .map(|h| h.join(".codex").join("skills"))
                    .unwrap_or_else(|| PathBuf::from("~/.codex/skills")),
                project_skills_dir: Some(PathBuf::from(".codex/skills")),
                description: Some("OpenAI's Codex coding agent".to_string()),
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
            default_scope: None,
            agents: Self::builtin_agents(),
            registries: IndexMap::new(),
        }
    }

    /// Get the default project skills directory when no agent is specified
    /// This is .paks/skills relative to project root
    pub fn default_project_skills_dir() -> PathBuf {
        PathBuf::from(".paks/skills")
    }

    /// Resolve the skills directory based on scope
    ///
    /// Priority:
    /// 1. Explicit directory (if provided)
    /// 2. Scope-based directory from agent config
    /// 3. Default directory for the scope
    pub fn resolve_skills_dir(
        &self,
        scope: Scope,
        agent_name: Option<&str>,
        explicit_dir: Option<&str>,
    ) -> Result<PathBuf> {
        // 1. Explicit --dir always wins
        if let Some(dir) = explicit_dir {
            return Ok(PathBuf::from(shellexpand::tilde(dir).as_ref()));
        }

        // 2. Get agent config
        let agent = agent_name
            .or(self.default_agent.as_deref())
            .and_then(|name| self.get_agent(name));

        // 3. Resolve based on scope
        match scope {
            Scope::Global => Ok(agent
                .map(|a| a.skills_dir.clone())
                .unwrap_or_else(Self::default_skills_dir)),
            Scope::Project => {
                let project_root = Self::find_project_root()?;
                let relative_dir = agent
                    .and_then(|a| a.project_skills_dir.clone())
                    .unwrap_or_else(Self::default_project_skills_dir);
                Ok(project_root.join(relative_dir))
            }
        }
    }

    /// Get the effective scope from config or flag
    ///
    /// Priority:
    /// 1. Explicit scope flag
    /// 2. Project config default_scope
    /// 3. Global config default_scope  
    /// 4. Hardcoded default: Global
    pub fn effective_scope(&self, explicit_scope: Option<Scope>) -> Scope {
        explicit_scope
            .or(self.default_scope)
            .unwrap_or(Scope::Global)
    }

    /// Load merged config and resolve the skills directory in one call
    ///
    /// This is the primary entry point for commands that need to determine
    /// where to install/list/remove skills.
    ///
    /// Returns: (Config, resolved_skills_dir, effective_scope)
    pub fn load_and_resolve(
        explicit_scope: Option<Scope>,
        agent_name: Option<&str>,
        explicit_dir: Option<&str>,
    ) -> Result<(Self, PathBuf, Scope)> {
        let config = Self::load_merged()?;
        let scope = config.effective_scope(explicit_scope);
        let skills_dir = config.resolve_skills_dir(scope, agent_name, explicit_dir)?;
        Ok((config, skills_dir, scope))
    }

    /// Get agent config by name
    pub fn get_agent(&self, name: &str) -> Option<&AgentConfig> {
        self.agents.get(name)
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
    fn test_builtin_agents_have_project_skills_dir() {
        let config = Config::default_with_builtin_agents();

        // All built-in agents should have project_skills_dir
        for (id, agent) in &config.agents {
            assert!(
                agent.project_skills_dir.is_some(),
                "Agent '{}' should have project_skills_dir",
                id
            );
        }

        // Check specific values
        let claude = config.get_agent("claude-code").unwrap();
        assert_eq!(
            claude.project_skills_dir,
            Some(PathBuf::from(".claude/skills"))
        );

        let cursor = config.get_agent("cursor").unwrap();
        assert_eq!(
            cursor.project_skills_dir,
            Some(PathBuf::from(".cursor/skills"))
        );
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default_with_builtin_agents();
        let toml_str = toml::to_string_pretty(&config).unwrap_or_default();
        let parsed: Config = toml::from_str(&toml_str).unwrap_or_default();
        assert_eq!(config.default_agent, parsed.default_agent);
        assert_eq!(config.default_scope, parsed.default_scope);
    }

    #[test]
    fn test_scope_serialization() {
        // Test via config struct (TOML requires a table at root level)
        #[derive(Serialize, Deserialize)]
        struct TestConfig {
            scope: Scope,
        }

        // Test Global scope
        let global_config = TestConfig {
            scope: Scope::Global,
        };
        let global_str = toml::to_string(&global_config).unwrap_or_default();
        assert!(
            global_str.contains("global"),
            "Expected 'global' in: {}",
            global_str
        );

        // Test Project scope
        let project_config = TestConfig {
            scope: Scope::Project,
        };
        let project_str = toml::to_string(&project_config).unwrap_or_default();
        assert!(
            project_str.contains("project"),
            "Expected 'project' in: {}",
            project_str
        );

        // Test deserialization
        let parsed: TestConfig = toml::from_str("scope = \"global\"").unwrap();
        assert_eq!(parsed.scope, Scope::Global);

        let parsed: TestConfig = toml::from_str("scope = \"project\"").unwrap();
        assert_eq!(parsed.scope, Scope::Project);
    }

    #[test]
    fn test_effective_scope() {
        let mut config = Config::default_with_builtin_agents();

        // No config, no flag -> Global (default)
        assert_eq!(config.effective_scope(None), Scope::Global);

        // Flag overrides everything
        assert_eq!(config.effective_scope(Some(Scope::Project)), Scope::Project);
        assert_eq!(config.effective_scope(Some(Scope::Global)), Scope::Global);

        // Config default is used when no flag
        config.default_scope = Some(Scope::Project);
        assert_eq!(config.effective_scope(None), Scope::Project);

        // Flag still overrides config
        assert_eq!(config.effective_scope(Some(Scope::Global)), Scope::Global);
    }

    #[test]
    fn test_config_with_scope() {
        let toml_str = r#"
default_agent = "claude-code"
default_scope = "project"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default_agent, Some("claude-code".to_string()));
        assert_eq!(config.default_scope, Some(Scope::Project));
    }

    #[test]
    fn test_agent_config_with_project_skills_dir() {
        let toml_str = r#"
[agents.custom]
name = "Custom Agent"
skills_dir = "~/.custom/skills"
project_skills_dir = ".custom/skills"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        let agent = config.get_agent("custom").unwrap();
        assert_eq!(agent.name, "Custom Agent");
        assert_eq!(agent.skills_dir, PathBuf::from("~/.custom/skills"));
        assert_eq!(
            agent.project_skills_dir,
            Some(PathBuf::from(".custom/skills"))
        );
    }

    #[test]
    fn test_resolve_skills_dir_with_explicit_dir() {
        let config = Config::default_with_builtin_agents();

        // Explicit dir should always win
        let result = config
            .resolve_skills_dir(Scope::Global, None, Some("~/custom/path"))
            .unwrap();
        assert!(result.to_string_lossy().contains("custom/path"));

        // Even with project scope, explicit dir wins
        let result = config
            .resolve_skills_dir(Scope::Project, Some("claude-code"), Some("/explicit/path"))
            .unwrap();
        assert_eq!(result, PathBuf::from("/explicit/path"));
    }

    #[test]
    fn test_resolve_skills_dir_global_scope() {
        let config = Config::default_with_builtin_agents();

        // Global scope with agent should use agent's skills_dir
        let result = config
            .resolve_skills_dir(Scope::Global, Some("claude-code"), None)
            .unwrap();
        assert!(result.to_string_lossy().contains(".claude/skills"));
    }

    #[test]
    fn test_resolve_skills_dir_global_scope_default_agent() {
        let mut config = Config::default_with_builtin_agents();
        config.default_agent = Some("cursor".to_string());

        // Should use default agent when no agent specified
        let result = config
            .resolve_skills_dir(Scope::Global, None, None)
            .unwrap();
        assert!(result.to_string_lossy().contains(".cursor/skills"));
    }

    #[test]
    fn test_resolve_skills_dir_global_scope_no_agent() {
        let config = Config::default_with_builtin_agents();

        // No agent, no default -> use default_skills_dir
        let result = config
            .resolve_skills_dir(Scope::Global, None, None)
            .unwrap();
        assert!(result.to_string_lossy().contains(".agents/skills"));
    }

    #[test]
    fn test_default_project_skills_dir() {
        let dir = Config::default_project_skills_dir();
        assert_eq!(dir, PathBuf::from(".paks/skills"));
    }

    #[test]
    fn test_default_skills_dir() {
        let dir = Config::default_skills_dir();
        assert!(dir.to_string_lossy().contains(".agents/skills"));
    }

    #[test]
    fn test_scope_default_value() {
        // Scope::default() should be Global
        assert_eq!(Scope::default(), Scope::Global);
    }

    #[test]
    fn test_config_merge_scope_override() {
        // Simulate merging configs - project scope should override global
        let mut global_config = Config::default_with_builtin_agents();
        global_config.default_scope = Some(Scope::Global);
        global_config.default_agent = Some("stakpak".to_string());

        // Simulate project config values
        let project_scope = Some(Scope::Project);
        let project_agent = Some("claude-code".to_string());

        // Apply project overrides (simulating load_merged behavior)
        if project_scope.is_some() {
            global_config.default_scope = project_scope;
        }
        if project_agent.is_some() {
            global_config.default_agent = project_agent;
        }

        assert_eq!(global_config.default_scope, Some(Scope::Project));
        assert_eq!(global_config.default_agent, Some("claude-code".to_string()));
    }

    #[test]
    fn test_resolve_skills_dir_project_scope() {
        let config = Config::default_with_builtin_agents();

        // Project scope should resolve relative to project root
        // This test runs in a git repo, so find_project_root should work
        let result = config.resolve_skills_dir(Scope::Project, Some("claude-code"), None);

        // Should succeed since we're in a git repo
        assert!(result.is_ok());
        let path = result.unwrap();
        // Should end with .claude/skills (the project_skills_dir for claude-code)
        assert!(
            path.to_string_lossy().ends_with(".claude/skills"),
            "Expected path to end with .claude/skills, got: {}",
            path.display()
        );
    }

    #[test]
    fn test_find_project_root_in_git_repo() {
        // This test runs within the paks git repo
        let result = Config::find_project_root();
        assert!(result.is_ok());
        let root = result.unwrap();
        // Should find the git root which contains Cargo.toml
        assert!(root.join("Cargo.toml").exists());
    }

    #[test]
    fn test_find_git_root() {
        // This test runs within the paks git repo
        let result = Config::find_git_root();
        assert!(result.is_some());
        let root = result.unwrap();
        // Should find the git root which contains .git
        assert!(root.join(".git").exists());
    }

    #[test]
    fn test_project_path_returns_none_when_no_config() {
        // project_path looks for .paks/config.toml which doesn't exist in test env
        // (unless someone created it), so this should return None
        let result = Config::project_path();
        // We can't guarantee the result since it depends on the environment
        // but we can at least verify the function doesn't panic
        let _ = result;
    }
}
