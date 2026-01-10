//! Publish command - publish a skill to the registry

use anyhow::{Result, bail};
use dialoguer::{Confirm, Input, Select};
use paks_api::{PaksClient, PublishPakRequest};
use std::io::{self, Write};
use std::path::Path;

use super::core::config::Config;
use super::core::git;
use super::core::skill::Skill;

pub struct PublishArgs {
    pub path: String,
    pub skip_validation: bool,
    pub dry_run: bool,
    pub yes: bool,
    pub tag: Option<String>,
}

/// Prompt for confirmation to continue with uncommitted changes
fn prompt_continue_with_changes(changes: &[String]) -> Result<bool> {
    println!("  ⚠ Uncommitted changes detected:");
    for change in changes.iter().take(10) {
        println!("    {}", change);
    }
    if changes.len() > 10 {
        println!("    ... and {} more", changes.len() - 10);
    }
    println!();
    // Flush stdout to ensure output is visible before prompt
    io::stdout().flush()?;

    Confirm::new()
        .with_prompt("Continue publishing with uncommitted changes?")
        .default(false)
        .interact()
        .map_err(Into::into)
}

/// Prompt for confirmation before publishing
fn prompt_confirm_publish(pak_name: &str, tag: &str, branch: &str, pak_path: &str) -> Result<bool> {
    Confirm::new()
        .with_prompt(format!(
            "Publish {} (tag: {} on branch: {}, path: {})?",
            pak_name, tag, branch, pak_path
        ))
        .default(true)
        .interact()
        .map_err(Into::into)
}

/// Tag selection result
enum TagSelection {
    Existing(String),
    New(String),
}

/// Filter tags to only include valid semver tags (v*.*.*)
fn filter_semver_tags(tags: &[String]) -> Vec<String> {
    tags.iter()
        .filter(|tag| parse_version(tag).is_ok())
        .cloned()
        .collect()
}

/// Prompt user to select version bump type, existing tag, or enter custom version
fn prompt_tag_selection(existing_tags: &[String], current_version: &str) -> Result<TagSelection> {
    // Filter to only semver tags
    let semver_tags = filter_semver_tags(existing_tags);

    // Check if we have a valid current version to bump from
    let has_valid_version = parse_version(current_version).is_ok();

    let mut options: Vec<String> = Vec::new();
    let mut option_count = 0;

    // Add bump options only if we have a valid base version
    if has_valid_version {
        let (major, minor, patch) = parse_version(current_version)?;
        let patch_version = format!("v{}.{}.{}", major, minor, patch + 1);
        let minor_version = format!("v{}.{}.{}", major, minor + 1, 0);
        let major_version = format!("v{}.{}.{}", major + 1, 0, 0);

        options.push(format!("📦 Patch  → {}", patch_version));
        options.push(format!("🔧 Minor  → {}", minor_version));
        options.push(format!("🚀 Major  → {}", major_version));
        option_count = 3;
    }

    // Add existing semver tags
    let existing_start_idx = options.len();
    for tag in &semver_tags {
        options.push(format!("📌 Use existing: {}", tag));
    }

    // Custom version entry at the end
    let custom_idx = options.len();
    options.push("✏️  Enter custom version...".to_string());

    let selection = Select::new()
        .with_prompt("Select a version")
        .items(&options)
        .default(0)
        .interact()?;

    if has_valid_version {
        let (major, minor, patch) = parse_version(current_version)?;
        if selection < option_count {
            // Bump options
            match selection {
                0 => Ok(TagSelection::New(format!(
                    "v{}.{}.{}",
                    major,
                    minor,
                    patch + 1
                ))),
                1 => Ok(TagSelection::New(format!("v{}.{}.{}", major, minor + 1, 0))),
                2 => Ok(TagSelection::New(format!("v{}.{}.{}", major + 1, 0, 0))),
                _ => unreachable!(),
            }
        } else if selection == custom_idx {
            prompt_custom_version()
        } else {
            // Existing tag
            let tag_idx = selection - existing_start_idx;
            Ok(TagSelection::Existing(semver_tags[tag_idx].clone()))
        }
    } else if selection == custom_idx {
        prompt_custom_version()
    } else {
        // Existing tag
        Ok(TagSelection::Existing(semver_tags[selection].clone()))
    }
}

/// Prompt user to enter a custom version
fn prompt_custom_version() -> Result<TagSelection> {
    let version: String = Input::new()
        .with_prompt("Enter version (e.g., 1.0.0 or v1.0.0)")
        .interact_text()?;

    // Normalize to v-prefixed format
    let tag = if version.starts_with('v') {
        version
    } else {
        format!("v{}", version)
    };

    // Validate it's a valid semver
    parse_version(&tag)?;

    Ok(TagSelection::New(tag))
}

/// Parse version string into (major, minor, patch)
fn parse_version(version: &str) -> Result<(u32, u32, u32)> {
    let v = version.strip_prefix('v').unwrap_or(version);
    let parts: Vec<&str> = v.split('.').collect();

    if parts.len() != 3 {
        bail!(
            "Invalid version format: {}. Expected MAJOR.MINOR.PATCH",
            version
        );
    }

    let major: u32 = parts[0]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid major version"))?;
    let minor: u32 = parts[1]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid minor version"))?;
    let patch: u32 = parts[2]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid patch version"))?;

    Ok((major, minor, patch))
}

/// Extract version string from tag (strips 'v' prefix if present)
fn tag_to_version(tag: &str) -> String {
    tag.strip_prefix('v').unwrap_or(tag).to_string()
}

/// Prompt for confirmation to update SKILL.md version before publishing
fn prompt_version_sync(current_version: &str, new_version: &str, branch: &str) -> Result<bool> {
    println!("  Version mismatch detected:");
    println!("    SKILL.md version: {}", current_version);
    println!("    Selected version: {}", new_version);
    println!();
    println!("  The following changes will be made before tagging:");
    println!(
        "    1. Update SKILL.md: metadata.version {} -> {}",
        current_version, new_version
    );
    println!("    2. Commit: \"chore: bump version to {}\"", new_version);
    println!("    3. Push to origin/{}", branch);
    println!();
    // Flush stdout to ensure output is visible before prompt
    io::stdout().flush()?;

    Confirm::new()
        .with_prompt("Proceed with version update?")
        .default(true)
        .interact()
        .map_err(Into::into)
}

pub async fn run(args: PublishArgs) -> Result<()> {
    let skill_path = Path::new(&args.path).canonicalize()?;

    // Step 1: Load and validate the skill
    let mut skill = Skill::load(&skill_path)?;
    println!("Publishing skill: {}", skill.name());

    // Validate unless skipped
    if !args.skip_validation {
        print!("  Validating SKILL.md... ");
        let warnings = skill.frontmatter.validate()?;
        println!("✓");
        for warning in &warnings {
            println!("  ⚠ {}", warning);
        }
    }

    // Get current version from SKILL.md
    let current_version = skill.version();

    // Step 2: Git checks
    if !git::is_git_repo(&skill_path) {
        bail!("Not a git repository.");
    }

    let remote = "origin";
    let repo_url = git::get_remote_url(&skill_path, remote)?;
    let branch = git::get_current_branch(&skill_path)?;

    // Get pak path relative to repo root (this is what we send to the API)
    let pak_path_in_repo = git::get_pak_path_in_repo(&skill_path)?;

    // Step 3: Check for uncommitted changes in the skill directory
    let uncommitted_changes = git::get_uncommitted_changes(&skill_path)?;
    if !uncommitted_changes.is_empty() && !args.yes {
        println!();
        if !prompt_continue_with_changes(&uncommitted_changes)? {
            println!("Aborted.");
            return Ok(());
        }
    } else if !uncommitted_changes.is_empty() && args.yes {
        println!(
            "  ⚠ {} uncommitted changes detected, continuing with --yes",
            uncommitted_changes.len()
        );
    }

    // Step 4: Determine which tag to use
    let existing_tags = git::list_tags(&skill_path)?;

    let (tag, needs_create) = if let Some(explicit_tag) = args.tag.clone() {
        // User explicitly provided a tag via --tag flag - validate it's semver
        let tag_to_check = if explicit_tag.starts_with('v') {
            explicit_tag.clone()
        } else {
            format!("v{}", explicit_tag)
        };
        // Validate it's a valid semver
        parse_version(&tag_to_check)?;
        if !git::tag_exists(&skill_path, &tag_to_check) {
            bail!("Tag {} does not exist.", tag_to_check);
        }
        (tag_to_check, false)
    } else if args.yes {
        // Non-interactive mode: create patch bump
        let (major, minor, patch) = parse_version(current_version)?;
        let new_tag = format!("v{}.{}.{}", major, minor, patch + 1);
        if git::tag_exists(&skill_path, &new_tag) {
            bail!("Tag {} already exists.", new_tag);
        }
        (new_tag, true)
    } else {
        // Interactive mode: let user choose bump type or existing tag
        println!();
        match prompt_tag_selection(&existing_tags, current_version)? {
            TagSelection::New(tag) => {
                if git::tag_exists(&skill_path, &tag) {
                    bail!("Tag {} already exists.", tag);
                }
                (tag, true)
            }
            TagSelection::Existing(tag) => (tag, false),
        }
    };

    // Step 4.5: Version synchronization
    // Check if SKILL.md version matches the selected tag version
    // Only sync if SKILL.md explicitly has a version field (it's optional)
    let tag_version = tag_to_version(&tag);
    let skill_version_opt = skill.version_opt();
    let needs_version_sync = skill_version_opt.is_some_and(|v| v != tag_version);

    // Case C: Using existing tag with version mismatch - fail with error
    if !needs_create && needs_version_sync {
        // Safe to unwrap: needs_version_sync is only true when skill_version_opt is Some
        let skill_version = skill_version_opt.unwrap();
        println!();
        println!("  Error: SKILL.md version mismatch");
        println!();
        println!("    Tag version:      {}", tag);
        println!("    SKILL.md version: {}", skill_version);
        println!();
        println!("  The registry requires SKILL.md metadata.version to match the tag.");
        println!("  Since tag {} already exists, you have two options:", tag);
        println!();
        println!("    1. Delete the tag and republish:");
        println!("       git tag -d {}", tag);
        println!("       git push origin :refs/tags/{}", tag);
        println!("       paks publish");
        println!();
        println!("    2. Create a new tag with the correct version:");
        println!("       # First update SKILL.md to the desired version");
        println!("       # Then run: paks publish");
        println!();
        bail!("Cannot publish: SKILL.md version doesn't match existing tag");
    }

    // Case B: Creating new tag with version mismatch - need to sync
    if needs_create && needs_version_sync {
        // Safe to unwrap: needs_version_sync is only true when skill_version_opt is Some
        let skill_version = skill_version_opt.unwrap();
        println!();
        if !args.yes {
            // Interactive mode: prompt for confirmation
            if !prompt_version_sync(skill_version, &tag_version, &branch)? {
                println!("Aborted.");
                return Ok(());
            }
        } else {
            // Non-interactive mode: show what we're doing
            println!(
                "  Version mismatch: SKILL.md ({}) -> {}",
                skill_version, tag_version
            );
        }
    }

    // Dry run output
    if args.dry_run {
        println!();
        println!("[Dry run] Would execute:");
        println!("  Repository: {}", repo_url);
        println!("  Branch: {}", branch);
        println!("  Path: {}", pak_path_in_repo);
        println!("  Tag: {}", tag);
        if needs_version_sync && needs_create {
            // Safe to unwrap: needs_version_sync is only true when skill_version_opt is Some
            let skill_version = skill_version_opt.unwrap();
            println!(
                "  Version sync: Update SKILL.md ({} -> {}), commit, push",
                skill_version, tag_version
            );
        }
        if needs_create {
            println!("  Action: Create and push new tag, then register with registry");
        } else {
            println!("  Action: Register existing tag with registry");
        }
        println!();
        println!("✓ Dry run complete.");
        return Ok(());
    }

    // Step 5: Confirm before publishing (unless --yes)
    if !args.yes {
        println!();
        if !prompt_confirm_publish(skill.name(), &tag, &branch, &pak_path_in_repo)? {
            println!("Aborted.");
            return Ok(());
        }
    }

    // Step 6: Execute
    println!();

    // Sync version if needed (update SKILL.md, commit, push)
    if needs_create && needs_version_sync {
        // Safe to unwrap: needs_version_sync is only true when skill_version_opt is Some
        let skill_version = skill_version_opt.unwrap();
        print!(
            "  Updating SKILL.md version ({} -> {})... ",
            skill_version, tag_version
        );
        skill.set_version(&tag_version);
        skill.save()?;
        println!("✓");

        print!("  Staging changes... ");
        git::stage_file(&skill_path, "SKILL.md")?;
        println!("✓");

        print!("  Committing... ");
        let commit_msg = format!("chore: bump version to {}", tag_version);
        git::commit(&skill_path, &commit_msg)?;
        println!("✓");

        print!("  Pushing to origin/{}... ", branch);
        git::push_branch(&skill_path, remote, &branch)?;
        println!("✓");
    }

    // Create and push tag if needed
    if needs_create {
        let tag_msg = format!("Release {}", tag);

        print!("  Creating tag {}... ", tag);
        git::create_tag(&skill_path, &tag, &tag_msg)?;
        println!("✓");

        print!("  Pushing tag... ");
        git::push_tag(&skill_path, remote, &tag)?;
        println!("✓");
    } else {
        println!("  Using existing tag: {}", tag);
    }

    // Step 7: Register with registry
    print!("  Registering with registry... ");

    let config = Config::load()?;
    let token = config
        .get_auth_token()
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'paks login' first."))?;

    let mut client = PaksClient::new()?;
    client.set_token(token);

    let request = PublishPakRequest {
        repository: repo_url,
        path: if pak_path_in_repo == "." {
            None
        } else {
            Some(pak_path_in_repo.clone())
        },
        branch,
        tag: tag.clone(),
    };

    client.publish_pak(request).await?;
    println!("✓");

    println!();
    println!(
        "✓ Published {} @ {} (path: {})",
        skill.name(),
        tag,
        pak_path_in_repo
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_to_version_with_prefix() {
        assert_eq!(tag_to_version("v1.0.0"), "1.0.0");
        assert_eq!(tag_to_version("v0.1.0"), "0.1.0");
        assert_eq!(tag_to_version("v10.20.30"), "10.20.30");
    }

    #[test]
    fn test_tag_to_version_without_prefix() {
        assert_eq!(tag_to_version("1.0.0"), "1.0.0");
        assert_eq!(tag_to_version("0.1.0"), "0.1.0");
    }

    #[test]
    fn test_parse_version_valid() {
        assert_eq!(parse_version("v1.2.3").unwrap(), (1, 2, 3));
        assert_eq!(parse_version("1.2.3").unwrap(), (1, 2, 3));
        assert_eq!(parse_version("v0.0.0").unwrap(), (0, 0, 0));
        assert_eq!(parse_version("v10.20.30").unwrap(), (10, 20, 30));
    }

    #[test]
    fn test_parse_version_invalid() {
        // Missing parts
        assert!(parse_version("v1.2").is_err());
        assert!(parse_version("v1").is_err());
        assert!(parse_version("").is_err());

        // Too many parts
        assert!(parse_version("v1.2.3.4").is_err());

        // Non-numeric
        assert!(parse_version("v1.2.x").is_err());
        assert!(parse_version("va.b.c").is_err());
    }

    #[test]
    fn test_filter_semver_tags() {
        let tags = vec![
            "v1.0.0".to_string(),
            "v0.1.0".to_string(),
            "latest".to_string(),
            "v2.0.0-beta".to_string(), // This will fail parse_version
            "release-1".to_string(),
            "v0.0.1".to_string(),
        ];
        let filtered = filter_semver_tags(&tags);
        assert_eq!(filtered, vec!["v1.0.0", "v0.1.0", "v0.0.1"]);
    }

    #[test]
    fn test_version_sync_needed() {
        // Version sync is needed when skill has explicit version that differs from tag
        let skill_version: Option<&str> = Some("0.1.0");
        let tag_version = "0.2.0";
        let needs_sync = skill_version.is_some_and(|v| v != tag_version);
        assert!(needs_sync);
    }

    #[test]
    fn test_version_sync_not_needed_when_match() {
        // Version sync not needed when versions match
        let skill_version: Option<&str> = Some("0.1.0");
        let tag_version = "0.1.0";
        let needs_sync = skill_version.is_some_and(|v| v != tag_version);
        assert!(!needs_sync);
    }

    #[test]
    fn test_version_sync_not_needed_when_no_version() {
        // Version sync not needed when skill has no explicit version
        let skill_version: Option<&str> = None;
        let tag_version = "0.2.0";
        let needs_sync = skill_version.is_some_and(|v| v != tag_version);
        assert!(!needs_sync);
    }
}
