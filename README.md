<div align="center">

# 📦 Paks

**The package manager for AI Agent Skills**

Create, install, publish, and share skills across AI coding agents.

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
<a href="https://discord.gg/QTZjETP7GB"><img src="https://img.shields.io/badge/Discord-Join%20Community-5865F2?logo=discord&logoColor=white&style=flat-square" /></a>


[Installation](#installation) • [Quick Start](#quick-start) • [CLI Reference](#cli-reference) • [Registry](#registry) • [Contributing](#contributing)

⭐ Help us reach more developers and grow the Paks community. Star this repo!

</div>

---

## What is Paks?

Paks is a CLI-first package manager for [Agent Skills](https://agentskills.io) — reusable instruction sets that enhance AI coding agents like Stakpak, Claude Code, Cursor, OpenCode, GitHub Copilot, Goose, and more.

**The CLI** is your primary interface for:
- 🛠️ **Creating** new skills with proper structure
- 📥 **Installing** skills from the registry, git repos, or local paths
- 🚀 **Publishing** skills to share with the community
- 🔍 **Managing** skills across multiple AI agents

**The Web Registry** provides a browsable interface to discover and explore published skills.

## Why Paks?

AI coding agents are powerful, but they need context. Skills provide that context — coding standards, deployment procedures, API patterns, and domain knowledge. Paks makes it easy to:

- **Share expertise** — Package your team's best practices as installable skills
- **Stay consistent** — Install the same skills across all your AI agents
- **Build on others' work** — Discover and use community-created skills
- **Version and iterate** — Semantic versioning for skill updates

---

## Installation

### Homebrew (macOS & Linux)

```bash
brew tap stakpak/stakpak
brew install paks
```

### Download Binary

Pre-built binaries are available for all major platforms on the [Releases page](https://github.com/stakpak/paks/releases):

| Platform | Architecture | Download |
|----------|--------------|----------|
| macOS | Apple Silicon (M1/M2/M3) | `paks-darwin-aarch64.tar.gz` |
| macOS | Intel | `paks-darwin-x86_64.tar.gz` |
| Linux | x86_64 | `paks-linux-x86_64.tar.gz` |
| Linux | ARM64 | `paks-linux-aarch64.tar.gz` |
| Windows | x86_64 | `paks-windows-x86_64.zip` |

```bash
# Example: Download and install on Linux x86_64
curl -L https://github.com/stakpak/paks/releases/latest/download/paks-linux-x86_64.tar.gz | tar xz
sudo mv paks /usr/local/bin/
```

### From Source (Rust)

```bash
# Clone the repository
git clone https://github.com/stakpak/paks.git
cd paks

# Build the CLI
cargo build --release -p paks-cli

# The binary will be at ./target/release/paks
# Move it to your PATH
cp ./target/release/paks ~/.local/bin/
```

### Verify Installation

```bash
paks --version
paks --help
```

---

## Quick Start

### 1. Create Your First Skill

```bash
# Create a new skill
paks create my-awesome-skill

# Or with optional directories
paks create my-awesome-skill --with-scripts --with-references
```

This generates:

```
my-awesome-skill/
├── SKILL.md          # Skill manifest and instructions
├── scripts/          # (optional) Helper scripts
├── references/       # (optional) Reference documentation
└── assets/           # (optional) Static assets
```

### 2. Edit Your Skill

The `SKILL.md` file is the heart of your skill. It uses YAML frontmatter for metadata and Markdown for instructions:

```markdown
---
name: my-awesome-skill
description: A skill that helps with awesome things
version: 0.1.0
license: MIT
keywords:
  - awesome
  - productivity
---

# My Awesome Skill

## When to use this skill

Describe when this skill should be activated.

## Instructions

Add your instructions here. The AI agent will follow these
when the skill is active.
```

### 3. Validate Your Skill

```bash
paks validate my-awesome-skill

# Strict mode (warnings become errors)
paks validate my-awesome-skill --strict
```

### 4. Install Skills

```bash
# Install from registry
paks install kubernetes-deploy

# Install for a specific agent
paks install kubernetes-deploy --agent claude-code

# Install to project directory (instead of global)
paks install kubernetes-deploy --scope project

# Install from GitHub (just paste the URL from your browser)
paks install https://github.com/user/repo/tree/main/path/to/skill

# Install specific version
paks install kubernetes-deploy --version 1.2.0

# Force reinstall
paks install kubernetes-deploy --force
```

### 5. Publish Your Skill

```bash
# Dry run first
paks publish my-awesome-skill --dry-run

# Publish with version bump
paks publish my-awesome-skill --bump patch

# Publish (requires login)
paks login
paks publish my-awesome-skill
```

---

## CLI Reference

### Core Commands

| Command | Description |
|---------|-------------|
| `paks create <name>` | Create a new skill from template |
| `paks install <source>` | Install a skill |
| `paks publish [path]` | Publish a skill to the registry |
| `paks validate [path]` | Validate skill structure |
| `paks list` | List installed skills |
| `paks remove <name>` | Remove an installed skill |
| `paks search <query>` | Search the registry |
| `paks info <skill>` | Show skill details |

### Create Command

```bash
paks create <name> [OPTIONS]

Options:
  -o, --output <DIR>       Output directory (defaults to ./<name>)
  -t, --template <TYPE>    Template type: basic, devops, coding
      --with-scripts       Include scripts/ directory
      --with-references    Include references/ directory
      --with-assets        Include assets/ directory
```

**Examples:**

```bash
# Basic skill
paks create my-skill

# DevOps skill with all directories
paks create deploy-helper --template devops --with-scripts --with-references

# Specify output location
paks create my-skill --output ~/skills/my-skill
```

### Install Command

```bash
paks install <source> [OPTIONS]

Options:
  -a, --agent <AGENT>      Target agent (stakpak, claude-code, cursor, vscode, copilot, goose, opencode)
  -s, --scope <SCOPE>      Installation scope: global or project
  -d, --dir <PATH>         Custom install directory (overrides agent and scope)
  -v, --version <VERSION>  Specific version to install
  -f, --force              Force reinstall if exists
```

**Scope:**
- `global` (default): Installs to user directory (e.g., `~/.claude/skills`)
- `project`: Installs to project directory (e.g., `./.claude/skills`)

**Examples:**

```bash
# Install from registry (global scope by default)
paks install terraform-best-practices

# Install for Claude Code
paks install terraform-best-practices --agent claude-code

# Install to project directory (scope: project)
paks install terraform-best-practices --scope project

# Install from GitHub/GitLab (just paste the browser URL)
paks install https://github.com/org/repo/tree/main/skills/my-skill

# Install to custom directory
paks install my-skill --dir ~/custom/skills
```

### Publish Command

```bash
paks publish [path] [OPTIONS]

Options:
      --bump <LEVEL>       Version bump: patch, minor, major
      --skip-validation    Skip validation before publishing
      --dry-run            Show what would be published
```

**Examples:**

```bash
# Validate and show what would be published
paks publish --dry-run

# Publish with patch version bump (0.1.0 → 0.1.1)
paks publish --bump patch

# Publish with minor version bump (0.1.0 → 0.2.0)
paks publish --bump minor

# Publish specific directory
paks publish ./my-skill --bump patch
```

### List Command

```bash
paks list [OPTIONS]

Options:
  -a, --agent <AGENT>      List skills for specific agent
  -s, --scope <SCOPE>      Scope to list from: global or project
      --all                List skills from all agents
  -f, --format <FORMAT>    Output format: table, json, yaml
```

**Examples:**

```bash
# List skills for default agent (global scope by default)
paks list

# List project-scoped skills
paks list --scope project

# List all skills across all agents
paks list --all

# List as JSON
paks list --format json

# List for specific agent
paks list --agent cursor
```

### Agent Management

```bash
paks agent list              # List configured agents
paks agent add <name> -d <dir> [-p <project-dir>]  # Add custom agent
paks agent remove <name>     # Remove custom agent
paks agent default <name>    # Set default agent
paks agent show [name]       # Show agent details
```

**Built-in Agents:**

| Agent | Global Directory | Project Directory |
|-------|-----------------|-------------------|
| `stakpak` | `~/.stakpak/skills` | `.stakpak/skills` |
| `claude-code` | `~/.claude/skills` | `.claude/skills` |
| `cursor` | `~/.cursor/skills` | `.cursor/skills` |
| `vscode` | `~/.vscode/skills` | `.vscode/skills` |
| `copilot` | `~/.copilot/skills` | `.copilot/skills` |
| `goose` | `~/.config/goose/skills` | `.goose/skills` |
| `opencode` | `~/.config/opencode/skill` | `.opencode/skill` |

**Examples:**

```bash
# Add a custom agent with both global and project directories
paks agent add my-agent --dir ~/my-agent/skills --project-dir .my-agent/skills

# Set as default
paks agent default my-agent

# View all agents
paks agent show
```

### Authentication

```bash
paks login [--token <TOKEN>]  # Login to registry
paks logout                    # Logout from registry
```

---

## Skill Structure

### SKILL.md Frontmatter

The `SKILL.md` file uses YAML frontmatter following the [Agent Skills specification](https://agentskills.io):

```yaml
---
# Required fields
name: my-skill                    # 1-64 chars, lowercase + hyphens
description: What this skill does # 1-1024 chars

# Optional (Agent Skills spec)
license: MIT
compatibility: Requires Node.js 18+
metadata:
  author: Your Name
  website: https://example.com
allowed-tools: |                  # Experimental
  - read_file
  - write_file

# Paks extensions (for package management)
version: 1.0.0                    # Semantic version
authors:
  - Your Name <you@example.com>
repository: https://github.com/you/skill
homepage: https://your-skill.dev
keywords:
  - devops
  - kubernetes
categories:
  - deployment
dependencies:
  - name: base-skill
    version: ">=1.0.0"
---
```

### Directory Structure

```
my-skill/
├── SKILL.md              # Required: Manifest + instructions
├── scripts/              # Optional: Helper scripts
│   ├── deploy.sh
│   └── validate.py
├── references/           # Optional: Reference docs
│   ├── api-docs.md
│   └── examples/
└── assets/               # Optional: Static files
    └── templates/
```

---

## Registry

### Web Interface

The **Paks Registry** web interface provides:

- 🔍 **Browse** — Discover skills by category, keyword, or popularity
- 📖 **Read** — View skill documentation and instructions
- 📊 **Stats** — See download counts and version history
- 👤 **Profiles** — View publisher profiles and their skills

Visit the registry at: **[http://localhost:3001](http://localhost:3001)** (development)

### API

The registry exposes a REST API for programmatic access:

```bash
# Search skills
GET /api/skills?q=kubernetes

# Get skill details
GET /api/skills/:name

# Get specific version
GET /api/skills/:name/:version
```

---

## Configuration

### Global Configuration

Paks stores global configuration at `~/.paks/config.toml`:

```toml
# Default agent when --agent is not specified
default_agent = "stakpak"

# Default installation scope: "global" or "project"
default_scope = "global"

# Custom agents
[agents.my-custom-agent]
name = "My Custom Agent"
skills_dir = "/path/to/skills"
project_skills_dir = ".my-agent/skills"  # Relative to project root
description = "Custom agent for my workflow"

# Registry configuration
[registries.default]
url = "https://registry.paks.dev"
```

### Project Configuration

You can also create a project-level configuration at `.paks/config.toml` in your project root:

```toml
# Project-level config overrides global config
default_scope = "project"  # Make project scope the default for this project
default_agent = "claude-code"
```

**Configuration priority (highest to lowest):**
1. CLI flags (`--scope`, `--agent`, `--dir`)
2. Project config (`.paks/config.toml`)
3. Global config (`~/.paks/config.toml`)
4. Built-in defaults

This allows teams to standardize skill installations across all developers by committing `.paks/config.toml` to version control.

---

## Project Structure

```
paks/
├── apps/
│   ├── cli/              # Rust CLI application
│   │   ├── src/
│   │   │   ├── commands/ # CLI command implementations
│   │   │   │   ├── create.rs
│   │   │   │   ├── install.rs
│   │   │   │   ├── publish.rs
│   │   │   │   ├── validate.rs
│   │   │   │   └── ...
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   ├── web/              # Registry web interface
│   │   └── src/
│   │       ├── routes/   # TanStack Router pages
│   │       └── components/
│   └── docs/             # Documentation site
├── packages/
│   ├── core/             # Shared Rust library
│   └── config/           # Shared TypeScript config
├── Cargo.toml            # Rust workspace
├── package.json          # Node.js workspace
└── turbo.json            # Turborepo config
```

---

## Development

### Prerequisites

- **Rust** 2024 edition (nightly)
- **Node.js** 18+
- **pnpm** 8+

### Setup

```bash
# Install dependencies
pnpm install

# Build everything
pnpm run build

# Development mode
pnpm run dev
```

### Available Scripts

| Script | Description |
|--------|-------------|
| `pnpm run dev` | Start all apps in development mode |
| `pnpm run build` | Build all applications |
| `pnpm run dev:web` | Start only the web application |
| `pnpm run check-types` | TypeScript type checking |
| `pnpm run check` | Run Oxlint and Oxfmt |

### Rust Development

```bash
# Build CLI
cargo build -p paks-cli

# Run CLI
cargo run -p paks-cli -- --help

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

---

## Need Support Creating Your First Pak?
If this is your first time writing a Pak, you don’t have to figure it out alone.
Our community is actively building and sharing skills, and we’re happy to help you:
- Write your first SKILL.md
- Review structure and metadata
- Decide what belongs in a Pak (and what doesn’t)
- Validate and publish with confidence

Join the Stakpak Discord to ask questions, get feedback, and see how others are using Paks

Join the community: https://discord.gg/QTZjETP7GB

---

## Contributing

We welcome contributions! Here's how to get started:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Guidelines

- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed
- Keep commits atomic and well-described

---

## License

Apache 2.0 License — see [LICENSE](LICENSE) for details.

---

<div align="center">

**Built with ❤️ by [Stakpak](https://stakpak.dev)**

[Website](https://stakpak.dev) • [Documentation](https://docs.stakpak.dev) • [Discord](https://discord.gg/stakpak)

</div>
