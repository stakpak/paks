//! Validate command - validate a skill's structure and SKILL.md

use anyhow::{Result, bail};
use std::path::Path;

use super::core::skill::Skill;

pub struct ValidateArgs {
    pub path: String,
    pub strict: bool,
}

pub async fn run(args: ValidateArgs) -> Result<()> {
    let skill_path = Path::new(&args.path);

    // Load and parse the skill
    let skill = match Skill::load(skill_path) {
        Ok(s) => s,
        Err(e) => {
            println!("✗ Failed to load skill: {}", e);
            bail!("Validation failed");
        }
    };

    println!("Validating skill: {}", skill.name());

    let mut has_errors = false;
    let mut warnings = Vec::new();

    // Validate frontmatter
    match skill.frontmatter.validate() {
        Ok(w) => warnings.extend(w),
        Err(e) => {
            println!("  ✗ Frontmatter error: {}", e);
            has_errors = true;
        }
    }

    // Check for version (recommended for publishing)
    if skill.frontmatter.version.is_none() {
        warnings.push("No version specified - required for publishing".to_string());
    }

    // Check for license (recommended)
    if skill.frontmatter.license.is_none() {
        warnings.push("No license specified - recommended for sharing".to_string());
    }

    // Check optional directories structure
    if skill.has_scripts() {
        let scripts_dir = skill_path.join("scripts");
        if let Ok(entries) = std::fs::read_dir(&scripts_dir) {
            let count = entries.count();
            if count == 0 {
                warnings.push("scripts/ directory is empty".to_string());
            } else {
                println!("  ✓ scripts/ ({} files)", count);
            }
        }
    }

    if skill.has_references() {
        let refs_dir = skill_path.join("references");
        if let Ok(entries) = std::fs::read_dir(&refs_dir) {
            let count = entries.count();
            if count == 0 {
                warnings.push("references/ directory is empty".to_string());
            } else {
                println!("  ✓ references/ ({} files)", count);
            }
        }
    }

    if skill.has_assets() {
        let assets_dir = skill_path.join("assets");
        if let Ok(entries) = std::fs::read_dir(&assets_dir) {
            let count = entries
                .filter(|e| {
                    e.as_ref()
                        .map(|e| e.file_name() != ".gitkeep")
                        .unwrap_or(false)
                })
                .count();
            if count == 0 {
                warnings.push("assets/ directory is empty".to_string());
            } else {
                println!("  ✓ assets/ ({} files)", count);
            }
        }
    }

    // Print warnings
    for warning in &warnings {
        println!("  ⚠ {}", warning);
    }

    // In strict mode, warnings are errors
    if args.strict && !warnings.is_empty() {
        has_errors = true;
    }

    if has_errors {
        println!("\n✗ Validation failed");
        bail!("Validation failed");
    } else if warnings.is_empty() {
        println!("\n✓ Skill is valid");
    } else {
        println!("\n✓ Skill is valid ({} warnings)", warnings.len());
    }

    // Print skill summary
    println!("\nSkill Summary:");
    println!("  Name: {}", skill.name());
    println!("  Version: {}", skill.version());
    println!("  Description: {}", skill.frontmatter.description);
    if let Some(license) = &skill.frontmatter.license {
        println!("  License: {}", license);
    }
    if !skill.frontmatter.keywords.is_empty() {
        println!("  Keywords: {}", skill.frontmatter.keywords.join(", "));
    }

    Ok(())
}
