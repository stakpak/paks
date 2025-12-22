//! Create command - scaffold a new skill from template

use anyhow::{Result, bail};
use std::path::PathBuf;

use super::core::skill::Skill;

pub struct CreateArgs {
    pub name: String,
    pub output: Option<String>,
    pub template: String,
    pub with_scripts: bool,
    pub with_references: bool,
    pub with_assets: bool,
}

pub async fn run(args: CreateArgs) -> Result<()> {
    let output_dir: PathBuf = args.output.unwrap_or_else(|| args.name.clone()).into();

    // Check if directory already exists
    if output_dir.exists() {
        bail!(
            "Directory '{}' already exists. Use a different name or remove it first.",
            output_dir.display()
        );
    }

    // Create the skill with default description based on template
    let description = match args.template.as_str() {
        "basic" => format!("A skill that provides {} functionality", args.name),
        "devops" => format!("DevOps automation skill for {}", args.name),
        "coding" => format!("Coding assistance skill for {}", args.name),
        _ => format!("A skill for {}", args.name),
    };

    let skill = Skill::new(output_dir.clone(), &args.name, &description);

    // Validate the skill before creating
    skill.frontmatter.validate()?;

    // Create the output directory
    std::fs::create_dir_all(&output_dir)?;

    // Save the SKILL.md
    skill.save()?;

    println!(
        "✓ Created skill '{}' in '{}'",
        args.name,
        output_dir.display()
    );

    // Create optional directories
    if args.with_scripts {
        let scripts_dir = output_dir.join("scripts");
        std::fs::create_dir_all(&scripts_dir)?;
        // Create a placeholder script
        std::fs::write(
            scripts_dir.join("example.sh"),
            "#!/bin/bash\n# Example script for the skill\necho \"Hello from the skill!\"\n",
        )?;
        println!("  ✓ Created scripts/");
    }

    if args.with_references {
        let refs_dir = output_dir.join("references");
        std::fs::create_dir_all(&refs_dir)?;
        // Create a placeholder reference
        std::fs::write(
            refs_dir.join("README.md"),
            "# References\n\nAdd reference documentation here.\n",
        )?;
        println!("  ✓ Created references/");
    }

    if args.with_assets {
        let assets_dir = output_dir.join("assets");
        std::fs::create_dir_all(&assets_dir)?;
        std::fs::write(assets_dir.join(".gitkeep"), "")?;
        println!("  ✓ Created assets/");
    }

    println!("\nNext steps:");
    println!(
        "  1. Edit {}/SKILL.md to customize your skill",
        output_dir.display()
    );
    println!(
        "  2. Run 'paks validate {}' to check your skill",
        output_dir.display()
    );
    println!(
        "  3. Run 'paks publish {}' to share it",
        output_dir.display()
    );

    Ok(())
}
