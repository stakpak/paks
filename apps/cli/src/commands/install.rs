//! Install command - install a skill to an agent's skills directory

use super::core::config::Config;
use super::core::skill::Skill;
use anyhow::{Context, Result, bail};
use paks_api::{ApiError, PaksClient};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct InstallArgs {
    pub source: String,
    pub agent: Option<String>,
    pub dir: Option<String>,
    pub force: bool,
}

/// Parsed skill reference from user input
#[derive(Debug)]
struct SkillRef {
    /// Account/owner name (e.g., "stakpak")
    account: String,
    /// Skill name (e.g., "kubernetes-deploy")
    name: String,
    /// Optional version (e.g., "1.2.3")
    version: Option<String>,
}

/// Source type for skill installation
#[derive(Debug)]
enum SourceType {
    /// Registry reference: account/skill[@version]
    Registry(SkillRef),
    /// Git URL (HTTPS or SSH)
    Git {
        url: String,
        git_ref: Option<String>,
        path: Option<String>,
    },
    /// Local filesystem path
    Local(PathBuf),
}

impl SkillRef {
    /// Parse a skill reference from input string
    /// Format: account/skill[@version]
    fn parse(input: &str) -> Result<Self> {
        let (identifier, version) = if let Some(at_pos) = input.rfind('@') {
            let id = &input[..at_pos];
            let ver = &input[at_pos + 1..];
            if ver.is_empty() {
                bail!("Version cannot be empty after @");
            }
            (id, Some(ver.to_string()))
        } else {
            (input, None)
        };

        let parts: Vec<&str> = identifier.split('/').collect();
        if parts.len() != 2 {
            bail!(
                "Invalid skill reference '{}'. Expected format: account/skill[@version]",
                input
            );
        }

        let account = parts[0].to_string();
        let name = parts[1].to_string();

        // Validate account name (lowercase alphanumeric + hyphens, 1-39 chars)
        if account.is_empty() || account.len() > 39 {
            bail!("Account name must be 1-39 characters");
        }
        if !account
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            bail!("Account name must contain only lowercase letters, numbers, and hyphens");
        }

        // Validate skill name (lowercase alphanumeric + hyphens, 1-64 chars)
        if name.is_empty() || name.len() > 64 {
            bail!("Skill name must be 1-64 characters");
        }
        if !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            bail!("Skill name must contain only lowercase letters, numbers, and hyphens");
        }

        Ok(Self {
            account,
            name,
            version,
        })
    }

    /// Get the URI for API calls (account/name[@version])
    fn to_uri(&self) -> String {
        match &self.version {
            Some(v) => format!("{}/{}@{}", self.account, self.name, v),
            None => format!("{}/{}", self.account, self.name),
        }
    }
}

/// Detect the source type from user input
fn detect_source_type(source: &str) -> SourceType {
    // Check for git URLs first
    if source.starts_with("https://")
        || source.starts_with("http://")
        || source.starts_with("git@")
        || source.starts_with("ssh://")
    {
        // Parse git URL for ref and path fragments
        let (url, git_ref, path) = parse_git_url(source);
        return SourceType::Git { url, git_ref, path };
    }

    // Check for local paths
    if source.starts_with("./") || source.starts_with("../") || source.starts_with('/') {
        return SourceType::Local(PathBuf::from(source));
    }

    // Check if it looks like a Windows absolute path
    if source.len() >= 2 && source.chars().nth(1) == Some(':') {
        return SourceType::Local(PathBuf::from(source));
    }

    // Check if path exists locally (handles cases like "my-skill" in current dir)
    let path = PathBuf::from(source);
    if path.exists() && path.join("SKILL.md").exists() {
        return SourceType::Local(path);
    }

    // Default: treat as registry reference
    // Try to parse as account/skill[@version]
    if let Ok(skill_ref) = SkillRef::parse(source) {
        return SourceType::Registry(skill_ref);
    }

    // If parsing failed but contains a slash, still try as registry
    if source.contains('/') && !source.contains("://") {
        // This will fail later with a proper error message
        if let Ok(skill_ref) = SkillRef::parse(source) {
            return SourceType::Registry(skill_ref);
        }
    }

    // Fallback to local path
    SourceType::Local(PathBuf::from(source))
}

/// Parse git URL for ref and path fragments
/// Supports: url#ref=v1.0.0 and url#path=skills/my-skill
fn parse_git_url(url: &str) -> (String, Option<String>, Option<String>) {
    if let Some(hash_pos) = url.find('#') {
        let base_url = url[..hash_pos].to_string();
        let fragment = &url[hash_pos + 1..];

        let mut git_ref = None;
        let mut path = None;

        for part in fragment.split('&') {
            if let Some(value) = part.strip_prefix("ref=") {
                git_ref = Some(value.to_string());
            } else if let Some(value) = part.strip_prefix("tag=") {
                git_ref = Some(value.to_string());
            } else if let Some(value) = part.strip_prefix("branch=") {
                git_ref = Some(value.to_string());
            } else if let Some(value) = part.strip_prefix("path=") {
                path = Some(value.to_string());
            }
        }

        (base_url, git_ref, path)
    } else {
        (url.to_string(), None, None)
    }
}

pub async fn run(args: InstallArgs) -> Result<()> {
    // Determine install directory
    let install_dir = if let Some(dir) = &args.dir {
        PathBuf::from(shellexpand::tilde(dir).as_ref())
    } else {
        let config = Config::load()?;
        let agent_name = args.agent.as_ref().or(config.default_agent.as_ref());

        if let Some(name) = agent_name {
            config
                .get_agent(name)
                .map(|a| a.skills_dir.clone())
                .unwrap_or_else(Config::default_skills_dir)
        } else {
            // No agent specified and no default - use ~/.paks/skills
            Config::default_skills_dir()
        }
    };

    // Detect source type
    let source_type = detect_source_type(&args.source);

    match source_type {
        SourceType::Registry(skill_ref) => {
            install_from_registry(skill_ref, &install_dir, args.force).await
        }
        SourceType::Git { url, git_ref, path } => {
            install_from_git(
                &url,
                git_ref.as_deref(),
                path.as_deref(),
                &install_dir,
                args.force,
            )
            .await
        }
        SourceType::Local(path) => install_from_local(&path, &install_dir, args.force).await,
    }
}

/// Install a skill from the paks registry
async fn install_from_registry(skill_ref: SkillRef, install_dir: &Path, force: bool) -> Result<()> {
    println!("Installing {} from registry...", skill_ref.to_uri());

    // Create API client
    let client = PaksClient::builder()
        .base_url("https://apiv2.stakpak.dev")
        .build()
        .context("Failed to create API client")?;

    // Fetch install metadata from registry
    let uri = skill_ref.to_uri();
    let install_info = match client.get_pak_install(&uri).await {
        Ok(info) => info,
        Err(ApiError::NotFound(_)) => {
            bail!(
                "Skill '{}' not found in registry.\n\
                 Hint: Check the skill name or search with 'paks search {}'",
                uri,
                skill_ref.name
            );
        }
        Err(ApiError::Api { status: 403, .. }) => {
            bail!(
                "Access denied to skill '{}'.\n\
                 Hint: This may be a private skill. Try 'paks login' first.",
                uri
            );
        }
        Err(e) => {
            bail!("Failed to fetch skill info: {}", e);
        }
    };

    println!(
        "  Found: {}/{}@{}",
        install_info.pak.owner, install_info.pak.name, install_info.version.version
    );

    // Determine target directory (flat: owner--skill to avoid nesting)
    let target_dir = install_dir.join(format!(
        "{}--{}",
        install_info.pak.owner, install_info.pak.name
    ));

    // Check if already installed
    if target_dir.exists() {
        if !force {
            // Check installed version
            if let Ok(existing) = Skill::load(&target_dir) {
                let installed_version = existing.version();
                if installed_version == install_info.version.version {
                    println!(
                        "✓ Already installed: {}/{}@{}",
                        install_info.pak.owner, install_info.pak.name, installed_version
                    );
                    return Ok(());
                }
                println!(
                    "  Installed version: {} → {}",
                    installed_version, install_info.version.version
                );
            }
            bail!(
                "Skill already exists at {}.\n\
                 Use --force to reinstall.",
                target_dir.display()
            );
        }
        println!("  Removing existing installation...");
        std::fs::remove_dir_all(&target_dir)
            .with_context(|| format!("Failed to remove {}", target_dir.display()))?;
    }

    // Clone from git at the specific tag, installing to account/skill path
    install_from_git_to_target(
        &install_info.repository.clone_url,
        Some(&install_info.version.tag),
        if install_info.install.path == "." {
            None
        } else {
            Some(&install_info.install.path)
        },
        &target_dir,
        force,
    )
    .await?;

    println!(
        "✓ Installed {}/{}@{}",
        install_info.pak.owner, install_info.pak.name, install_info.version.version
    );
    println!("  Location: {}", target_dir.display());

    Ok(())
}

/// Install a skill from a git repository (standalone, not from registry)
async fn install_from_git(
    url: &str,
    git_ref: Option<&str>,
    subpath: Option<&str>,
    install_dir: &Path,
    force: bool,
) -> Result<()> {
    println!("Installing from git: {}", url);
    if let Some(r) = git_ref {
        println!("  Ref: {}", r);
    }
    if let Some(p) = subpath {
        println!("  Path: {}", p);
    }

    // Clone and get skill info
    let (source_path, temp_dir) = clone_git_repo(url, git_ref, subpath).await?;

    // Load skill to get metadata
    let skill = Skill::load(&source_path).context("Failed to load skill from repository")?;
    let skill_name = skill.name().to_string();

    // For standalone git installs, use just the skill name (no account prefix)
    let target_dir = install_dir.join(&skill_name);

    // Check if already installed
    if target_dir.exists() {
        if !force {
            bail!(
                "Skill '{}' already exists at {}.\n\
                 Use --force to reinstall.",
                skill_name,
                target_dir.display()
            );
        }
        println!("  Removing existing installation...");
        std::fs::remove_dir_all(&target_dir)
            .with_context(|| format!("Failed to remove {}", target_dir.display()))?;
    }

    // Copy to target
    copy_skill_to_target(&source_path, &target_dir)?;

    println!("✓ Installed {} from git", skill_name);
    println!("  Location: {}", target_dir.display());

    // temp_dir is dropped here, cleaning up the clone
    drop(temp_dir);
    Ok(())
}

/// Install a skill from git to a specific target directory (used by registry install)
async fn install_from_git_to_target(
    url: &str,
    git_ref: Option<&str>,
    subpath: Option<&str>,
    target_dir: &Path,
    force: bool,
) -> Result<()> {
    // Clone and get skill info
    let (source_path, temp_dir) = clone_git_repo(url, git_ref, subpath).await?;

    // Validate skill structure
    if !source_path.join("SKILL.md").exists() {
        bail!(
            "No SKILL.md found in {}.\n\
             This doesn't appear to be a valid skill.",
            source_path.display()
        );
    }

    // Check if already installed (should be handled by caller, but double-check)
    if target_dir.exists() && !force {
        bail!(
            "Target directory already exists: {}.\n\
             Use --force to reinstall.",
            target_dir.display()
        );
    }

    // Copy to target
    copy_skill_to_target(&source_path, target_dir)?;

    // temp_dir is dropped here, cleaning up the clone
    drop(temp_dir);
    Ok(())
}

/// Clone a git repository and return the path to the skill source
async fn clone_git_repo(
    url: &str,
    git_ref: Option<&str>,
    subpath: Option<&str>,
) -> Result<(PathBuf, tempfile::TempDir)> {
    // Create temp directory for clone
    let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
    let clone_path = temp_dir.path();

    // Build git clone command
    let mut cmd = Command::new("git");
    cmd.arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--single-branch");

    if let Some(r) = git_ref {
        cmd.arg("--branch").arg(r);
    }

    cmd.arg(url).arg(clone_path);

    println!("  Cloning repository...");
    let output = cmd.output().context("Failed to execute git clone")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Git clone failed: {}", stderr.trim());
    }

    // Determine source path within clone
    let source_path = if let Some(p) = subpath {
        clone_path.join(p)
    } else {
        clone_path.to_path_buf()
    };

    // Validate skill structure
    if !source_path.join("SKILL.md").exists() {
        bail!(
            "No SKILL.md found in {}.\n\
             This doesn't appear to be a valid skill.",
            source_path.display()
        );
    }

    Ok((source_path, temp_dir))
}

/// Copy skill files to target directory
fn copy_skill_to_target(source_path: &Path, target_dir: &Path) -> Result<()> {
    // Create parent directories
    if let Some(parent) = target_dir.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    // Copy skill to target
    println!("  Copying to {}...", target_dir.display());
    copy_dir_recursive(source_path, target_dir)?;

    // Remove .git directory if it was copied
    let git_dir = target_dir.join(".git");
    if git_dir.exists() {
        std::fs::remove_dir_all(&git_dir).ok();
    }

    Ok(())
}

/// Install a skill from a local path
async fn install_from_local(source: &Path, install_dir: &Path, force: bool) -> Result<()> {
    let source = if source.is_absolute() {
        source.to_path_buf()
    } else {
        std::env::current_dir()?.join(source)
    };

    println!("Installing from local path: {}", source.display());

    // Validate source exists
    if !source.exists() {
        bail!("Source path does not exist: {}", source.display());
    }

    // Validate skill structure
    if !source.join("SKILL.md").exists() {
        bail!(
            "No SKILL.md found in {}.\n\
             This doesn't appear to be a valid skill.",
            source.display()
        );
    }

    // Load skill to get metadata
    let skill = Skill::load(&source).context("Failed to load skill")?;
    let skill_name = skill.name().to_string();

    // Determine target directory
    let target_dir = install_dir.join(&skill_name);

    // Check if source and target are the same
    if source.canonicalize().ok() == target_dir.canonicalize().ok() {
        println!("✓ Skill is already in the target location");
        return Ok(());
    }

    // Check if already installed
    if target_dir.exists() {
        if !force {
            bail!(
                "Skill '{}' already exists at {}.\n\
                 Use --force to reinstall.",
                skill_name,
                target_dir.display()
            );
        }
        println!("  Removing existing installation...");
        std::fs::remove_dir_all(&target_dir)
            .with_context(|| format!("Failed to remove {}", target_dir.display()))?;
    }

    // Create parent directories
    if let Some(parent) = target_dir.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    // Copy skill to target
    println!("  Copying to {}...", target_dir.display());
    copy_dir_recursive(&source, &target_dir)?;

    println!("✓ Installed {} from local path", skill_name);
    println!("  Location: {}", target_dir.display());

    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)
        .with_context(|| format!("Failed to create directory {}", dst.display()))?;

    for entry in std::fs::read_dir(src)
        .with_context(|| format!("Failed to read directory {}", src.display()))?
    {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            // Skip .git directories
            if entry.file_name() == ".git" {
                continue;
            }
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            std::fs::copy(&src_path, &dst_path).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    src_path.display(),
                    dst_path.display()
                )
            })?;
        } else if file_type.is_symlink() {
            // Copy symlink target
            let target = std::fs::read_link(&src_path)?;
            #[cfg(unix)]
            std::os::unix::fs::symlink(&target, &dst_path)
                .with_context(|| format!("Failed to create symlink at {}", dst_path.display()))?;
            #[cfg(windows)]
            {
                if src_path.is_dir() {
                    std::os::windows::fs::symlink_dir(&target, &dst_path)?;
                } else {
                    std::os::windows::fs::symlink_file(&target, &dst_path)?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_ref_parse() {
        let ref1 = SkillRef::parse("stakpak/kubernetes-deploy").unwrap();
        assert_eq!(ref1.account, "stakpak");
        assert_eq!(ref1.name, "kubernetes-deploy");
        assert!(ref1.version.is_none());

        let ref2 = SkillRef::parse("stakpak/kubernetes-deploy@1.2.3").unwrap();
        assert_eq!(ref2.account, "stakpak");
        assert_eq!(ref2.name, "kubernetes-deploy");
        assert_eq!(ref2.version, Some("1.2.3".to_string()));
    }

    #[test]
    fn test_skill_ref_parse_invalid() {
        assert!(SkillRef::parse("invalid").is_err());
        assert!(SkillRef::parse("too/many/slashes").is_err());
        assert!(SkillRef::parse("UPPERCASE/skill").is_err());
    }

    #[test]
    fn test_detect_source_type() {
        // Registry references
        matches!(
            detect_source_type("stakpak/kubernetes-deploy"),
            SourceType::Registry(_)
        );
        matches!(
            detect_source_type("stakpak/kubernetes-deploy@1.0.0"),
            SourceType::Registry(_)
        );

        // Git URLs
        matches!(
            detect_source_type("https://github.com/user/repo.git"),
            SourceType::Git { .. }
        );
        matches!(
            detect_source_type("git@github.com:user/repo.git"),
            SourceType::Git { .. }
        );

        // Local paths
        matches!(detect_source_type("./my-skill"), SourceType::Local(_));
        matches!(detect_source_type("../other-skill"), SourceType::Local(_));
        matches!(detect_source_type("/absolute/path"), SourceType::Local(_));
    }

    #[test]
    fn test_parse_git_url() {
        let (url, git_ref, path) =
            parse_git_url("https://github.com/user/repo.git#ref=v1.0.0&path=skills/my-skill");
        assert_eq!(url, "https://github.com/user/repo.git");
        assert_eq!(git_ref, Some("v1.0.0".to_string()));
        assert_eq!(path, Some("skills/my-skill".to_string()));

        let (url2, git_ref2, path2) = parse_git_url("https://github.com/user/repo.git");
        assert_eq!(url2, "https://github.com/user/repo.git");
        assert!(git_ref2.is_none());
        assert!(path2.is_none());
    }
}
