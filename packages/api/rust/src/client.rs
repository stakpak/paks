//! Paks Registry API Client

use crate::error::ApiError;
use paks_api_schema::*;
use reqwest::{Client, Response, StatusCode, header};
use std::time::Duration;
use url::Url;

/// Default API base URL
pub const DEFAULT_BASE_URL: &str = "https://apiv2.stakpak.dev";

/// Default request timeout in seconds
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Paks Registry API client
#[derive(Debug, Clone)]
pub struct PaksClient {
    base_url: Url,
    http_client: Client,
    auth_token: Option<String>,
}

impl PaksClient {
    /// Create a new client with default settings
    pub fn new() -> Result<Self, ApiError> {
        Self::builder().build()
    }

    /// Create a new client builder
    pub fn builder() -> PaksClientBuilder {
        PaksClientBuilder::default()
    }

    /// Set the authentication token
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.auth_token = Some(token.into());
    }

    /// Clear the authentication token
    pub fn clear_token(&mut self) {
        self.auth_token = None;
    }

    /// Check if the client has an auth token set
    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }

    // ========================================================================
    // Paks Endpoints
    // ========================================================================

    /// List paks with optional sorting, filtering, and pagination
    pub async fn list_paks(
        &self,
        query: ListPaksQuery,
    ) -> Result<Vec<PakWithLatestVersion>, ApiError> {
        let url = self.build_url("/v1/paks")?;
        let response = self
            .http_client
            .get(url)
            .query(&query)
            .headers(self.build_headers(false))
            .send()
            .await?;

        let result: ListPaksResponse = self.handle_response(response).await?;
        Ok(result.items)
    }

    /// Search paks by identifier (owner/pak_name) or keywords
    pub async fn search_paks(&self, query: SearchPaksQuery) -> Result<Vec<Pak>, ApiError> {
        let url = self.build_url("/v1/paks/search")?;
        let response = self
            .http_client
            .get(url)
            .query(&query)
            .headers(self.build_headers(false))
            .send()
            .await?;

        let result: SearchPaksResponse = self.handle_response(response).await?;
        Ok(result.results)
    }

    /// Get pak content by URI
    ///
    /// URI format: `owner/pak_name[@version][/path]`
    pub async fn get_pak_content(&self, uri: &str) -> Result<PakContentResponse, ApiError> {
        let encoded_uri = urlencoding::encode(uri);
        let path = format!("/v1/paks/content/{}", encoded_uri);
        let url = self.build_url(&path)?;

        let response = self
            .http_client
            .get(url)
            .headers(self.build_headers(false))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get a pak by owner and name
    pub async fn get_pak(&self, owner: &str, pak_name: &str) -> Result<Option<Pak>, ApiError> {
        let query = SearchPaksQuery {
            owner: Some(owner.to_string()),
            pak_name: Some(pak_name.to_string()),
            limit: Some(1),
            ..Default::default()
        };

        let results = self.search_paks(query).await?;
        Ok(results.into_iter().next())
    }

    // ========================================================================
    // Install Endpoints
    // ========================================================================

    /// Get pak installation info by URI
    ///
    /// URI format: `owner/pak_name[@version]`
    ///
    /// This endpoint returns all metadata needed to install a pak from git,
    /// and automatically records a download event.
    pub async fn get_pak_install(&self, uri: &str) -> Result<PakInstallResponse, ApiError> {
        let encoded_uri = urlencoding::encode(uri);
        let path = format!("/v1/paks/install/{}", encoded_uri);
        let url = self.build_url(&path)?;

        let response = self
            .http_client
            .get(url)
            .headers(self.build_headers(false))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // ========================================================================
    // Auth Endpoints
    // ========================================================================

    /// Verify the current auth token
    pub async fn verify_token(&self) -> Result<VerifyTokenResponse, ApiError> {
        if !self.is_authenticated() {
            return Err(ApiError::AuthRequired);
        }

        let url = self.build_url("/v1/auth/verify")?;
        let response = self
            .http_client
            .get(url)
            .headers(self.build_headers(true))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get current user info
    pub async fn get_current_user(&self) -> Result<UserInfo, ApiError> {
        if !self.is_authenticated() {
            return Err(ApiError::AuthRequired);
        }

        let url = self.build_url("/v1/account")?;
        let response = self
            .http_client
            .get(url)
            .headers(self.build_headers(true))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // ========================================================================
    // Publish Endpoints
    // ========================================================================

    /// Publish a pak to the registry
    ///
    /// The server will:
    /// 1. Validate tag format (vMAJOR.MINOR.PATCH with optional prerelease)
    /// 2. Validate tag exists in the repository
    /// 3. Validate branch exists in the repository
    /// 4. Fetch and parse SKILL.md from the tag at the specified path
    /// 5. Validate SKILL.md frontmatter (name, description, metadata.version)
    /// 6. Validate pak name format (lowercase, alphanumeric, hyphens)
    /// 7. Validate version in SKILL.md matches tag
    /// 8. Create pak (if new) or add version (if exists)
    pub async fn publish_pak(
        &self,
        request: PublishPakRequest,
    ) -> Result<PublishPakResponse, ApiError> {
        if !self.is_authenticated() {
            return Err(ApiError::AuthRequired);
        }

        let url = self.build_url("/v1/paks/publish")?;
        let response = self
            .http_client
            .post(url)
            .headers(self.build_headers(true))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    // ========================================================================
    // Internal Helpers
    // ========================================================================

    fn build_url(&self, path: &str) -> Result<Url, ApiError> {
        self.base_url.join(path).map_err(ApiError::InvalidUrl)
    }

    fn build_headers(&self, require_auth: bool) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();

        // User-Agent
        let version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("paks-api/{}", version);
        if let Ok(value) = header::HeaderValue::from_str(&user_agent) {
            headers.insert(header::USER_AGENT, value);
        }

        // Content-Type
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        // Accept
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        // Authorization
        if let Some(token) = &self.auth_token {
            let auth_value = format!("Bearer {}", token);
            if let Ok(value) = header::HeaderValue::from_str(&auth_value) {
                headers.insert(header::AUTHORIZATION, value);
            }
        } else if require_auth {
            // This case is handled by the caller checking is_authenticated()
        }

        headers
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, ApiError> {
        let status = response.status();

        match status {
            StatusCode::OK | StatusCode::CREATED => {
                let body = response.text().await?;
                // Handle empty response body (e.g., 200 OK with no content)
                if body.is_empty() || body.trim().is_empty() {
                    // Try to deserialize from empty JSON object for types that support Default
                    serde_json::from_str("{}").map_err(ApiError::Parse)
                } else {
                    serde_json::from_str(&body).map_err(ApiError::Parse)
                }
            }
            StatusCode::UNAUTHORIZED => Err(ApiError::InvalidToken),
            StatusCode::NOT_FOUND => {
                let url = response.url().to_string();
                Err(ApiError::NotFound(url))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse().ok());
                Err(ApiError::RateLimited { retry_after })
            }
            _ => {
                let body = response.text().await.unwrap_or_default();
                let message =
                    if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&body) {
                        error_response.error.message
                    } else {
                        body
                    };
                Err(ApiError::Api {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }
}

impl Default for PaksClient {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback with minimal client - should never fail in practice
            Self {
                base_url: Url::parse(DEFAULT_BASE_URL).unwrap_or_else(|_| unreachable!()),
                http_client: Client::new(),
                auth_token: None,
            }
        })
    }
}

/// Builder for PaksClient
#[derive(Debug, Default)]
pub struct PaksClientBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    auth_token: Option<String>,
}

impl PaksClientBuilder {
    /// Set the base URL for the API
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the authentication token
    pub fn auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    /// Build the client
    pub fn build(self) -> Result<PaksClient, ApiError> {
        let base_url_str = self.base_url.as_deref().unwrap_or(DEFAULT_BASE_URL);
        let base_url = Url::parse(base_url_str)?;

        let timeout = self
            .timeout
            .unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECS));

        let http_client = Client::builder().timeout(timeout).build()?;

        Ok(PaksClient {
            base_url,
            http_client,
            auth_token: self.auth_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder_default() {
        let client = PaksClient::builder().build();
        assert!(client.is_ok());
        let client = client.unwrap();
        assert!(!client.is_authenticated());
    }

    #[test]
    fn test_client_builder_with_token() {
        let client = PaksClient::builder()
            .auth_token("test_token")
            .build()
            .unwrap();
        assert!(client.is_authenticated());
    }

    #[test]
    fn test_client_builder_custom_url() {
        let client = PaksClient::builder()
            .base_url("https://custom.api.dev")
            .build()
            .unwrap();
        assert_eq!(client.base_url.as_str(), "https://custom.api.dev/");
    }
}
