//! GitHub REST API adapter implementation.
//!
//! This adapter implements the GitHubPort trait using GitHub's REST API.
//! It handles authentication, rate limiting, and error responses.
//!
//! # Authentication
//!
//! Requires `GITHUB_TOKEN` environment variable with appropriate scopes:
//! - `repo` for private repositories
//! - `public_repo` for public repositories only
//!
//! Create a token at: https://github.com/settings/tokens
//!
//! # Rate Limiting
//!
//! GitHub API has rate limits:
//! - 5,000 requests/hour for authenticated requests
//! - 60 requests/hour for unauthenticated
//!
//! This adapter checks rate limit headers and returns actionable errors.

use crate::ports::github::*;
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde_json::json;

/// GitHub REST API adapter.
///
/// Implements GitHubPort using GitHub's REST API v3.
/// Requires GITHUB_TOKEN environment variable.
pub struct GitHubApiAdapter {
    client: Client,
    token: String,
    repo: String,
    base_url: String,
}

impl GitHubApiAdapter {
    /// Create a new GitHub API adapter.
    ///
    /// # Arguments
    ///
    /// * `repo` - Repository in "owner/repo" format (e.g., "adanoelle/tapestry")
    ///
    /// # Errors
    ///
    /// Returns `GitHubError::Auth` if GITHUB_TOKEN is not set or client creation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rfd::adapters::github_api::GitHubApiAdapter;
    /// // Requires GITHUB_TOKEN environment variable
    /// let adapter = GitHubApiAdapter::new("adanoelle/tapestry".to_string())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(repo: String) -> Result<Self, GitHubError> {
        let token = std::env::var("GITHUB_TOKEN").map_err(|_| GitHubError::Auth {
            message: "GITHUB_TOKEN environment variable not set".to_string(),
            suggestion: concat!(
                "Create a personal access token at https://github.com/settings/tokens ",
                "with 'repo' scope, then: export GITHUB_TOKEN=ghp_xxx"
            )
            .to_string(),
        })?;

        let client = Client::builder()
            .user_agent("tapestry-rfd-cli/0.1.0")
            .build()
            .map_err(|e| GitHubError::Network {
                message: format!("Failed to create HTTP client: {}", e),
            })?;

        Ok(Self {
            client,
            token,
            repo,
            base_url: "https://api.github.com".to_string(),
        })
    }

    /// Make an authenticated GitHub API request.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method (GET, POST, PATCH, etc.)
    /// * `path` - API endpoint path (e.g., "/repos/owner/repo/issues")
    /// * `body` - Optional JSON body for the request
    ///
    /// # Returns
    ///
    /// The parsed JSON response.
    ///
    /// # Errors
    ///
    /// - `GitHubError::Auth` - Authentication failed
    /// - `GitHubError::RateLimit` - Rate limit exceeded
    /// - `GitHubError::NotFound` - Resource not found
    /// - `GitHubError::Network` - Connection failed
    /// - `GitHubError::Parse` - Response parsing failed
    async fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, GitHubError> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await.map_err(|e| GitHubError::Network {
            message: format!("Request failed: {}", e),
        })?;

        let status = response.status();

        // Check rate limit headers
        if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
            if let Ok(remaining_str) = remaining.to_str() {
                if let Ok(remaining_num) = remaining_str.parse::<u32>() {
                    // Warn if running low on requests
                    if remaining_num < 100 {
                        eprintln!(
                            "Warning: Only {} GitHub API requests remaining",
                            remaining_num
                        );
                    }

                    // Error if rate limit exceeded
                    if remaining_num == 0 {
                        if let Some(reset) = response.headers().get("x-ratelimit-reset") {
                            if let Ok(reset_str) = reset.to_str() {
                                if let Ok(reset_timestamp) = reset_str.parse::<i64>() {
                                    let reset_time = chrono::DateTime::from_timestamp(reset_timestamp, 0)
                                        .unwrap_or_else(|| chrono::Utc::now())
                                        .to_rfc3339();
                                    return Err(GitHubError::RateLimit { reset_at: reset_time });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Handle HTTP status codes
        match status {
            StatusCode::OK | StatusCode::CREATED => {
                response.json().await.map_err(|e| GitHubError::Parse {
                    message: format!("Failed to parse response: {}", e),
                })
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                let error_body = response.text().await.unwrap_or_default();
                Err(GitHubError::Auth {
                    message: format!("Authentication failed: {}", error_body),
                    suggestion: concat!(
                        "Check your GITHUB_TOKEN is valid and has the required scopes ",
                        "(repo or public_repo)"
                    )
                    .to_string(),
                })
            }
            StatusCode::NOT_FOUND => Err(GitHubError::NotFound {
                resource: path.to_string(),
            }),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_body = response.text().await.unwrap_or_default();
                Err(GitHubError::Api {
                    message: format!("Validation error: {}", error_body),
                    status_code: Some(status.as_u16()),
                })
            }
            _ => {
                let error_body = response.text().await.unwrap_or_default();
                Err(GitHubError::Api {
                    message: error_body,
                    status_code: Some(status.as_u16()),
                })
            }
        }
    }
}

#[async_trait]
impl GitHubPort for GitHubApiAdapter {
    async fn create_issue(
        &self,
        request: CreateIssueRequest,
    ) -> Result<GitHubIssue, GitHubError> {
        let body = json!({
            "title": request.title,
            "body": request.body,
            "labels": request.labels,
            "assignees": request.assignees,
        });

        let result = self
            .request(
                reqwest::Method::POST,
                &format!("/repos/{}/issues", self.repo),
                Some(body),
            )
            .await?;

        Ok(GitHubIssue {
            number: result["number"]
                .as_u64()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'number' in response".to_string(),
                })? as u32,
            url: result["url"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'url' in response".to_string(),
                })?
                .to_string(),
            html_url: result["html_url"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'html_url' in response".to_string(),
                })?
                .to_string(),
            state: match result["state"].as_str() {
                Some("open") => IssueState::Open,
                Some("closed") => IssueState::Closed,
                _ => IssueState::Open,
            },
            title: result["title"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'title' in response".to_string(),
                })?
                .to_string(),
            body: result["body"].as_str().map(String::from),
            labels: result["labels"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v["name"].as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            created_at: result["created_at"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'created_at' in response".to_string(),
                })?
                .to_string(),
            updated_at: result["updated_at"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'updated_at' in response".to_string(),
                })?
                .to_string(),
        })
    }

    async fn get_issue(&self, number: u32) -> Result<GitHubIssue, GitHubError> {
        let result = self
            .request(
                reqwest::Method::GET,
                &format!("/repos/{}/issues/{}", self.repo, number),
                None,
            )
            .await?;

        Ok(GitHubIssue {
            number: result["number"]
                .as_u64()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'number' in response".to_string(),
                })? as u32,
            url: result["url"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'url' in response".to_string(),
                })?
                .to_string(),
            html_url: result["html_url"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'html_url' in response".to_string(),
                })?
                .to_string(),
            state: match result["state"].as_str() {
                Some("open") => IssueState::Open,
                Some("closed") => IssueState::Closed,
                _ => IssueState::Open,
            },
            title: result["title"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'title' in response".to_string(),
                })?
                .to_string(),
            body: result["body"].as_str().map(String::from),
            labels: result["labels"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v["name"].as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            created_at: result["created_at"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'created_at' in response".to_string(),
                })?
                .to_string(),
            updated_at: result["updated_at"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'updated_at' in response".to_string(),
                })?
                .to_string(),
        })
    }

    async fn add_comment(
        &self,
        number: u32,
        body: String,
    ) -> Result<GitHubComment, GitHubError> {
        let request_body = json!({ "body": body });

        let result = self
            .request(
                reqwest::Method::POST,
                &format!("/repos/{}/issues/{}/comments", self.repo, number),
                Some(request_body),
            )
            .await?;

        Ok(GitHubComment {
            id: result["id"]
                .as_u64()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'id' in response".to_string(),
                })?,
            url: result["html_url"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'html_url' in response".to_string(),
                })?
                .to_string(),
            body: result["body"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'body' in response".to_string(),
                })?
                .to_string(),
            created_at: result["created_at"]
                .as_str()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'created_at' in response".to_string(),
                })?
                .to_string(),
        })
    }

    async fn set_labels(&self, number: u32, labels: Vec<String>) -> Result<(), GitHubError> {
        let body = json!({ "labels": labels });

        self.request(
            reqwest::Method::PUT,
            &format!("/repos/{}/issues/{}/labels", self.repo, number),
            Some(body),
        )
        .await?;

        Ok(())
    }

    async fn update_state(&self, number: u32, state: IssueState) -> Result<(), GitHubError> {
        let state_str = match state {
            IssueState::Open => "open",
            IssueState::Closed => "closed",
        };

        let body = json!({ "state": state_str });

        self.request(
            reqwest::Method::PATCH,
            &format!("/repos/{}/issues/{}", self.repo, number),
            Some(body),
        )
        .await?;

        Ok(())
    }

    async fn rate_limit(&self) -> Result<RateLimit, GitHubError> {
        let result = self
            .request(reqwest::Method::GET, "/rate_limit", None)
            .await?;

        let core = &result["resources"]["core"];
        Ok(RateLimit {
            limit: core["limit"]
                .as_u64()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'limit' in rate_limit response".to_string(),
                })? as u32,
            remaining: core["remaining"]
                .as_u64()
                .ok_or_else(|| GitHubError::Parse {
                    message: "Missing 'remaining' in rate_limit response".to_string(),
                })? as u32,
            reset_at: chrono::DateTime::from_timestamp(
                core["reset"]
                    .as_i64()
                    .ok_or_else(|| GitHubError::Parse {
                        message: "Missing 'reset' in rate_limit response".to_string(),
                    })?,
                0,
            )
            .ok_or_else(|| GitHubError::Parse {
                message: "Invalid timestamp in rate_limit response".to_string(),
            })?
            .to_rfc3339(),
        })
    }

    async fn auth_status(&self) -> Result<AuthStatus, GitHubError> {
        let result = self.request(reqwest::Method::GET, "/user", None).await?;

        Ok(AuthStatus {
            authenticated: true,
            user: result["login"].as_str().map(String::from),
            scopes: vec!["repo".to_string()], // Simplified - would need to parse X-OAuth-Scopes header
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_requires_token() {
        // Remove GITHUB_TOKEN if it exists for this test
        std::env::remove_var("GITHUB_TOKEN");

        let result = GitHubApiAdapter::new("test/repo".to_string());
        assert!(result.is_err());

        if let Err(GitHubError::Auth { message, .. }) = result {
            assert!(message.contains("GITHUB_TOKEN"));
        } else {
            panic!("Expected Auth error");
        }
    }

    // Integration tests would go here, gated by feature flag
    // #[tokio::test]
    // #[cfg(feature = "integration-tests")]
    // async fn test_create_issue_real_api() { ... }
}
