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
    } else {
        if selection == custom_idx {
            prompt_custom_version()
        } else {
            // Existing tag
            Ok(TagSelection::Existing(semver_tags[selection].clone()))
        }
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

pub async fn run(args: PublishArgs) -> Result<()> {
    let skill_path = Path::new(&args.path).canonicalize()?;

    // Step 1: Load and validate the skill
    let skill = Skill::load(&skill_path)?;
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

    // Dry run output
    if args.dry_run {
        println!();
        println!("[Dry run] Would execute:");
        println!("  Repository: {}", repo_url);
        println!("  Branch: {}", branch);
        println!("  Path: {}", pak_path_in_repo);
        println!("  Tag: {}", tag);
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

    let mut client = PaksClient::builder()
        .base_url("http://localhost:4000")
        .build()?;
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
