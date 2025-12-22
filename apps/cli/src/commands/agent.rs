//! Agent command - manage agent configurations

use anyhow::{Result, bail};
use std::path::PathBuf;

use super::core::config::{AgentConfig, Config};

pub enum AgentCommand {
    List,
    Add { name: String, dir: String },
    Remove { name: String },
    Default { name: String },
    Show { name: Option<String> },
}

pub async fn run(cmd: AgentCommand) -> Result<()> {
    let mut config = Config::load()?;

    match cmd {
        AgentCommand::List => {
            println!("Configured agents:\n");

            // Preserve insertion order (stakpak first)
            for (id, agent) in &config.agents {
                let default_marker = if config.default_agent.as_ref() == Some(id) {
                    " (default)"
                } else {
                    ""
                };
                println!("  {}{}", id, default_marker);
                println!("    Name: {}", agent.name);
                println!("    Directory: {}", agent.skills_dir.display());
                if let Some(desc) = &agent.description {
                    println!("    Description: {}", desc);
                }
                println!();
            }
        }

        AgentCommand::Add { name, dir } => {
            // Validate name
            if name.is_empty() {
                bail!("Agent name cannot be empty");
            }

            if config.agents.contains_key(&name) {
                bail!(
                    "Agent '{}' already exists. Use 'paks agent remove {}' first.",
                    name,
                    name
                );
            }

            // Expand path
            let skills_dir: PathBuf = shellexpand::tilde(&dir).to_string().into();

            // Create the agent config
            let agent_config = AgentConfig {
                name: name.clone(),
                skills_dir: skills_dir.clone(),
                description: None,
            };

            config.agents.insert(name.clone(), agent_config);
            config.save()?;

            println!("✓ Added agent '{}'", name);
            println!("  Directory: {}", skills_dir.display());

            // Create directory if it doesn't exist
            if !skills_dir.exists() {
                std::fs::create_dir_all(&skills_dir)?;
                println!("  Created directory: {}", skills_dir.display());
            }
        }

        AgentCommand::Remove { name } => {
            // Check if it's a built-in agent
            let builtins = [
                "stakpak",
                "claude-code",
                "cursor",
                "vscode",
                "copilot",
                "goose",
                "opencode",
            ];
            if builtins.contains(&name.as_str()) {
                bail!(
                    "Cannot remove built-in agent '{}'. Built-in agents are always available.",
                    name
                );
            }

            if !config.agents.contains_key(&name) {
                bail!("Agent '{}' not found", name);
            }

            config.agents.shift_remove(&name);

            // Clear default if it was this agent
            if config.default_agent.as_ref() == Some(&name) {
                config.default_agent = None;
            }

            config.save()?;
            println!("✓ Removed agent '{}'", name);
        }

        AgentCommand::Default { name } => {
            if !config.agents.contains_key(&name) {
                bail!("Agent '{}' not found", name);
            }

            config.default_agent = Some(name.clone());
            config.save()?;

            println!("✓ Default agent set to '{}'", name);
        }

        AgentCommand::Show { name } => {
            if let Some(agent_name) = name {
                if let Some(agent) = config.get_agent(&agent_name) {
                    let is_default = config.default_agent.as_ref() == Some(&agent_name);
                    println!(
                        "Agent: {}{}",
                        agent_name,
                        if is_default { " (default)" } else { "" }
                    );
                    println!("  Name: {}", agent.name);
                    println!("  Directory: {}", agent.skills_dir.display());
                    if let Some(desc) = &agent.description {
                        println!("  Description: {}", desc);
                    }

                    // Show directory status
                    if agent.skills_dir.exists() {
                        if let Ok(entries) = std::fs::read_dir(&agent.skills_dir) {
                            let count = entries
                                .filter(|e| e.as_ref().map(|e| e.path().is_dir()).unwrap_or(false))
                                .count();
                            println!("  Skills installed: {}", count);
                        }
                    } else {
                        println!("  Directory: (not created)");
                    }
                } else {
                    bail!("Agent '{}' not found", agent_name);
                }
            } else {
                // Show all agents with details (preserve insertion order)
                for (id, agent) in &config.agents {
                    let is_default = config.default_agent.as_ref() == Some(id);
                    println!("{}{}:", id, if is_default { " (default)" } else { "" });
                    println!("  Name: {}", agent.name);
                    println!("  Directory: {}", agent.skills_dir.display());
                    if let Some(desc) = &agent.description {
                        println!("  Description: {}", desc);
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}
