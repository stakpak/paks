//! Git helper functions for publish command

use anyhow::{Result, bail};
use std::path::Path;
use std::process::Command;

/// Execute a git command and return stdout
pub fn git_cmd(args: &[&str], path: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check if path is inside a git repository
pub fn is_git_repo(path: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get the URL of a remote
pub fn get_remote_url(path: &Path, remote: &str) -> Result<String> {
    git_cmd(&["remote", "get-url", remote], path)
}

/// Get the current branch name
pub fn get_current_branch(path: &Path) -> Result<String> {
    let branch = git_cmd(&["rev-parse", "--abbrev-ref", "HEAD"], path)?;
    if branch == "HEAD" {
        bail!("Detached HEAD. Checkout a branch first.");
    }
    Ok(branch)
}

/// Check if a tag exists locally
pub fn tag_exists(path: &Path, tag: &str) -> bool {
    git_cmd(&["tag", "-l", tag], path)
        .map(|o| !o.is_empty())
        .unwrap_or(false)
}

/// Create an annotated tag
pub fn create_tag(path: &Path, tag: &str, message: &str) -> Result<()> {
    git_cmd(&["tag", "-a", tag, "-m", message], path)?;
    Ok(())
}

/// Push a tag to remote
pub fn push_tag(path: &Path, remote: &str, tag: &str) -> Result<()> {
    git_cmd(&["push", remote, tag], path)?;
    Ok(())
}

/// Get the pak path relative to the repository root
pub fn get_pak_path_in_repo(pak_path: &Path) -> Result<String> {
    let repo_root = git_cmd(&["rev-parse", "--show-toplevel"], pak_path)?;
    let repo_root = Path::new(&repo_root);
    let abs_pak = pak_path.canonicalize()?;
    let rel_path = abs_pak.strip_prefix(repo_root)?;

    if rel_path.as_os_str().is_empty() {
        Ok(".".to_string())
    } else {
        Ok(rel_path.to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_is_git_repo_false() {
        let dir = tempdir().unwrap();
        assert!(!is_git_repo(dir.path()));
    }
}
