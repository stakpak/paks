//! Skill manifest and structure handling
//!
//! Everything lives in SKILL.md with YAML frontmatter.
//! Paks extends the Agent Skills spec frontmatter with package management fields.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// SKILL.md frontmatter - combines Agent Skills spec with paks package fields
///
/// Required fields (Agent Skills spec):
/// - name: Skill identifier (1-64 chars, lowercase + hyphens)
/// - description: What the skill does (1-1024 chars)
///
/// Optional fields (Agent Skills spec):
/// - license: License name or reference
/// - compatibility: Environment requirements
/// - metadata: Arbitrary key-value pairs (includes version for paks)
/// - allowed-tools: Pre-approved tools (experimental)
///
/// Paks extensions (for package management):
/// - metadata.version: Semantic version for publishing (inside metadata)
/// - authors: List of authors
/// - repository: Source repository URL
/// - homepage: Project homepage
/// - keywords: Search keywords
/// - categories: Skill categories
/// - dependencies: Other skills this depends on
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    // === Agent Skills spec required fields ===
    /// Skill name (required, 1-64 chars, lowercase + hyphens)
    pub name: String,

    /// Description of what the skill does (required, 1-1024 chars)
    pub description: String,

    // === Agent Skills spec optional fields ===
    /// License
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// Compatibility notes (max 500 chars)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,

    /// Additional metadata (Agent Skills spec)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Pre-approved tools (experimental, Agent Skills spec)
    #[serde(
        default,
        rename = "allowed-tools",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_tools: Option<String>,

    // === Paks package management extensions ===
    /// Authors (paks extension)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,

    /// Repository URL (paks extension)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,

    /// Homepage URL (paks extension)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    /// Keywords for search (paks extension)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,

    /// Categories (paks extension)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,

    /// Dependencies on other skills (paks extension)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<SkillDependency>,
}

/// Skill dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDependency {
    /// Dependency skill name
    pub name: String,

    /// Version requirement (semver range)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Git repository URL (alternative to registry)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<String>,

    /// Git ref (branch/tag/commit)
    #[serde(default, rename = "ref", skip_serializing_if = "Option::is_none")]
    pub git_ref: Option<String>,

    /// Local path (for development)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl SkillFrontmatter {
    /// Validate the frontmatter according to Agent Skills spec
    pub fn validate(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Name validation
        if self.name.is_empty() || self.name.len() > 64 {
            bail!("name must be 1-64 characters");
        }

        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
        {
            bail!("name must contain only lowercase letters, numbers, and hyphens");
        }

        if self.name.starts_with('-') || self.name.ends_with('-') {
            bail!("name must not start or end with a hyphen");
        }

        if self.name.contains("--") {
            bail!("name must not contain consecutive hyphens");
        }

        // Description validation
        if self.description.is_empty() || self.description.len() > 1024 {
            bail!("description must be 1-1024 characters");
        }

        if self.description.len() < 20 {
            warnings.push("description is very short; consider adding more detail".to_string());
        }

        // Compatibility validation
        if let Some(compat) = &self.compatibility
            && compat.len() > 500
        {
            bail!("compatibility must be at most 500 characters");
        }

        Ok(warnings)
    }
}

/// Represents a complete skill on disk
#[derive(Debug)]
pub struct Skill {
    /// Root directory of the skill
    pub path: PathBuf,

    /// SKILL.md frontmatter (contains all metadata)
    pub frontmatter: SkillFrontmatter,

    /// SKILL.md body content (instructions)
    pub instructions: String,
}

impl Skill {
    /// Load a skill from a directory
    pub fn load(skill_dir: &Path) -> Result<Self> {
        let skill_md_path = skill_dir.join("SKILL.md");
        if !skill_md_path.exists() {
            bail!("No SKILL.md found in {}", skill_dir.display());
        }

        let content = std::fs::read_to_string(&skill_md_path)
            .with_context(|| format!("Failed to read {}", skill_md_path.display()))?;

        let (frontmatter, instructions) = parse_skill_md(&content)?;

        Ok(Self {
            path: skill_dir.to_path_buf(),
            frontmatter,
            instructions,
        })
    }

    /// Save skill to disk
    pub fn save(&self) -> Result<()> {
        let skill_md_path = self.path.join("SKILL.md");
        let content = generate_skill_md(&self.frontmatter, &self.instructions)?;

        std::fs::write(&skill_md_path, content)
            .with_context(|| format!("Failed to write {}", skill_md_path.display()))?;

        Ok(())
    }

    /// Create a new skill with minimal info
    pub fn new(path: PathBuf, name: &str, description: &str) -> Self {
        Self {
            path,
            frontmatter: SkillFrontmatter {
                name: name.to_string(),
                description: description.to_string(),
                license: Some("MIT".to_string()),
                compatibility: None,
                metadata: Some(HashMap::from([("version".to_string(), "0.1.0".to_string())])),
                allowed_tools: None,
                authors: Vec::new(),
                repository: None,
                homepage: None,
                keywords: Vec::new(),
                categories: Vec::new(),
                dependencies: Vec::new(),
            },
            instructions: format!(
                "# {}\n\n## When to use this skill\n\nDescribe when this skill should be activated.\n\n## Instructions\n\nAdd your instructions here.\n",
                name
            ),
        }
    }

    /// Check if skill has scripts directory
    pub fn has_scripts(&self) -> bool {
        self.path.join("scripts").is_dir()
    }

    /// Check if skill has references directory
    pub fn has_references(&self) -> bool {
        self.path.join("references").is_dir()
    }

    /// Check if skill has assets directory
    pub fn has_assets(&self) -> bool {
        self.path.join("assets").is_dir()
    }

    /// Get the skill name
    pub fn name(&self) -> &str {
        &self.frontmatter.name
    }

    /// Get the skill version from metadata (defaults to "0.1.0" if not set)
    pub fn version(&self) -> &str {
        self.frontmatter
            .metadata
            .as_ref()
            .and_then(|m| m.get("version"))
            .map(|s| s.as_str())
            .unwrap_or("0.1.0")
    }

    /// Get the skill version as Option (for publish checks)
    pub fn version_opt(&self) -> Option<&str> {
        self.frontmatter
            .metadata
            .as_ref()
            .and_then(|m| m.get("version"))
            .map(|s| s.as_str())
    }
}

/// Parse SKILL.md content into frontmatter and body
fn parse_skill_md(content: &str) -> Result<(SkillFrontmatter, String)> {
    let content = content.trim();

    if !content.starts_with("---") {
        bail!("SKILL.md must start with YAML frontmatter (---)");
    }

    let rest = &content[3..];
    let end_marker = rest
        .find("\n---")
        .context("SKILL.md frontmatter not properly closed (missing ---)")?;

    let frontmatter_str = &rest[..end_marker].trim();
    let body = rest[end_marker + 4..].trim();

    // Parse YAML frontmatter
    let frontmatter: SkillFrontmatter = serde_yaml_ng::from_str(frontmatter_str)
        .context("Failed to parse SKILL.md frontmatter as YAML")?;

    Ok((frontmatter, body.to_string()))
}

/// Generate SKILL.md content from frontmatter and body
pub fn generate_skill_md(frontmatter: &SkillFrontmatter, body: &str) -> Result<String> {
    let yaml = serde_yaml_ng::to_string(frontmatter).context("Failed to serialize frontmatter")?;

    Ok(format!("---\n{}---\n\n{}", yaml, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontmatter_validation() {
        let valid = SkillFrontmatter {
            name: "my-skill".to_string(),
            description: "A skill that does something useful for users".to_string(),
            license: None,
            compatibility: None,
            metadata: None,
            allowed_tools: None,
            authors: Vec::new(),
            repository: None,
            homepage: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            dependencies: Vec::new(),
        };
        assert!(valid.validate().is_ok());

        let invalid_name = SkillFrontmatter {
            name: "My-Skill".to_string(), // uppercase not allowed
            description: "A skill".to_string(),
            license: None,
            compatibility: None,
            metadata: None,
            allowed_tools: None,
            authors: Vec::new(),
            repository: None,
            homepage: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            dependencies: Vec::new(),
        };
        assert!(invalid_name.validate().is_err());
    }

    #[test]
    fn test_parse_skill_md() {
        let content = r#"---
name: test-skill
description: A test skill for unit testing
---

# Test Skill

Instructions go here.
"#;
        let (fm, body) = parse_skill_md(content).unwrap();
        assert_eq!(fm.name, "test-skill");
        assert!(body.contains("# Test Skill"));
    }
}
