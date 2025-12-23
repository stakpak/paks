//! API error types

use thiserror::Error;

/// Errors that can occur when interacting with the Paks Registry API
#[derive(Error, Debug)]
pub enum ApiError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Failed to parse response
    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    /// Authentication required but no token provided
    #[error("Authentication required")]
    AuthRequired,

    /// Invalid or expired token
    #[error("Invalid or expired token")]
    InvalidToken,

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Rate limited
    #[error("Rate limited. Retry after {retry_after:?} seconds")]
    RateLimited { retry_after: Option<u64> },

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),
}
