use clap::{Parser, Subcommand};

mod commands;

use commands::{
    agent::AgentCommand,
    create::CreateArgs,
    info::InfoArgs,
    install::InstallArgs,
    list::{ListArgs, OutputFormat},
    login::LoginArgs,
    publish::{BumpLevel, PublishArgs},
    remove::RemoveArgs,
    search::SearchArgs,
    validate::ValidateArgs,
};

#[derive(Parser)]
#[command(name = "paks")]
#[command(version)]
#[command(about = "Agent Skills package manager - scaffold, install, and publish skills")]
#[command(
    long_about = "Paks is a CLI tool for managing Agent Skills (https://agentskills.io).

It helps you:
  • Create new skills with proper structure
  • Install skills from registries or git repos
  • Publish skills to share with others
  • Manage installed skills across different agents"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new skill from template
    #[command(alias = "new")]
    Create {
        /// Skill name (lowercase, hyphens allowed)
        name: String,

        /// Output directory (defaults to ./<name>)
        #[arg(short, long)]
        output: Option<String>,

        /// Use a specific template
        #[arg(short, long, default_value = "basic")]
        template: String,

        /// Include example scripts directory
        #[arg(long)]
        with_scripts: bool,

        /// Include references directory
        #[arg(long)]
        with_references: bool,

        /// Include assets directory
        #[arg(long)]
        with_assets: bool,
    },

    /// Install a skill to your agent's skills directory
    Install {
        /// Skill source (registry name, git URL, or local path)
        source: String,

        /// Target agent to install for
        #[arg(short, long, value_enum)]
        agent: Option<CliAgent>,

        /// Custom install directory (overrides agent default)
        #[arg(short, long)]
        dir: Option<String>,

        /// Specific version to install
        #[arg(short, long)]
        version: Option<String>,

        /// Force reinstall if already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Publish a skill to the registry
    Publish {
        /// Path to skill directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,

        /// Publish as a new version (auto-increment patch)
        #[arg(long)]
        bump: Option<CliBumpLevel>,

        /// Skip validation before publishing
        #[arg(long)]
        skip_validation: bool,

        /// Dry run - validate and show what would be published
        #[arg(long)]
        dry_run: bool,
    },

    /// List installed skills
    List {
        /// Target agent to list skills for
        #[arg(short, long, value_enum)]
        agent: Option<CliAgent>,

        /// Show skills from all configured agents
        #[arg(long)]
        all: bool,

        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: CliOutputFormat,
    },

    /// Remove an installed skill
    Remove {
        /// Skill name to remove
        name: String,

        /// Target agent to remove from
        #[arg(short, long, value_enum)]
        agent: Option<CliAgent>,

        /// Remove from all agents
        #[arg(long)]
        all: bool,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Validate a skill's structure and SKILL.md
    Validate {
        /// Path to skill directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,

        /// Strict mode - treat warnings as errors
        #[arg(long)]
        strict: bool,
    },

    /// Search for skills in the registry
    Search {
        /// Search query
        query: String,

        /// Maximum results to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Show details about a skill
    Info {
        /// Skill name or path
        skill: String,

        /// Show full SKILL.md content
        #[arg(long)]
        full: bool,
    },

    /// Login to the registry
    Login {
        /// API token (will prompt if not provided)
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Logout from the registry
    Logout,

    /// Manage agent configurations
    #[command(subcommand)]
    Agent(AgentCommands),
}

#[derive(Subcommand)]
enum AgentCommands {
    /// List configured agents
    List,

    /// Add a new agent configuration
    Add {
        /// Agent identifier
        name: String,

        /// Skills directory path
        #[arg(short, long)]
        dir: String,
    },

    /// Remove an agent configuration
    Remove {
        /// Agent identifier
        name: String,
    },

    /// Set the default agent
    Default {
        /// Agent identifier
        name: String,
    },

    /// Show agent configuration
    Show {
        /// Agent identifier (shows all if not specified)
        name: Option<String>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum CliAgent {
    /// Stakpak (default)
    Stakpak,
    /// Claude Code
    ClaudeCode,
    /// Cursor
    Cursor,
    /// VS Code with Copilot
    Vscode,
    /// GitHub Copilot
    Copilot,
    /// Goose
    Goose,
    /// OpenCode
    OpenCode,
    /// Custom (use --dir to specify)
    Custom,
}

impl std::fmt::Display for CliAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CliAgent::ClaudeCode => "claude-code",
            CliAgent::Cursor => "cursor",
            CliAgent::Vscode => "vscode",
            CliAgent::Copilot => "copilot",
            CliAgent::Goose => "goose",
            CliAgent::OpenCode => "opencode",
            CliAgent::Stakpak => "stakpak",
            CliAgent::Custom => "custom",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum CliOutputFormat {
    Table,
    Json,
    Yaml,
}

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum CliBumpLevel {
    Patch,
    Minor,
    Major,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            name,
            output,
            template,
            with_scripts,
            with_references,
            with_assets,
        } => {
            commands::create::run(CreateArgs {
                name,
                output,
                template,
                with_scripts,
                with_references,
                with_assets,
            })
            .await?;
        }

        Commands::Install {
            source,
            agent,
            dir,
            version,
            force,
        } => {
            commands::install::run(InstallArgs {
                source,
                agent: agent.map(|a| a.to_string()),
                dir,
                version,
                force,
            })
            .await?;
        }

        Commands::Publish {
            path,
            bump,
            skip_validation,
            dry_run,
        } => {
            commands::publish::run(PublishArgs {
                path,
                bump: bump.map(|b| match b {
                    CliBumpLevel::Patch => BumpLevel::Patch,
                    CliBumpLevel::Minor => BumpLevel::Minor,
                    CliBumpLevel::Major => BumpLevel::Major,
                }),
                skip_validation,
                dry_run,
            })
            .await?;
        }

        Commands::List { agent, all, format } => {
            commands::list::run(ListArgs {
                agent: agent.map(|a| a.to_string()),
                all,
                format: match format {
                    CliOutputFormat::Table => OutputFormat::Table,
                    CliOutputFormat::Json => OutputFormat::Json,
                    CliOutputFormat::Yaml => OutputFormat::Yaml,
                },
            })
            .await?;
        }

        Commands::Remove {
            name,
            agent,
            all,
            yes,
        } => {
            commands::remove::run(RemoveArgs {
                name,
                agent: agent.map(|a| a.to_string()),
                all,
                yes,
            })
            .await?;
        }

        Commands::Validate { path, strict } => {
            commands::validate::run(ValidateArgs { path, strict }).await?;
        }

        Commands::Search { query, limit } => {
            commands::search::run(SearchArgs { query, limit }).await?;
        }

        Commands::Info { skill, full } => {
            commands::info::run(InfoArgs { skill, full }).await?;
        }

        Commands::Login { token } => {
            commands::login::run_login(LoginArgs { token }).await?;
        }

        Commands::Logout => {
            commands::login::run_logout().await?;
        }

        Commands::Agent(cmd) => {
            let agent_cmd = match cmd {
                AgentCommands::List => AgentCommand::List,
                AgentCommands::Add { name, dir } => AgentCommand::Add { name, dir },
                AgentCommands::Remove { name } => AgentCommand::Remove { name },
                AgentCommands::Default { name } => AgentCommand::Default { name },
                AgentCommands::Show { name } => AgentCommand::Show { name },
            };
            commands::agent::run(agent_cmd).await?;
        }
    }

    Ok(())
}
