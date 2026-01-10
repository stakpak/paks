//! List command - list installed skills

use anyhow::Result;
use std::path::Path;

use super::core::config::{Config, Scope};
use super::core::skill::Skill;

#[derive(Clone, Copy)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

pub struct ListArgs {
    pub agent: Option<String>,
    pub scope: Option<Scope>,
    pub all: bool,
    pub format: OutputFormat,
}

/// Skill info for listing
struct SkillInfo {
    name: String,
    version: String,
    description: String,
}

pub async fn run(args: ListArgs) -> Result<()> {
    let config = Config::load_merged()?;

    if args.all {
        // List from all agents (both global and project if scope not specified)
        println!("Installed skills:\n");

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

            let skills = list_skills_in_dir(&skills_dir);
            if !skills.is_empty() {
                println!("{} ({}):", agent_config.name, skills_dir.display());
                print_skills(&skills, args.format);
                println!();
            } else if skills_dir.exists() {
                println!("{}: (no skills installed)", id);
                println!();
            }
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

        println!(
            "Skills for {} ({}, scope: {}):\n",
            agent_name,
            skills_dir.display(),
            scope_label
        );
        let skills = list_skills_in_dir(&skills_dir);
        if skills.is_empty() {
            println!("  (no skills installed)");
        } else {
            print_skills(&skills, args.format);
        }
    }

    Ok(())
}

/// List all skills in a directory
fn list_skills_in_dir(dir: &Path) -> Vec<SkillInfo> {
    let mut skills = Vec::new();

    if !dir.exists() {
        return skills;
    }

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Ok(skill) = Skill::load(&path)
            {
                skills.push(SkillInfo {
                    name: skill.name().to_string(),
                    version: skill.version().to_string(),
                    description: skill.frontmatter.description.clone(),
                });
            }
        }
    }

    skills.sort_by(|a, b| a.name.cmp(&b.name));
    skills
}

/// Print skills in the specified format
fn print_skills(skills: &[SkillInfo], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            // Calculate column widths
            let name_width = skills
                .iter()
                .map(|s| s.name.len())
                .max()
                .unwrap_or(10)
                .max(10);
            let version_width = skills
                .iter()
                .map(|s| s.version.len())
                .max()
                .unwrap_or(7)
                .max(7);

            println!(
                "  {:<name_width$}  {:<version_width$}  DESCRIPTION",
                "NAME",
                "VERSION",
                name_width = name_width,
                version_width = version_width
            );
            println!(
                "  {:<name_width$}  {:<version_width$}  {}",
                "─".repeat(name_width),
                "─".repeat(version_width),
                "─".repeat(40),
                name_width = name_width,
                version_width = version_width
            );

            for skill in skills {
                let desc = if skill.description.len() > 50 {
                    format!("{}...", &skill.description[..47])
                } else {
                    skill.description.clone()
                };
                println!(
                    "  {:<name_width$}  {:<version_width$}  {}",
                    skill.name,
                    skill.version,
                    desc,
                    name_width = name_width,
                    version_width = version_width
                );
            }
        }
        OutputFormat::Json => {
            let json: Vec<_> = skills
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "name": s.name,
                        "version": s.version,
                        "description": s.description
                    })
                })
                .collect();
            println!(
                "{}",
                serde_json::to_string_pretty(&json).unwrap_or_default()
            );
        }
        OutputFormat::Yaml => {
            for skill in skills {
                println!("- name: {}", skill.name);
                println!("  version: {}", skill.version);
                println!("  description: {}", skill.description);
            }
        }
    }
}
