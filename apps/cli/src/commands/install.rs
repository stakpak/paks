//! Install command - install a skill to an agent's skills directory

use super::core::config::Config;
use anyhow::Result;

pub struct InstallArgs {
    pub source: String,
    pub agent: Option<String>,
    pub dir: Option<String>,
    pub version: Option<String>,
    pub force: bool,
}

pub async fn run(args: InstallArgs) -> Result<()> {
    // Determine install directory
    let install_dir = if let Some(dir) = args.dir {
        dir.into()
    } else if let Some(agent_name) = &args.agent {
        let config = Config::load()?;
        config
            .get_agent(agent_name)
            .map(|a| a.skills_dir.clone())
            .unwrap_or_else(Config::default_skills_dir)
    } else {
        // No agent specified - use ~/.paks/skills
        Config::default_skills_dir()
    };

    println!("Installing skill from: {}", args.source);
    println!("  Target: {}", install_dir.display());

    if let Some(v) = &args.version {
        println!("  Version: {}", v);
    }
    if args.force {
        println!("  Force: yes");
    }

    // TODO: Implement skill installation
    // 1. Parse source (registry name, git URL, or local path)
    // 2. Fetch skill from source
    // 3. Validate skill structure
    // 4. Copy to install directory
    // 5. Handle existing skill (--force)

    Ok(())
}
