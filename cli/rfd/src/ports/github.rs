//! GitHub integration port (hexagonal architecture boundary).
//!
//! This module defines the interface between RFD domain logic and GitHub.
//! Multiple adapters can implement this trait (API, mock, etc.).
//!
//! # Architecture
//!
//! ```text
//! RFD Domain → GitHubPort (trait) → Adapter (API/Mock)
//! ```
//!
//! # Design Principles
//!
//! - **Port = Interface**: Defines what operations we need from GitHub
//! - **Adapter = Implementation**: How we actually talk to GitHub
//! - **Domain Independence**: RFD logic doesn't depend on GitHub details
//! - **Testability**: Mock adapter for tests, API adapter for production
//!
//! # For Junior Developers
//!
//! This is hexagonal architecture (ports and adapters pattern):
//! - The "port" is the trait `GitHubPort`
//! - The "adapters" are the implementations (API, mock)
//! - This allows us to swap implementations without changing domain code
//! - Tests use mock adapter (no real GitHub API calls)
//! - Production uses API adapter (real GitHub REST API)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Port for GitHub operations.
///
/// This trait defines all GitHub operations that the RFD tool needs.
/// Different adapters can implement this trait to provide different
/// implementations (e.g., REST API, mock for testing).
#[async_trait]
pub trait GitHubPort: Send + Sync {
    /// Create a GitHub issue.
    ///
    /// # Arguments
    ///
    /// * `request` - Issue creation details (title, body, labels)
    ///
    /// # Returns
    ///
    /// The created issue with its number and URL.
    ///
    /// # Errors
    ///
    /// - `GitHubError::Auth` - Authentication failed
    /// - `GitHubError::RateLimit` - API rate limit exceeded
    /// - `GitHubError::Api` - Other API errors
    async fn create_issue(
        &self,
        request: CreateIssueRequest,
    ) -> Result<GitHubIssue, GitHubError>;

    /// Get issue details by number.
    ///
    /// # Arguments
    ///
    /// * `number` - Issue number (e.g., 42)
    ///
    /// # Returns
    ///
    /// The issue details including current state and labels.
    ///
    /// # Errors
    ///
    /// - `GitHubError::NotFound` - Issue doesn't exist
    /// - `GitHubError::Auth` - Authentication failed
    async fn get_issue(&self, number: u32) -> Result<GitHubIssue, GitHubError>;

    /// Add a comment to an issue.
    ///
    /// # Arguments
    ///
    /// * `number` - Issue number
    /// * `body` - Comment body (markdown)
    ///
    /// # Returns
    ///
    /// The created comment with its ID.
    ///
    /// # Errors
    ///
    /// - `GitHubError::NotFound` - Issue doesn't exist
    /// - `GitHubError::PermissionDenied` - Can't comment on issue
    async fn add_comment(
        &self,
        number: u32,
        body: String,
    ) -> Result<GitHubComment, GitHubError>;

    /// Set issue labels (replaces all existing labels).
    ///
    /// # Arguments
    ///
    /// * `number` - Issue number
    /// * `labels` - New labels to set
    ///
    /// # Errors
    ///
    /// - `GitHubError::NotFound` - Issue doesn't exist
    /// - `GitHubError::PermissionDenied` - Can't modify labels
    async fn set_labels(&self, number: u32, labels: Vec<String>) -> Result<(), GitHubError>;

    /// Update issue state (open/closed).
    ///
    /// # Arguments
    ///
    /// * `number` - Issue number
    /// * `state` - New state (open or closed)
    ///
    /// # Errors
    ///
    /// - `GitHubError::NotFound` - Issue doesn't exist
    /// - `GitHubError::PermissionDenied` - Can't modify issue
    async fn update_state(
        &self,
        number: u32,
        state: IssueState,
    ) -> Result<(), GitHubError>;

    /// Check GitHub API rate limit status.
    ///
    /// # Returns
    ///
    /// Current rate limit information (remaining requests, reset time).
    async fn rate_limit(&self) -> Result<RateLimit, GitHubError>;

    /// Validate authentication and get user info.
    ///
    /// # Returns
    ///
    /// Authentication status and user details.
    ///
    /// # Errors
    ///
    /// - `GitHubError::Auth` - Token is invalid or missing
    async fn auth_status(&self) -> Result<AuthStatus, GitHubError>;
}

// ============================================================================
// Domain Models
// ============================================================================

/// Request to create a GitHub issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueRequest {
    /// Issue title
    pub title: String,

    /// Issue body (markdown)
    pub body: String,

    /// Labels to add to the issue
    #[serde(default)]
    pub labels: Vec<String>,

    /// Users to assign (optional)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
}

/// GitHub issue details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    /// Issue number (e.g., 42)
    pub number: u32,

    /// API URL (e.g., https://api.github.com/repos/org/repo/issues/42)
    pub url: String,

    /// Web URL (e.g., https://github.com/org/repo/issues/42)
    pub html_url: String,

    /// Current state (open or closed)
    pub state: IssueState,

    /// Issue title
    pub title: String,

    /// Issue body (markdown, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Current labels
    #[serde(default)]
    pub labels: Vec<String>,

    /// Creation timestamp (ISO 8601)
    pub created_at: String,

    /// Last update timestamp (ISO 8601)
    pub updated_at: String,
}

/// Issue state (open or closed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueState {
    /// Issue is open
    Open,
    /// Issue is closed
    Closed,
}

impl std::fmt::Display for IssueState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueState::Open => write!(f, "open"),
            IssueState::Closed => write!(f, "closed"),
        }
    }
}

/// GitHub comment details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubComment {
    /// Comment ID
    pub id: u64,

    /// Web URL for the comment
    pub url: String,

    /// Comment body (markdown)
    pub body: String,

    /// Creation timestamp (ISO 8601)
    pub created_at: String,
}

/// GitHub API rate limit information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Total requests allowed per hour
    pub limit: u32,

    /// Remaining requests in current window
    pub remaining: u32,

    /// When the rate limit resets (ISO 8601)
    pub reset_at: String,
}

/// Authentication status and user information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatus {
    /// Whether authentication is valid
    pub authenticated: bool,

    /// GitHub username (if authenticated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Token scopes (e.g., ["repo", "user"])
    #[serde(default)]
    pub scopes: Vec<String>,
}

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during GitHub operations.
///
/// All errors include actionable suggestions for how to fix them.
/// This makes the tool more agent-friendly and helpful for users.
#[derive(Debug, Error)]
pub enum GitHubError {
    /// Authentication failed (missing or invalid token).
    #[error("Authentication failed: {message}\nSuggestion: {suggestion}")]
    Auth {
        /// What went wrong
        message: String,
        /// How to fix it
        suggestion: String,
    },

    /// GitHub API rate limit exceeded.
    #[error("Rate limit exceeded. Resets at {reset_at}\nSuggestion: Wait or use a different token")]
    RateLimit {
        /// When the rate limit resets (ISO 8601)
        reset_at: String,
    },

    /// Resource not found (issue, repository, etc.).
    #[error("Not found: {resource}")]
    NotFound {
        /// What resource wasn't found
        resource: String,
    },

    /// Permission denied (insufficient token scopes).
    #[error("Permission denied: {message}\nSuggestion: Check your token has 'repo' scope")]
    PermissionDenied {
        /// What operation was denied
        message: String,
    },

    /// GitHub API returned an error.
    #[error("GitHub API error: {message}")]
    Api {
        /// Error message from GitHub
        message: String,
        /// HTTP status code (if available)
        status_code: Option<u16>,
    },

    /// Network error (connection failed, timeout, etc.).
    #[error("Network error: {message}\nSuggestion: Check your internet connection")]
    Network {
        /// What went wrong
        message: String,
    },

    /// Failed to parse GitHub API response.
    #[error("Parse error: {message}\nSuggestion: This is likely a bug, please report it")]
    Parse {
        /// What went wrong
        message: String,
    },
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extract issue number from GitHub issue URL.
///
/// # Examples
///
/// ```
/// # use rfd::ports::github::extract_issue_number;
/// assert_eq!(
///     extract_issue_number("https://github.com/org/repo/issues/42"),
///     Some(42)
/// );
/// ```
pub fn extract_issue_number(url: &str) -> Option<u32> {
    url.split('/')
        .last()
        .and_then(|s| s.parse().ok())
}

/// Check if a URL is a valid GitHub issue URL.
///
/// # Examples
///
/// ```
/// # use rfd::ports::github::is_github_issue_url;
/// assert!(is_github_issue_url("https://github.com/org/repo/issues/42"));
/// assert!(!is_github_issue_url("https://example.com"));
/// ```
pub fn is_github_issue_url(url: &str) -> bool {
    url.starts_with("https://github.com/") && url.contains("/issues/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_issue_number() {
        assert_eq!(
            extract_issue_number("https://github.com/org/repo/issues/42"),
            Some(42)
        );
        assert_eq!(
            extract_issue_number("https://github.com/org/repo/issues/123"),
            Some(123)
        );
        assert_eq!(
            extract_issue_number("https://github.com/org/repo/pull/42"),
            Some(42)
        );
        assert_eq!(
            extract_issue_number("not a url"),
            None
        );
    }

    #[test]
    fn test_is_github_issue_url() {
        assert!(is_github_issue_url("https://github.com/org/repo/issues/42"));
        assert!(is_github_issue_url("https://github.com/org/repo/issues/123"));
        assert!(!is_github_issue_url("https://example.com"));
        assert!(!is_github_issue_url("https://github.com/org/repo"));
    }

    #[test]
    fn test_issue_state_display() {
        assert_eq!(IssueState::Open.to_string(), "open");
        assert_eq!(IssueState::Closed.to_string(), "closed");
    }
}
