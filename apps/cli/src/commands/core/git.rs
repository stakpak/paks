//! Git helper functions for publish command

use anyhow::{Result, bail};
use std::path::Path;
use std::process::Command;

/// Execute a git command and return stdout
pub fn git_cmd(args: &[&str], path: &Path) -> Result<String> {
    let output = Command::new("git").args(args).current_dir(path).output()?;

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

/// Fetch tags from remote to sync local tags with remote
pub fn fetch_tags(path: &Path, remote: &str) -> Result<()> {
    git_cmd(&["fetch", remote, "--tags", "--force"], path)?;
    Ok(())
}

/// List all tags sorted by version (newest first)
pub fn list_tags(path: &Path) -> Result<Vec<String>> {
    let output = git_cmd(&["tag", "-l", "--sort=-v:refname"], path)?;
    if output.is_empty() {
        return Ok(vec![]);
    }
    Ok(output.lines().map(|s| s.to_string()).collect())
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

/// Check for uncommitted changes in a directory (staged + unstaged + untracked)
/// Returns a list of changed files relative to the directory
pub fn get_uncommitted_changes(path: &Path) -> Result<Vec<String>> {
    // When running git status from within the target directory,
    // use "." to check the current directory and its subdirectories
    let output = git_cmd(&["status", "--porcelain", "."], path)?;

    if output.is_empty() {
        return Ok(vec![]);
    }

    Ok(output.lines().map(|s| s.to_string()).collect())
}

/// Stage a file for commit
pub fn stage_file(path: &Path, file_path: &str) -> Result<()> {
    git_cmd(&["add", file_path], path)?;
    Ok(())
}

/// Create a commit with the given message
pub fn commit(path: &Path, message: &str) -> Result<()> {
    git_cmd(&["commit", "-m", message], path)?;
    Ok(())
}

/// Push current branch to remote
pub fn push_branch(path: &Path, remote: &str, branch: &str) -> Result<()> {
    git_cmd(&["push", remote, branch], path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tempfile::tempdir;

    /// Helper to create a git repo in a temp directory
    fn create_git_repo() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        dir
    }

    #[test]
    fn test_is_git_repo_false() {
        let dir = tempdir().unwrap();
        assert!(!is_git_repo(dir.path()));
    }

    #[test]
    fn test_is_git_repo_true() {
        let dir = create_git_repo();
        assert!(is_git_repo(dir.path()));
    }

    #[test]
    fn test_get_current_branch() {
        let dir = create_git_repo();
        // Create an initial commit so we have a branch
        std::fs::write(dir.path().join("test.txt"), "test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        let branch = get_current_branch(dir.path()).unwrap();
        // Branch could be "main" or "master" depending on git config
        assert!(branch == "main" || branch == "master");
    }

    #[test]
    fn test_tag_exists_false() {
        let dir = create_git_repo();
        assert!(!tag_exists(dir.path(), "v1.0.0"));
    }

    #[test]
    fn test_tag_exists_true() {
        let dir = create_git_repo();
        // Create initial commit
        std::fs::write(dir.path().join("test.txt"), "test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        // Create a tag
        Command::new("git")
            .args(["tag", "v1.0.0"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        assert!(tag_exists(dir.path(), "v1.0.0"));
        assert!(!tag_exists(dir.path(), "v2.0.0"));
    }

    #[test]
    fn test_list_tags() {
        let dir = create_git_repo();
        // Create initial commit
        std::fs::write(dir.path().join("test.txt"), "test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        // Create tags
        Command::new("git")
            .args(["tag", "v0.1.0"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["tag", "v1.0.0"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        let tags = list_tags(dir.path()).unwrap();
        assert!(tags.contains(&"v0.1.0".to_string()));
        assert!(tags.contains(&"v1.0.0".to_string()));
    }

    #[test]
    fn test_list_tags_empty() {
        let dir = create_git_repo();
        let tags = list_tags(dir.path()).unwrap();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_create_tag() {
        let dir = create_git_repo();
        // Create initial commit
        std::fs::write(dir.path().join("test.txt"), "test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        create_tag(dir.path(), "v1.0.0", "Release v1.0.0").unwrap();
        assert!(tag_exists(dir.path(), "v1.0.0"));
    }

    #[test]
    fn test_stage_file() {
        let dir = create_git_repo();
        std::fs::write(dir.path().join("test.txt"), "test content").unwrap();

        stage_file(dir.path(), "test.txt").unwrap();

        // Verify file is staged
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        let status = String::from_utf8_lossy(&output.stdout);
        assert!(status.contains("A  test.txt"));
    }

    #[test]
    fn test_commit() {
        let dir = create_git_repo();
        std::fs::write(dir.path().join("test.txt"), "test content").unwrap();
        stage_file(dir.path(), "test.txt").unwrap();

        commit(dir.path(), "Test commit message").unwrap();

        // Verify commit was created
        let output = Command::new("git")
            .args(["log", "--oneline", "-1"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        let log = String::from_utf8_lossy(&output.stdout);
        assert!(log.contains("Test commit message"));
    }

    #[test]
    fn test_get_uncommitted_changes() {
        let dir = create_git_repo();
        // Create initial commit
        std::fs::write(dir.path().join("test.txt"), "test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        // No changes initially
        let changes = get_uncommitted_changes(dir.path()).unwrap();
        assert!(changes.is_empty());

        // Modify a file
        std::fs::write(dir.path().join("test.txt"), "modified").unwrap();
        let changes = get_uncommitted_changes(dir.path()).unwrap();
        assert!(!changes.is_empty());

        // Add untracked file
        std::fs::write(dir.path().join("new.txt"), "new file").unwrap();
        let changes = get_uncommitted_changes(dir.path()).unwrap();
        assert!(changes.len() >= 2);
    }
}
