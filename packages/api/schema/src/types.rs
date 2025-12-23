//! API request and response types
//!
//! All types derive `JsonSchema` for schema generation.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// ============================================================================
// Enums
// ============================================================================

/// Pak visibility level
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum PakVisibility {
    #[default]
    Public,
    Unlisted,
    Private,
}

impl fmt::Display for PakVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PakVisibility::Public => write!(f, "PUBLIC"),
            PakVisibility::Unlisted => write!(f, "UNLISTED"),
            PakVisibility::Private => write!(f, "PRIVATE"),
        }
    }
}

/// Pak status
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum PakStatus {
    #[default]
    Active,
    Deprecated,
}

impl fmt::Display for PakStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PakStatus::Active => write!(f, "ACTIVE"),
            PakStatus::Deprecated => write!(f, "DEPRECATED"),
        }
    }
}

/// Pak version review status
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum PakVersionStatus {
    #[default]
    Submitted,
    Approved,
    Rejected,
}

impl fmt::Display for PakVersionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PakVersionStatus::Submitted => write!(f, "SUBMITTED"),
            PakVersionStatus::Approved => write!(f, "APPROVED"),
            PakVersionStatus::Rejected => write!(f, "REJECTED"),
        }
    }
}

/// Sort order for listing paks
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PakSortBy {
    #[default]
    Trending,
    MostPopular,
    Recent,
}

impl fmt::Display for PakSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PakSortBy::Trending => write!(f, "TRENDING"),
            PakSortBy::MostPopular => write!(f, "MOST_POPULAR"),
            PakSortBy::Recent => write!(f, "RECENT"),
        }
    }
}

/// Time window for download/usage counts
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PakTimeWindow {
    Daily,
    Weekly,
    Monthly,
    #[default]
    AllTime,
}

impl fmt::Display for PakTimeWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PakTimeWindow::Daily => write!(f, "DAILY"),
            PakTimeWindow::Weekly => write!(f, "WEEKLY"),
            PakTimeWindow::Monthly => write!(f, "MONTHLY"),
            PakTimeWindow::AllTime => write!(f, "ALL_TIME"),
        }
    }
}

/// Type of content item
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ContentItemType {
    File,
    Dir,
}

// ============================================================================
// Core Models
// ============================================================================

/// A pak (knowledge package) in the registry
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Pak {
    /// Unique identifier
    pub id: Uuid,
    /// Pak name
    pub name: String,
    /// Owner/account name
    pub owner_name: String,
    /// Short URI (owner/name)
    pub uri: String,
    /// Full URI with protocol (stakpak://owner/name)
    pub full_uri: String,
    /// Path within repository (for monorepos)
    pub path: Option<String>,
    /// Git repository URL
    pub repository_url: String,
    /// Pak description
    pub description: Option<String>,
    /// Tags/keywords
    pub tags: Vec<String>,
    /// Visibility level
    pub visibility: PakVisibility,
    /// Status
    pub status: PakStatus,
    /// Downloads in current time window
    pub download_count: i64,
    /// Usages in current time window
    pub usage_count: i64,
    /// Total downloads all time
    pub total_downloads: i64,
    /// Total usages all time
    pub total_usages: i64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// A specific version of a pak
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PakVersion {
    /// Unique identifier
    pub id: Uuid,
    /// Semantic version (e.g., 1.0.0)
    pub version: String,
    /// Git tag (e.g., v1.0.0)
    pub git_tag: String,
    /// Content checksum
    pub checksum: String,
    /// Size in bytes
    pub size_bytes: Option<i64>,
    /// pak.toml manifest content
    pub manifest: String,
    /// Review status
    pub status: PakVersionStatus,
    /// Download count for this version
    pub downloads: i64,
    /// Usage count for this version
    pub usages: i64,
    /// Publication timestamp
    pub published_at: DateTime<Utc>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Pak with its latest version info
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PakWithLatestVersion {
    /// The pak
    #[serde(flatten)]
    pub pak: Pak,
    /// Latest version info (null if no versions)
    pub latest_version: Option<PakVersion>,
}

/// Pak version with full pak info and optional path
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PakVersionWithPakAndPath {
    /// Unique identifier
    pub id: Uuid,
    /// Semantic version
    pub version: String,
    /// Git tag
    pub git_tag: String,
    /// Content checksum
    pub checksum: String,
    /// Size in bytes
    pub size_bytes: Option<i64>,
    /// pak.toml manifest content
    pub manifest: String,
    /// Review status
    pub status: PakVersionStatus,
    /// Download count
    pub downloads: i64,
    /// Usage count
    pub usages: i64,
    /// Publication timestamp
    pub published_at: DateTime<Utc>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// The pak
    pub pak: Pak,
    /// Requested path within pak
    pub path: Option<String>,
}

// ============================================================================
// Content Models
// ============================================================================

/// Content item in a directory listing
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentItem {
    /// File or directory name
    pub name: String,
    /// Full URI to this item
    pub uri: String,
    /// Type of item
    #[serde(rename = "type")]
    pub item_type: ContentItemType,
    /// Size in bytes (for files)
    pub size: Option<i64>,
    /// File content (if pre-fetched)
    pub content: Option<String>,
}

/// Content of a pak - either a file or directory
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "type")]
pub enum PakContent {
    /// File content
    File {
        /// The file content
        content: String,
    },
    /// Directory listing
    Directory {
        /// Directory contents
        items: Vec<ContentItem>,
    },
}

/// Response from the content endpoint
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PakContentResponse {
    /// Requested URI
    pub uri: String,
    /// Content (file or directory)
    pub content: PakContent,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Query parameters for listing paks
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ListPaksQuery {
    /// Sort order: TRENDING, MOST_POPULAR, or RECENT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<PakSortBy>,
    /// Time window for download counts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<PakTimeWindow>,
    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Response from listing paks
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ListPaksResponse {
    /// List of paks with their latest versions
    pub results: Vec<PakWithLatestVersion>,
}

/// Query parameters for searching paks
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct SearchPaksQuery {
    /// Owner name (for identifier search)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Pak name (for identifier search)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pak_name: Option<String>,
    /// Freeform keyword query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Response from searching paks
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchPaksResponse {
    /// List of matching paks
    pub results: Vec<Pak>,
}

// ============================================================================
// Auth Models
// ============================================================================

/// User information returned from auth endpoints
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UserInfo {
    /// User ID
    pub id: String,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// Avatar URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

/// Response from token verification
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct VerifyTokenResponse {
    /// Whether the token is valid
    pub valid: bool,
    /// User info
    pub user: UserInfo,
    /// Token expiration time (null if no expiry)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

// ============================================================================
// Publish Models
// ============================================================================

/// Request body for POST /v1/paks/publish
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PublishPakRequest {
    /// Git clone URL (HTTPS) - must be a GitHub URL
    pub repository: String,
    /// Path to pak within repo ("." for root, "paks/my-pak" for monorepo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Branch the tag was created from
    pub branch: String,
    /// Git tag name (must start with `v` and follow semver)
    pub tag: String,
}

/// Response from publish endpoint (empty on success - 200 OK)
#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct PublishPakResponse {}

// ============================================================================
// Install Models
// ============================================================================

/// Pak info for installation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InstallPakInfo {
    /// Unique pak identifier
    pub id: Uuid,
    /// Owner/account name
    pub owner: String,
    /// Pak name
    pub name: String,
    /// Pak description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Visibility level
    pub visibility: PakVisibility,
}

/// Version info for installation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InstallVersionInfo {
    /// Semantic version (e.g., 1.2.3)
    pub version: String,
    /// Git tag (e.g., v1.2.3)
    pub tag: String,
    /// Full commit SHA
    pub commit_hash: String,
    /// Publication timestamp
    pub published_at: DateTime<Utc>,
}

/// Repository info for installation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InstallRepositoryInfo {
    /// Repository URL
    pub url: String,
    /// HTTPS clone URL
    pub clone_url: String,
    /// SSH clone URL
    pub ssh_url: String,
    /// Default branch name
    pub default_branch: String,
}

/// Install path info
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InstallPathInfo {
    /// Path within repo (for monorepos), "." for root
    pub path: String,
    /// List of files/directories in the pak
    pub files: Vec<String>,
}

/// Response from the install endpoint
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PakInstallResponse {
    /// Pak information
    pub pak: InstallPakInfo,
    /// Version information
    pub version: InstallVersionInfo,
    /// Repository information
    pub repository: InstallRepositoryInfo,
    /// Installation path info
    pub install: InstallPathInfo,
}

// ============================================================================
// Error Models
// ============================================================================

/// Error detail
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ErrorDetail {
    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Error message
    pub message: String,
}

/// API error response
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ErrorResponse {
    /// Error details
    pub error: ErrorDetail,
}

// ============================================================================
// Root Schema (for generation)
// ============================================================================

/// Root schema containing all API types
/// Used for JSON Schema generation
#[derive(JsonSchema)]
pub struct PaksApiSchema {
    // Enums
    pub pak_visibility: PakVisibility,
    pub pak_status: PakStatus,
    pub pak_version_status: PakVersionStatus,
    pub pak_sort_by: PakSortBy,
    pub pak_time_window: PakTimeWindow,
    pub content_item_type: ContentItemType,

    // Core models
    pub pak: Pak,
    pub pak_version: PakVersion,
    pub pak_with_latest_version: PakWithLatestVersion,
    pub pak_version_with_pak_and_path: PakVersionWithPakAndPath,

    // Content models
    pub content_item: ContentItem,
    pub pak_content: PakContent,
    pub pak_content_response: PakContentResponse,

    // Request/Response types
    pub list_paks_query: ListPaksQuery,
    pub list_paks_response: ListPaksResponse,
    pub search_paks_query: SearchPaksQuery,
    pub search_paks_response: SearchPaksResponse,

    // Auth models
    pub user_info: UserInfo,
    pub verify_token_response: VerifyTokenResponse,

    // Publish models
    pub publish_pak_request: PublishPakRequest,
    pub publish_pak_response: PublishPakResponse,

    // Install models
    pub install_pak_info: InstallPakInfo,
    pub install_version_info: InstallVersionInfo,
    pub install_repository_info: InstallRepositoryInfo,
    pub install_path_info: InstallPathInfo,
    pub pak_install_response: PakInstallResponse,

    // Error models
    pub error_detail: ErrorDetail,
    pub error_response: ErrorResponse,
}
