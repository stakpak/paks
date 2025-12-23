# Paks CLI Roadmap

> Remaining tasks and features for the Paks CLI

## Status Legend

| Status | Meaning |
|--------|---------|
| ✅ | Complete |
| 🚧 | Partial / In Progress |
| ❌ | Not Started |

---

## Command Implementation Status

| Command | Status | Notes |
|---------|--------|-------|
| `create` | ✅ | Fully implemented |
| `validate` | ✅ | Fully implemented |
| `list` | ✅ | Fully implemented |
| `remove` | ✅ | Fully implemented |
| `agent` | ✅ | Fully implemented |
| `info` | 🚧 | Local only - registry lookup missing |
| `install` | ✅ | Registry, git, and local installation |
| `search` | ❌ | Stub only |
| `publish` | 🚧 | Validation works, git tag flow missing |
| `login` | ❌ | Stub only |
| `logout` | ❌ | Stub only |

---

## Remaining Tasks

### 1. Authentication System

**Priority:** High  
**Status:** ❌ Not Started  
**Files:** `apps/cli/src/commands/login.rs`

#### Overview
Users authenticate by providing a **Stakpak API token** directly. Tokens are generated from the Stakpak web dashboard and passed to the CLI. No OAuth flows or browser interactions—just simple token-based auth.

#### Tasks

- [ ] **Token-based Login**
  - [ ] Accept token via `--token` flag: `paks login --token stk_xxxxxxxxxxxx`
  - [ ] Prompt for token interactively if `--token` not provided:
    ```
    $ paks login
    Enter your Stakpak API token: ▊
    
    (Get your token at https://stakpak.dev/settings/tokens)
    ```
  - [ ] Support `PAKS_TOKEN` environment variable for CI/CD
  - [ ] Support `STAKPAK_TOKEN` as alias
  - [ ] Mask token input in terminal (don't echo characters)
  - [ ] Validate token format before storing (basic sanity check)

- [ ] **Token Validation**
  - [ ] Validate token against Stakpak API on login: `GET https://api.stakpak.dev/auth/verify`
    ```json
    Request Headers:
      Authorization: Bearer stk_xxxxxxxxxxxx
    
    Response (200 OK):
    {
      "valid": true,
      "user": {
        "id": "user_xxx",
        "username": "johndoe",
        "email": "john@example.com",
        "avatar_url": "https://..."
      },
      "expires_at": "2024-12-31T23:59:59Z"  // null if no expiry
    }
    
    Response (401 Unauthorized):
    {
      "error": {
        "code": "INVALID_TOKEN",
        "message": "Token is invalid or expired"
      }
    }
    ```
  - [ ] Reject invalid tokens with clear error message
  - [ ] Show success message with username on valid login
  - [ ] Check token expiration before operations (if token has expiry)
  - [ ] Warn when token expires soon (< 7 days)

- [ ] **Secure Token Storage**
  - [ ] **macOS**: Use Keychain Services via `keyring` crate
    - Service: `dev.stakpak.paks`
    - Account: `api_token`
  - [ ] **Linux**: Use Secret Service API (GNOME Keyring, KWallet) via `keyring` crate
  - [ ] **Windows**: Use Windows Credential Manager via `keyring` crate
  - [ ] **Fallback**: Encrypted file at `~/.config/paks/credentials` with user-only permissions (0600)
  - [ ] Never store tokens in plain text config files
  - [ ] Store token metadata (user info, expiry) in config for quick access

- [ ] **Logout Implementation**
  - [ ] Remove token from secure storage
  - [ ] Clear cached user data from config
  - [ ] Confirm logout to user
  - [ ] Note: Token is NOT revoked server-side (user can revoke from dashboard)

- [ ] **User Info Command**
  - [ ] Add `paks whoami` command
  - [ ] If token stored locally, fetch fresh data: `GET https://api.stakpak.dev/auth/me`
  - [ ] Display: username, email, account status
  - [ ] Show authentication status (logged in/out)
  - [ ] Show token expiration time (if applicable)
  - [ ] If not logged in, show helpful message:
    ```
    Not logged in.
    Run 'paks login' to authenticate.
    ```

- [ ] **Error Handling**
  - [ ] Invalid token: "Invalid token. Check your token at https://stakpak.dev/settings/tokens"
  - [ ] Expired token: "Token expired. Generate a new token at https://stakpak.dev/settings/tokens"
  - [ ] Network error: "Could not reach Stakpak API. Check your connection."
  - [ ] Storage error: "Could not save token securely. Check system keychain."

#### CLI Interface

```bash
# Login with token flag
paks login --token stk_xxxxxxxxxxxx

# Login interactively (prompts for token)
paks login

# Login via environment variable (for CI/CD)
export PAKS_TOKEN=stk_xxxxxxxxxxxx
paks publish --bump patch

# Check current user
paks whoami

# Logout (removes stored token)
paks logout
```

#### Example Output

```
$ paks login --token stk_xxxxxxxxxxxx

Validating token... ✓
Logged in as johndoe (john@example.com)

$ paks login

Enter your Stakpak API token: ••••••••••••••••
(Get your token at https://stakpak.dev/settings/tokens)

Validating token... ✓
Logged in as johndoe (john@example.com)

$ paks whoami

Username: johndoe
Email:    john@example.com
Status:   Authenticated
Expires:  Never

$ paks logout

Logged out successfully.
```

---

### 2. Publish (Git-based Release Flow)

**Priority:** High  
**Status:** 🚧 Partial  
**Files:** `apps/cli/src/commands/publish.rs`

#### Overview
Publishing is **git-based**. The registry doesn't store skill files directly—it indexes skills by fetching data from git tags. When a user publishes, the CLI:
1. Validates the skill
2. Bumps version (optional)
3. Commits changes
4. Creates a git tag
5. Pushes to remote
6. Notifies registry to index the new version

#### Completed
- [x] Skill validation before publish
- [x] Version bumping (patch/minor/major)
- [x] Dry-run mode
- [x] File listing for package

#### Tasks

- [ ] **Git Repository Validation**
  - [ ] Check if current directory is inside a git repository
    ```rust
    // Use git2 crate or shell out to `git rev-parse --is-inside-work-tree`
    ```
  - [ ] Verify `.git` directory exists and is valid
  - [ ] Check for uncommitted changes (staged and unstaged)
    ```bash
    git status --porcelain
    ```
  - [ ] Warn user if working directory is dirty, offer to continue or abort
  - [ ] Verify at least one remote is configured
    ```bash
    git remote -v
    ```
  - [ ] Detect default remote (prefer `origin`, fallback to first available)
  - [ ] Validate remote URL is accessible (optional network check)
  - [ ] Check user has push permissions (may require auth check)

- [ ] **Version Tag Management**
  - [ ] Parse current version from `SKILL.md` frontmatter
  - [ ] Check if tag already exists locally: `git tag -l v{version}`
  - [ ] Check if tag exists on remote: `git ls-remote --tags origin v{version}`
  - [ ] If tag exists, error with helpful message:
    ```
    Error: Tag v1.2.3 already exists.
    Hint: Use --bump to increment version, or delete the existing tag.
    ```
  - [ ] Create annotated tag with metadata:
    ```bash
    git tag -a v1.2.3 -m "Release v1.2.3

    Skill: my-awesome-skill
    Description: A skill that does awesome things
    Published: 2024-01-15T10:30:00Z"
    ```
  - [ ] Only support semantic versioning with `v` prefix (e.g., `v1.2.3`)
    - No custom tag prefixes - standardized format only
    - Version must follow semver: `vMAJOR.MINOR.PATCH[-PRERELEASE]`
    - Prerelease suffixes allowed: `-alpha`, `-beta`, `-rc.N` (e.g., `v1.2.3-beta`, `v2.0.0-rc.1`)
  - [ ] Support lightweight tags via `--lightweight` flag (not recommended)

- [ ] **Git Operations Workflow**
  1. [ ] **Pre-flight checks**
     - Validate skill structure
     - Check git status
     - Verify remote access
  2. [ ] **Version bump** (if `--bump` specified)
     - Update version in `SKILL.md` frontmatter
     - Stage the change: `git add SKILL.md`
     - Commit with message: `chore: bump version to {version}`
  3. [ ] **Create tag**
     - Create annotated tag: `git tag -a v{version} -m "{message}"`
  4. [ ] **Push changes**
     - Push commit (if version was bumped): `git push origin {branch}`
     - Push tag: `git push origin v{version}`
  5. [ ] **Registry notification**
     - POST to registry API to trigger indexing

- [ ] **Registry Notification** (handled via GitHub App webhook - no CLI action needed)
  - [ ] Registry automatically indexes new tags via GitHub webhook
  - [ ] CLI just needs to push the tag; registry picks it up
  - [ ] Show registry URL where skill will be available after indexing

- [ ] **Pre-publish Validation**
  - [ ] Validate `SKILL.md` exists and has valid frontmatter
  - [ ] Required fields check: `name`, `description`, `version`
  - [ ] Version must follow semver (MAJOR.MINOR.PATCH)
  - [ ] Name must match directory name (optional, configurable)
  - [ ] Check for `LICENSE` file (warn if missing)
  - [ ] Check for `README.md` (warn if missing, SKILL.md is primary)
  - [ ] Validate no secrets in skill files (basic pattern matching)

- [ ] **Error Handling**
  - [ ] **Tag exists**: Suggest version bump or manual tag deletion
  - [ ] **Push rejected**: Check for force push needs, upstream changes
  - [ ] **Auth failed**: Prompt for re-authentication
  - [ ] **Network error**: Retry with exponential backoff
  - [ ] **Dirty working directory**: List changed files, offer to stash
  - [ ] **No remote**: Guide user to add remote
  - [ ] **Detached HEAD**: Warn user, suggest checking out a branch

- [ ] **Dry-run Mode Enhancements**
  - [ ] Show exact git commands that would be executed
  - [ ] Show tag name and message that would be created
  - [ ] Show files that would be included in the skill
  - [ ] Validate everything without making changes
  - [ ] Output in a format that can be copy-pasted to run manually

- [ ] **Additional Flags**
  - [ ] `--no-push`: Create tag locally without pushing (for review)
  - [ ] `--no-commit`: Skip commit, only create tag (assumes manual commit)
  - [ ] `--message <msg>`: Custom tag message
  - [ ] `--force`: Force push tag (dangerous, requires confirmation)
  - [ ] `--skip-registry`: Don't notify registry (for private skills)
  - [ ] `--branch <branch>`: Specify branch to push to (default: current)

#### CLI Interface

```bash
# Dry run - see what would happen
paks publish --dry-run

# Publish with patch version bump (0.1.0 → 0.1.1)
paks publish --bump patch

# Publish with custom message
paks publish --bump minor --message "Added new feature X"

# Publish without pushing (create local tag only)
paks publish --bump patch --no-push

# Publish from subdirectory (monorepo)
paks publish ./skills/my-skill --bump patch
```

#### Example Output

```
Publishing skill: my-awesome-skill

Pre-flight checks:
  ✓ Git repository detected
  ✓ Working directory clean
  ✓ Remote 'origin' configured (https://github.com/user/skill-repo)
  ✓ SKILL.md valid

Version: 0.1.0 → 0.1.1 (patch bump)

Git operations:
  → Updating SKILL.md version...
  → Committing: chore: bump version to 0.1.1
  → Creating tag: v0.1.1
  → Pushing to origin/main...
  → Pushing tag v0.1.1...

Registry:
  → Notifying registry to index v0.1.1...
  ✓ Skill indexed successfully

✓ Published my-awesome-skill@0.1.1
  https://paks.dev/skills/my-awesome-skill
```

---

### 3. Skill Installation

**Priority:** High  
**Status:** ✅ Complete  
**Files:** `apps/cli/src/commands/install.rs`

#### Overview
Install skills from multiple sources: the paks registry, git repositories, or local paths. Skills are copied to the target agent's skills directory.

**Skills are namespaced by account**: `account_name/skill_name` (e.g., `stakpak/kubernetes-deploy`).

#### Completed
- [x] Target directory resolution (agent-based or custom)
- [x] CLI argument parsing
- [x] SkillRef parsing with version support (`account/skill[@version]`)
- [x] Source type auto-detection (registry, git URL, local path)
- [x] Registry installation via paks-api client
- [x] Git installation with tag/branch/commit support
- [x] Local path installation with recursive copy
- [x] Flat directory naming (`owner--skill`) for agent compatibility
- [x] Force reinstall with `--force` flag

#### Tasks

- [x] **Namespaced Skill Names**
  - [x] Skills are identified as `account_name/skill_name`
    - Examples: `stakpak/kubernetes-deploy`, `johndoe/terraform-aws`
  - [x] Parse namespace from skill identifier:
    ```rust
    struct SkillRef {
        account: String,    // "stakpak"
        name: String,       // "kubernetes-deploy"
        version: Option<String>,  // "1.2.3"
    }
    ```
  - [x] Validate namespace format:
    - Format: `{account}/{skill}[@version]`
    - Account name: lowercase alphanumeric + hyphens, 1-39 chars
    - Skill name: lowercase alphanumeric + hyphens, 1-64 chars
  - [x] Registry API uses namespaced paths: `GET /v1/paks/install/{account}/{name}`
  - [x] Install directory structure: `~/.agents/skills/owner--skill/` (flat, agent-friendly)
  - [x] Handle collisions: same skill name under different accounts is allowed (via `owner--skill` naming)

- [x] **Source Detection & Parsing**
  - [x] **Registry name**: Namespaced identifier like `stakpak/kubernetes-deploy`
    - Format: `{account}/{skill}[@version]` (contains exactly one `/`)
    - Query registry API: `GET /v1/paks/install/{account}/{name}`
  - [x] **Git URL**: Full repository URL
    - HTTPS: `https://github.com/user/skill-repo.git`
    - SSH: `git@github.com:user/skill-repo.git`
    - With subdirectory: `url#path=skills/my-skill`
    - With ref: `url#ref=v1.2.3` or `url#tag=v1.2.3` or `url#branch=main`
  - [x] **Local path**: Filesystem path
    - Relative: `./my-skill`, `../other-skill`
    - Absolute: `/home/user/skills/my-skill`
  - [x] Auto-detect source type from input string

- [x] **Registry Installation**
  - [x] Use install API: `GET /v1/paks/install/{owner}/{pak_name}[@version]`
  - [x] API returns all metadata needed for installation
  - [x] API automatically records download event (no separate tracking needed)
  - [x] Resolve version (inline syntax or latest)
  - [x] Clone from `repository.clone_url` at `version.tag`
  - [x] Extract `install.path` to target directory

- [x] **Git Installation**
  - [x] Clone repository to temporary directory (shallow clone)
  - [x] Checkout specific ref via URL fragment (`#ref=`, `#tag=`, `#branch=`)
  - [x] Handle monorepo structure via `#path=` fragment
  - [x] Validate skill structure after checkout
  - [x] Copy skill directory to target location
  - [x] Clean up temporary clone

- [x] **Local Installation**
  - [x] Validate source path exists
  - [x] Validate it's a valid skill (has SKILL.md)
  - [x] Copy directory recursively to target
  - [x] Handle symlinks (copy as symlinks on Unix)

- [x] **Installation Behavior**
  - [x] **Default**: Error if skill already installed
  - [x] **`--force`**: Remove existing and reinstall
  - [ ] **`--upgrade`**: Install only if newer version available (future)
  - [x] Check installed version from existing SKILL.md
  - [x] Show version comparison when upgrading

- [ ] **Post-install Hooks** (optional, future)
  - [ ] Run `scripts/post-install.sh` if exists
  - [ ] Support `post_install` field in frontmatter
  - [ ] Sandbox script execution for security

- [ ] **Progress & Output** (future enhancement)
  - [ ] Show download/clone progress with progress bar (indicatif crate)
  - [ ] Spinner for operations without known size (git clone, API calls)
  - [ ] List files being installed
  - [ ] Show final installation path
  - [ ] Warn about overwritten files

#### CLI Interface

```bash
# Install from registry (latest version) - namespaced
paks install stakpak/kubernetes-deploy

# Install specific version
paks install stakpak/kubernetes-deploy@1.2.3

# Install for specific agent
paks install stakpak/kubernetes-deploy --agent claude-code

# Install from git
paks install https://github.com/user/skill-repo.git

# Install from git with specific tag
paks install https://github.com/user/skill-repo.git --tag v1.0.0

# Install from monorepo
paks install https://github.com/org/skills-monorepo.git --path skills/kubernetes

# Install from local path
paks install ./my-local-skill

# Force reinstall
paks install stakpak/kubernetes-deploy --force
```

---

### 4. Registry Search

**Priority:** Medium  
**Status:** ❌ Not Started  
**Files:** `apps/cli/src/commands/search.rs`

#### Overview
Search the paks registry for skills by name, keyword, category, or author.

#### Tasks

- [ ] **Registry API Integration**
  - [ ] Query endpoint: `GET /api/skills/search?q={query}`
  - [ ] Support query parameters:
    - `q`: Full-text search query
    - `category`: Filter by category
    - `keyword`: Filter by keyword
    - `author`: Filter by author/publisher
    - `sort`: Sort order (downloads, recent, name, relevance)
    - `limit`: Results per page (default: 20)
    - `offset`: Pagination offset
  - [ ] Handle empty results gracefully
  - [ ] Handle API errors with helpful messages

- [ ] **Results Display**
  - [ ] Table format (default):
    ```
    SKILL                          VERSION  DOWNLOADS  DESCRIPTION
    ───────────────────────────────────────────────────────────────────────
    stakpak/kubernetes-deploy      2.1.0    1,234      Deploy apps to Kubernetes...
    stakpak/terraform-aws          1.5.2    892        Terraform best practices...
    johndoe/docker-compose         1.0.0    567        Docker Compose workflows...
    ```
  - [ ] JSON format (`--format json`) for scripting
  - [ ] YAML format (`--format yaml`)
  - [ ] Detailed format (`--detailed`) with full descriptions

- [ ] **Filter Options**
  - [ ] `--category <cat>`: Filter by category (devops, coding, security, etc.)
  - [ ] `--keyword <kw>`: Filter by keyword tag
  - [ ] `--author <name>`: Filter by publisher
  - [ ] Multiple filters combine with AND logic

- [ ] **Sort Options**
  - [ ] `--sort downloads`: Most downloaded first (default)
  - [ ] `--sort recent`: Recently updated first
  - [ ] `--sort name`: Alphabetical
  - [ ] `--sort relevance`: Best match for query

- [ ] **Pagination**
  - [ ] `--limit <n>`: Number of results (default: 20, max: 100)
  - [ ] `--page <n>`: Page number for pagination
  - [ ] Show total count and current page info

#### CLI Interface

```bash
# Basic search
paks search kubernetes

# Search with filters
paks search deploy --category devops --sort downloads

# Search by account (all skills from an account)
paks search --account stakpak

# Output as JSON
paks search terraform --format json

# Paginated results
paks search docker --limit 10 --page 2
```

---

### 5. Registry Info Lookup

**Priority:** Medium  
**Status:** 🚧 Partial  
**Files:** `apps/cli/src/commands/info.rs`

#### Completed
- [x] Local skill info display
- [x] Full content display with `--full`

#### Tasks

- [ ] **Registry Lookup**
  - [ ] Detect if input is local path or registry name
  - [ ] Fetch from registry: `GET /api/skills/{name}`
  - [ ] Fetch specific version: `GET /api/skills/{name}/versions/{version}`
  - [ ] Cache responses briefly to avoid repeated requests

- [ ] **Display Registry Metadata**
  - [ ] All local fields plus:
    - Download count (total and recent)
    - First published date
    - Last updated date
    - Publisher info (name, profile URL)
    - Repository URL
    - All available versions with dates
  - [ ] Show installation status:
    ```
    Status: Installed (v1.2.0) - Update available (v1.3.0)
    ```

- [ ] **Version Listing**
  - [ ] `paks info skill-name --versions`: List all versions
  - [ ] Show version, date, and changelog summary

#### CLI Interface

```bash
# Info for local skill
paks info ./my-skill

# Info from registry (namespaced)
paks info stakpak/kubernetes-deploy

# Specific version info
paks info stakpak/kubernetes-deploy --version 1.2.0

# List all versions
paks info stakpak/kubernetes-deploy --versions

# Full content
paks info stakpak/kubernetes-deploy --full
```

---

### 6. Registry API Client

**Priority:** High  
**Status:** ❌ Not Started  
**Files:** New module `packages/core/src/registry/` or `apps/cli/src/registry/`

#### Overview
Shared HTTP client for all registry interactions. Handles authentication, retries, and error mapping.

#### Tasks

- [ ] **Client Structure**
  ```rust
  pub struct RegistryClient {
      base_url: String,
      http_client: reqwest::Client,
      auth_token: Option<String>,
  }
  ```

- [ ] **Configuration**
  - [ ] Base URL from config (default: `https://api.paks.dev`)
  - [ ] Support custom registries
  - [ ] Timeout configuration (default: 30s)
  - [ ] User-Agent header: `paks-cli/{version}`

- [ ] **Authentication**
  - [ ] Load token from secure storage
  - [ ] Inject `Authorization: Bearer {token}` header
  - [ ] Handle 401 responses (prompt for re-auth)
  - [ ] Support anonymous requests for public endpoints

- [ ] **API Endpoints**
  ```rust
  impl RegistryClient {
      // Skills
      async fn search_skills(&self, query: SearchQuery) -> Result<SearchResults>;
      async fn get_skill(&self, name: &str) -> Result<SkillMetadata>;
      async fn get_skill_version(&self, name: &str, version: &str) -> Result<SkillVersion>;
      async fn list_versions(&self, name: &str) -> Result<Vec<VersionInfo>>;
      
      // Publishing
      async fn notify_publish(&self, repo: &str, tag: &str) -> Result<()>;
      
      // Auth
      async fn verify_token(&self) -> Result<UserInfo>;
      async fn get_current_user(&self) -> Result<UserInfo>;
  }
  ```

- [ ] **Response Types**
  ```rust
  #[derive(Deserialize)]
  pub struct SkillMetadata {
      pub name: String,
      pub description: String,
      pub version: String,
      pub repository: String,
      pub author: AuthorInfo,
      pub downloads: u64,
      pub created_at: DateTime<Utc>,
      pub updated_at: DateTime<Utc>,
      pub keywords: Vec<String>,
      pub categories: Vec<String>,
  }
  ```

- [ ] **Error Handling**
  - [ ] Map HTTP status codes to typed errors
  - [ ] Parse error responses from API
  - [ ] Retry transient failures (5xx, timeouts)
  - [ ] Exponential backoff with jitter

- [ ] **Rate Limiting**
  - [ ] Respect `Retry-After` header
  - [ ] Implement client-side rate limiting
  - [ ] Show helpful message when rate limited

- [ ] **Caching** (optional)
  - [ ] Cache skill metadata briefly (5 min)
  - [ ] Cache search results (1 min)
  - [ ] Invalidate on publish
  - [ ] `--no-cache` flag to bypass

---

### 7. Configuration Enhancements

**Priority:** Low  
**Status:** 🚧 Partial  
**Files:** `apps/cli/src/commands/core/config.rs`

#### Tasks

- [ ] **Multiple Registry Support**
  ```toml
  [registries]
  default = "paks"
  
  [registries.paks]
  url = "https://api.paks.dev"
  
  [registries.private]
  url = "https://skills.mycompany.com"
  auth_required = true
  ```
  - [ ] `paks registry add <name> --url <url>`
  - [ ] `paks registry remove <name>`
  - [ ] `paks registry list`
  - [ ] `paks registry default <name>`
  - [ ] Install from specific registry: `paks install skill --registry private`

- [ ] **Proxy Configuration**
  - [ ] HTTP_PROXY / HTTPS_PROXY environment variables
  - [ ] Config file proxy settings
  - [ ] No-proxy list

- [ ] **Offline Mode**
  - [ ] `offline = true` in config
  - [ ] `--offline` flag
  - [ ] Use cached data only
  - [ ] Clear error when network required

- [ ] **Cache Configuration**
  - [ ] Custom cache directory
  - [ ] Cache TTL settings
  - [ ] Max cache size

---

### 8. Installation Lockfile

**Priority:** Medium  
**Status:** ❌ Not Started  
**Files:** `apps/cli/src/commands/core/lockfile.rs`

#### Overview
Track all installed skills across multiple agents in a global lockfile. This enables:
- Knowing where each skill is installed (which agent/directory)
- Tracking installation source (registry, git, local)
- Version tracking for updates
- Supporting `paks list --all` across all agents

#### Lockfile Location
`~/.config/paks/paks-lock.toml`

#### Lockfile Structure
```toml
# Global lockfile for tracking all installed skills
# Auto-generated by paks - do not edit manually

[[installations]]
skill = "stakpak/kubernetes-deploy"
version = "1.2.3"
source = "registry"                    # registry | git | local
source_uri = "stakpak/kubernetes-deploy@1.2.3"
installed_at = "2024-01-15T10:30:00Z"
agent = "claude-code"                  # agent name or null for custom dir
install_dir = "/Users/user/.claude/skills/stakpak/kubernetes-deploy"

[[installations]]
skill = "my-local-skill"
version = "0.1.0"
source = "local"
source_uri = "/path/to/my-local-skill"
installed_at = "2024-01-16T14:00:00Z"
agent = "stakpak"
install_dir = "/Users/user/.stakpak/skills/my-local-skill"

[[installations]]
skill = "custom-skill"
version = "2.0.0"
source = "git"
source_uri = "https://github.com/user/skill-repo.git#tag=v2.0.0"
installed_at = "2024-01-17T09:15:00Z"
agent = null                           # custom directory, no agent
install_dir = "/custom/path/skills/custom-skill"
```

#### Tasks

- [ ] **Lockfile Data Structure**
  ```rust
  #[derive(Serialize, Deserialize)]
  pub struct Lockfile {
      pub installations: Vec<Installation>,
  }
  
  #[derive(Serialize, Deserialize)]
  pub struct Installation {
      pub skill: String,           // skill name or account/skill
      pub version: String,
      pub source: InstallSource,   // Registry, Git, Local
      pub source_uri: String,      // original source reference
      pub installed_at: DateTime<Utc>,
      pub agent: Option<String>,   // agent name or None for custom
      pub install_dir: PathBuf,
  }
  
  #[derive(Serialize, Deserialize)]
  pub enum InstallSource {
      Registry,
      Git,
      Local,
  }
  ```

- [ ] **Lockfile Operations**
  - [ ] `Lockfile::load()` - Load from disk or create empty
  - [ ] `Lockfile::save()` - Write to disk atomically
  - [ ] `Lockfile::add_installation()` - Record new installation
  - [ ] `Lockfile::remove_installation()` - Remove on uninstall
  - [ ] `Lockfile::find_by_skill()` - Find all installations of a skill
  - [ ] `Lockfile::find_by_agent()` - List skills for an agent
  - [ ] `Lockfile::find_by_dir()` - Find installation by directory

- [ ] **Integration with Install Command**
  - [ ] Record installation in lockfile after successful install
  - [ ] Check lockfile before install to detect existing installations
  - [ ] Update lockfile entry on `--force` reinstall

- [ ] **Integration with Remove Command**
  - [ ] Remove entry from lockfile when skill is uninstalled
  - [ ] Use lockfile to find skill location if not specified

- [ ] **Integration with List Command**
  - [ ] Use lockfile for `paks list --all` (faster than scanning dirs)
  - [ ] Show source info (registry/git/local) in listing
  - [ ] Cross-reference lockfile with actual directories (detect orphans)

- [ ] **Lockfile Maintenance**
  - [ ] `paks lock sync` - Reconcile lockfile with actual installations
  - [ ] Detect and report orphaned entries (lockfile entry but no dir)
  - [ ] Detect and report untracked skills (dir exists but no lockfile entry)
  - [ ] Auto-repair option to fix inconsistencies

- [ ] **Edge Cases**
  - [ ] Handle concurrent access (file locking)
  - [ ] Graceful handling of corrupted lockfile
  - [ ] Migration from older lockfile versions
  - [ ] Handle moved/renamed directories

---

### 9. Additional Features (Nice to Have)

**Priority:** Low  
**Status:** ❌ Not Started

#### Tasks

- [ ] **`paks update`**: Update all installed skills to latest versions
  - [ ] Check each installed skill against registry
  - [ ] Show available updates
  - [ ] `--dry-run` to preview
  - [ ] `--all` to update everything

- [ ] **`paks outdated`**: Show skills with available updates
  - [ ] Table: skill name, current version, latest version
  - [ ] Exit code 1 if outdated (for CI)

- [ ] **`paks cache clean`**: Clear download cache
  - [ ] Show cache size before clearing
  - [ ] `--dry-run` to preview

- [ ] **`paks doctor`**: Diagnose common issues
  - [ ] Check git installation
  - [ ] Verify authentication
  - [ ] Test registry connectivity
  - [ ] Validate config file
  - [ ] Check agent directories

- [ ] **`paks init`**: Initialize skill in current directory
  - [ ] Interactive prompts for metadata
  - [ ] Create SKILL.md from template
  - [ ] Optionally create directories

- [ ] **Shell Completions**
  - [ ] Generate completions: `paks completions bash|zsh|fish|powershell`
  - [ ] Complete command names
  - [ ] Complete skill names from registry
  - [ ] Complete agent names

- [ ] **UX Improvements**
  - [ ] Progress bars for downloads (indicatif crate)
  - [ ] Colored output with `--color auto|always|never`
  - [ ] Verbose mode `-v` / `--verbose`
  - [ ] Quiet mode `-q` / `--quiet`
  - [ ] JSON output for all commands `--json`

---

## Implementation Order (Suggested)

```
Phase 1: Core Infrastructure
├── 1. Registry API Client (foundation for everything)
└── 2. Authentication System (required for publishing)

Phase 2: Publishing Flow
└── 3. Publish Command (git-based release flow)

Phase 3: Installation
├── 4. Install from Registry
├── 5. Install from Git
└── 6. Install from Local Path

Phase 4: Discovery
├── 7. Search Command
└── 8. Info Command (registry lookup)

Phase 5: Polish
├── 9. Configuration Enhancements
└── 10. Additional Features (update, outdated, doctor, etc.)
```

---

## Dependencies

### Rust Crates to Add

```toml
[dependencies]
# HTTP client with async support
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# Async runtime (if not already present)
tokio = { version = "1", features = ["full"] }

# Git operations
git2 = "0.19"

# Secure credential storage
keyring = "3"

# Progress bars and spinners
indicatif = "0.17"

# Colored terminal output
colored = "2"

# Semver parsing and comparison
semver = "1"

# URL parsing
url = "2"

# Temporary directories
tempfile = "3"

# File system operations
fs_extra = "1.3"

# Date/time handling
chrono = { version = "0.4", features = ["serde"] }
```

---

## API Specification (Draft)

### Endpoints

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| GET | `/api/skills` | List/search skills | No |
| GET | `/api/skills/{name}` | Get skill details | No |
| GET | `/api/skills/{name}/versions` | List all versions | No |
| GET | `/api/skills/{name}/versions/{version}` | Get specific version | No |
| POST | `/api/skills/index` | Trigger indexing from git | Yes |
| GET | `/api/auth/verify` | Verify token | Yes |
| GET | `/api/auth/me` | Get current user | Yes |
| POST | `/api/auth/revoke` | Revoke token | Yes |

### Response Format

```json
{
  "data": { ... },
  "meta": {
    "total": 100,
    "page": 1,
    "per_page": 20
  }
}
```

### Error Format

```json
{
  "error": {
    "code": "SKILL_NOT_FOUND",
    "message": "Skill 'foo' not found",
    "details": { ... }
  }
}
```

---

## Notes

- Registry fetches skill data from git tags—no file uploads needed
- Authentication uses GitHub OAuth for seamless git integration
- Consider webhook support for automatic indexing on tag push
- Rate limiting: 100 requests/minute for authenticated, 20 for anonymous
- Skill names are globally unique (first-come-first-served)
- Consider namespaced skills in future: `@org/skill-name`
