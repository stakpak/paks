//! Info command - show details about a skill

use anyhow::{Result, bail};
use std::path::Path;

use super::core::skill::Skill;

pub struct InfoArgs {
    pub skill: String,
    pub full: bool,
}

pub async fn run(args: InfoArgs) -> Result<()> {
    let skill_path = Path::new(&args.skill);

    // Check if it's a local path
    if skill_path.exists() {
        let skill = Skill::load(skill_path)?;
        print_skill_info(&skill, args.full);
    } else {
        // TODO: Check registry for skill by name
        bail!(
            "Skill '{}' not found locally. Registry lookup not yet implemented.",
            args.skill
        );
    }

    Ok(())
}

fn print_skill_info(skill: &Skill, full: bool) {
    let fm = &skill.frontmatter;

    println!("╭─────────────────────────────────────────╮");
    println!("│ {}  v{}", fm.name, skill.version());
    println!("╰─────────────────────────────────────────╯");
    println!();
    println!("{}", fm.description);
    println!();

    // Metadata section
    println!("Metadata:");
    if let Some(license) = &fm.license {
        println!("  License:    {}", license);
    }
    if !fm.authors.is_empty() {
        println!("  Authors:    {}", fm.authors.join(", "));
    }
    if let Some(repo) = &fm.repository {
        println!("  Repository: {}", repo);
    }
    if let Some(homepage) = &fm.homepage {
        println!("  Homepage:   {}", homepage);
    }

    // Keywords and categories
    if !fm.keywords.is_empty() {
        println!("  Keywords:   {}", fm.keywords.join(", "));
    }
    if !fm.categories.is_empty() {
        println!("  Categories: {}", fm.categories.join(", "));
    }

    // Compatibility
    if let Some(compat) = &fm.compatibility {
        println!("  Compat:     {}", compat);
    }

    // Dependencies
    if !fm.dependencies.is_empty() {
        println!("\nDependencies:");
        for dep in &fm.dependencies {
            let version = dep.version.as_deref().unwrap_or("*");
            if let Some(git) = &dep.git {
                println!("  {} (git: {})", dep.name, git);
            } else if let Some(path) = &dep.path {
                println!("  {} (path: {})", dep.name, path);
            } else {
                println!("  {} @ {}", dep.name, version);
            }
        }
    }

    // Directory structure
    println!("\nStructure:");
    println!("  SKILL.md");
    if skill.has_scripts() {
        println!("  scripts/");
    }
    if skill.has_references() {
        println!("  references/");
    }
    if skill.has_assets() {
        println!("  assets/");
    }

    // Full content
    if full {
        println!("\n─────────────────────────────────────────");
        println!("SKILL.md Content:");
        println!("─────────────────────────────────────────");
        println!("{}", skill.instructions);
    }
}
