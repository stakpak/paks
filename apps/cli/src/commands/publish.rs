//! Publish command - publish a skill to the registry

use anyhow::{Result, bail};
use std::path::Path;

use super::core::skill::Skill;

#[derive(Clone, Copy)]
pub enum BumpLevel {
    Patch,
    Minor,
    Major,
}

pub struct PublishArgs {
    pub path: String,
    pub bump: Option<BumpLevel>,
    pub skip_validation: bool,
    pub dry_run: bool,
}

pub async fn run(args: PublishArgs) -> Result<()> {
    let skill_path = Path::new(&args.path);

    // Load the skill
    let mut skill = Skill::load(skill_path)?;

    println!("Publishing skill: {} v{}", skill.name(), skill.version());

    // Validate unless skipped
    if !args.skip_validation {
        let warnings = skill.frontmatter.validate()?;
        for warning in &warnings {
            println!("  ⚠ {}", warning);
        }
    }

    // Check required fields for publishing
    if skill.frontmatter.version.is_none() {
        bail!("Version is required for publishing. Add 'version' to SKILL.md frontmatter.");
    }

    // Bump version if requested
    if let Some(level) = args.bump {
        let current = skill.frontmatter.version.as_deref().unwrap_or("0.1.0");
        let new_version = bump_version(current, level)?;
        println!("  Bumping version: {} → {}", current, new_version);
        skill.frontmatter.version = Some(new_version);

        if !args.dry_run {
            skill.save()?;
        }
    }

    if args.dry_run {
        println!("\n[Dry run] Would publish:");
        println!("  Name: {}", skill.name());
        println!("  Version: {}", skill.version());
        println!("  Description: {}", skill.frontmatter.description);
        if let Some(license) = &skill.frontmatter.license {
            println!("  License: {}", license);
        }
        if !skill.frontmatter.keywords.is_empty() {
            println!("  Keywords: {}", skill.frontmatter.keywords.join(", "));
        }

        // List files that would be included
        println!("\n  Files:");
        println!("    SKILL.md");
        if skill.has_scripts() {
            println!("    scripts/");
        }
        if skill.has_references() {
            println!("    references/");
        }
        if skill.has_assets() {
            println!("    assets/");
        }

        println!("\n✓ Dry run complete. Use without --dry-run to publish.");
    } else {
        // TODO: Implement actual registry upload
        // 1. Check authentication (login token)
        // 2. Package skill files
        // 3. Upload to registry API
        println!("\n⚠ Registry upload not yet implemented.");
        println!("  Skill validated and ready for publishing.");
    }

    Ok(())
}

/// Bump a semantic version string
fn bump_version(version: &str, level: BumpLevel) -> Result<String> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        bail!(
            "Invalid version format '{}'. Expected MAJOR.MINOR.PATCH",
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

    let (new_major, new_minor, new_patch) = match level {
        BumpLevel::Major => (major + 1, 0, 0),
        BumpLevel::Minor => (major, minor + 1, 0),
        BumpLevel::Patch => (major, minor, patch + 1),
    };

    Ok(format!("{}.{}.{}", new_major, new_minor, new_patch))
}
