//! Publish command - publish a skill to the registry

use anyhow::{Result, bail};
use dialoguer::{Confirm, Input};
use paks_api::{PaksClient, PublishPakRequest};
use std::path::Path;

use super::core::config::Config;
use super::core::git;
use super::core::skill::Skill;

pub struct PublishArgs {
    pub path: String,
    pub skip_validation: bool,
    pub dry_run: bool,
    pub no_push: bool,
    pub message: Option<String>,
    pub branch: Option<String>,
    pub yes: bool,
}

/// Prompt for confirmation before publishing
fn prompt_confirm_publish(pak_name: &str, version: &str, tag: &str, branch: &str) -> Result<bool> {
    Confirm::new()
        .with_prompt(format!(
            "Publish {}@{} (tag: {} on branch: {})?",
            pak_name, version, tag, branch
        ))
        .default(true)
        .interact()
        .map_err(Into::into)
}

/// Prompt for custom tag message
fn prompt_tag_message(default: &str) -> Result<String> {
    Input::new()
        .with_prompt("Tag message")
        .default(default.to_string())
        .interact_text()
        .map_err(Into::into)
}

pub async fn run(args: PublishArgs) -> Result<()> {
    let skill_path = Path::new(&args.path);

    // Load the skill
    let skill = Skill::load(skill_path)?;

    println!("Publishing skill: {}", skill.name());

    // Validate unless skipped
    if !args.skip_validation {
        print!("  Validating... ");
        let warnings = skill.frontmatter.validate()?;
        println!("✓");
        for warning in &warnings {
            println!("  ⚠ {}", warning);
        }
    }

    // Check required fields for publishing
    let version = skill
        .frontmatter
        .version
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Version is required for publishing. Add 'version' to SKILL.md frontmatter."))?;

    // Git checks
    if !git::is_git_repo(skill_path) {
        bail!("Not a git repository.");
    }

    let remote = "origin";
    let repo_url = git::get_remote_url(skill_path, remote)?;
    let branch = args
        .branch
        .clone()
        .or_else(|| git::get_current_branch(skill_path).ok())
        .ok_or_else(|| anyhow::anyhow!("Could not determine current branch"))?;

    let tag = format!("v{}", version);

    // Check tag doesn't exist
    if git::tag_exists(skill_path, &tag) {
        bail!("Tag {} already exists. Bump the version in SKILL.md first.", tag);
    }

    // Get tag message - prompt if not provided and not dry-run and not --yes
    let tag_msg = if let Some(msg) = args.message.clone() {
        msg
    } else if !args.dry_run && !args.yes {
        prompt_tag_message(&format!("Release {}", tag))?
    } else {
        format!("Release {}", tag)
    };

    // Dry run
    if args.dry_run {
        println!();
        println!("[Dry run] Would execute:");
        println!("  1. git tag -a {} -m \"{}\"", tag, tag_msg);
        if !args.no_push {
            println!("  2. git push {} {}", remote, tag);
            println!("  3. POST /v1/paks/publish");
        }
        println!();
        println!("✓ Dry run complete.");
        return Ok(());
    }

    // Confirm before publishing (unless --yes)
    if !args.yes {
        println!();
        if !prompt_confirm_publish(skill.name(), version, &tag, &branch)? {
            println!("Aborted.");
            return Ok(());
        }
    }

    // Execute
    println!();

    // Create tag
    print!("  Creating tag {}... ", tag);
    git::create_tag(skill_path, &tag, &tag_msg)?;
    println!("✓");

    if args.no_push {
        println!();
        println!("✓ Tag created locally. Push manually when ready:");
        println!("  git push {} {}", remote, tag);
        return Ok(());
    }

    // Push tag
    print!("  Pushing tag... ");
    git::push_tag(skill_path, remote, &tag)?;
    println!("✓");

    // Notify registry
    print!("  Registering with registry... ");

    let config = Config::load()?;
    let token = config
        .get_auth_token()
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'paks login' first."))?;

    // Determine pak path relative to repo root
    let pak_path_in_repo = git::get_pak_path_in_repo(skill_path)?;

    let mut client = PaksClient::new()?;
    client.set_token(token);

    let request = PublishPakRequest {
        repository: repo_url,
        path: if pak_path_in_repo == "." {
            None
        } else {
            Some(pak_path_in_repo)
        },
        branch,
        tag: tag.clone(),
    };

    client.publish_pak(request).await?;
    println!("✓");

    println!();
    println!("✓ Published {}@{}", skill.name(), version);

    Ok(())
}
