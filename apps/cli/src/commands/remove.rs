//! Remove command - remove an installed skill

use anyhow::{Result, bail};
use std::io::{self, Write};
use std::path::PathBuf;

use super::core::config::Config;
use super::core::skill::Skill;

pub struct RemoveArgs {
    pub name: String,
    pub agent: Option<String>,
    pub all: bool,
    pub yes: bool,
}

pub async fn run(args: RemoveArgs) -> Result<()> {
    let config = Config::load()?;

    let mut removed_count = 0;
    let mut not_found_count = 0;

    if args.all {
        // Remove from all agent directories
        for (id, agent_config) in &config.agents {
            let skill_path = agent_config.skills_dir.join(&args.name);
            if skill_path.exists() {
                if confirm_removal(&args.name, &agent_config.name, args.yes)? {
                    remove_skill_dir(&skill_path)?;
                    println!("✓ Removed '{}' from {}", args.name, id);
                    removed_count += 1;
                }
            } else {
                not_found_count += 1;
            }
        }

        if removed_count == 0 && not_found_count > 0 {
            println!("Skill '{}' not found in any agent directory", args.name);
        }
    } else {
        // Get target directory
        let (agent_name, skills_dir) = if let Some(agent_name) = &args.agent {
            if let Some(agent_config) = config.get_agent(agent_name) {
                (agent_config.name.clone(), agent_config.skills_dir.clone())
            } else {
                bail!("Agent '{}' not found", agent_name);
            }
        } else {
            // Use default agent or ~/.paks/skills
            if let Some(agent) = config.get_default_agent() {
                let name = config.default_agent.as_deref().unwrap_or("default");
                (name.to_string(), agent.skills_dir.clone())
            } else {
                ("paks".to_string(), Config::default_skills_dir())
            }
        };

        let skill_path = skills_dir.join(&args.name);

        if !skill_path.exists() {
            bail!(
                "Skill '{}' not found in {} ({})",
                args.name,
                agent_name,
                skills_dir.display()
            );
        }

        // Verify it's a valid skill
        if let Err(e) = Skill::load(&skill_path) {
            println!("⚠ Warning: {} (removing anyway)", e);
        }

        if confirm_removal(&args.name, &agent_name, args.yes)? {
            remove_skill_dir(&skill_path)?;
            println!("✓ Removed '{}' from {}", args.name, agent_name);
        } else {
            println!("Cancelled");
        }
    }

    Ok(())
}

/// Confirm removal with user (unless --yes)
fn confirm_removal(skill_name: &str, agent_name: &str, skip_confirm: bool) -> Result<bool> {
    if skip_confirm {
        return Ok(true);
    }

    print!("Remove skill '{}' from {}? [y/N] ", skill_name, agent_name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes"))
}

/// Remove a skill directory
fn remove_skill_dir(path: &PathBuf) -> Result<()> {
    std::fs::remove_dir_all(path)?;
    Ok(())
}
