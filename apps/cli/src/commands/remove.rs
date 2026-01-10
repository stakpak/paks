//! Remove command - remove an installed skill

use anyhow::{Result, bail};
use std::io::{self, Write};
use std::path::PathBuf;

use super::core::config::{Config, Scope};
use super::core::skill::Skill;

pub struct RemoveArgs {
    pub name: String,
    pub agent: Option<String>,
    pub scope: Option<Scope>,
    pub all: bool,
    pub yes: bool,
}

pub async fn run(args: RemoveArgs) -> Result<()> {
    let config = Config::load_merged()?;

    let mut removed_count = 0;
    let mut not_found_count = 0;

    if args.all {
        // Remove from all agent directories (using scope)
        let scope = config.effective_scope(args.scope);

        for (id, agent_config) in &config.agents {
            let skills_dir = match scope {
                Scope::Global => agent_config.skills_dir.clone(),
                Scope::Project => {
                    if let Ok(project_root) = Config::find_project_root() {
                        agent_config
                            .project_skills_dir
                            .as_ref()
                            .map(|p| project_root.join(p))
                            .unwrap_or_else(|| {
                                project_root.join(Config::default_project_skills_dir())
                            })
                    } else {
                        continue;
                    }
                }
            };

            let skill_path = skills_dir.join(&args.name);
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
        // Use scope-aware resolution
        let (_config, skills_dir, scope) =
            Config::load_and_resolve(args.scope, args.agent.as_deref(), None)?;

        let scope_label = match scope {
            Scope::Global => "global",
            Scope::Project => "project",
        };

        let agent_name = args
            .agent
            .as_deref()
            .or(config.default_agent.as_deref())
            .unwrap_or("default");

        let skill_path = skills_dir.join(&args.name);

        if !skill_path.exists() {
            bail!(
                "Skill '{}' not found in {} ({}, scope: {})",
                args.name,
                agent_name,
                skills_dir.display(),
                scope_label
            );
        }

        // Verify it's a valid skill
        if let Err(e) = Skill::load(&skill_path) {
            println!("⚠ Warning: {} (removing anyway)", e);
        }

        if confirm_removal(
            &args.name,
            &format!("{} ({})", agent_name, scope_label),
            args.yes,
        )? {
            remove_skill_dir(&skill_path)?;
            println!(
                "✓ Removed '{}' from {} ({})",
                args.name, agent_name, scope_label
            );
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
