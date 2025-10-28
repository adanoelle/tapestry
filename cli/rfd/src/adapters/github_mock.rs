//! Mock GitHub adapter for testing.
//!
//! This adapter implements GitHubPort without making real GitHub API calls.
//! Perfect for unit tests where you want predictable behavior and fast execution.
//!
//! # Usage in Tests
//!
//! ```
//! # use rfd::adapters::github_mock::MockGitHubAdapter;
//! # use rfd::ports::github::*;
//! # tokio_test::block_on(async {
//! let mut mock = MockGitHubAdapter::new();
//!
//! // Create an issue (no real API call)
//! let issue = mock.create_issue(CreateIssueRequest {
//!     title: "Test Issue".to_string(),
//!     body: "Test body".to_string(),
//!     labels: vec![],
//!     assignees: vec![],
//! }).await?;
//!
//! assert_eq!(issue.number, 1);
//! assert_eq!(issue.title, "Test Issue");
//! # Ok::<(), GitHubError>(())
//! # });
//! ```

// GitHub integration (RFD-003) not yet wired up
#![allow(dead_code)]

use crate::ports::github::*;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock GitHub adapter for testing.
///
/// Simulates GitHub operations without making real API calls.
/// Issues are stored in memory and issue numbers auto-increment.
#[allow(dead_code)] // GitHub integration (RFD-003) not yet wired up
#[derive(Clone)]
pub struct MockGitHubAdapter {
    /// In-memory storage of issues (issue_number -> issue)
    issues: Arc<Mutex<HashMap<u32, GitHubIssue>>>,

    /// Next issue number to assign
    next_number: Arc<Mutex<u32>>,

    /// Repository name (for URL generation)
    repo: String,

    /// Simulated rate limit
    rate_limit: Arc<Mutex<RateLimit>>,

    /// Simulated authentication status
    auth_status: Arc<Mutex<AuthStatus>>,

    /// Whether to simulate errors
    simulate_errors: Arc<Mutex<Option<GitHubError>>>,
}

impl MockGitHubAdapter {
    /// Create a new mock adapter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rfd::adapters::github_mock::MockGitHubAdapter;
    /// let mock = MockGitHubAdapter::new();
    /// ```
    pub fn new() -> Self {
        Self::with_repo("test/repo".to_string())
    }

    /// Create a new mock adapter with a specific repository.
    ///
    /// # Arguments
    ///
    /// * `repo` - Repository in "owner/repo" format
    pub fn with_repo(repo: String) -> Self {
        Self {
            issues: Arc::new(Mutex::new(HashMap::new())),
            next_number: Arc::new(Mutex::new(1)),
            repo,
            rate_limit: Arc::new(Mutex::new(RateLimit {
                limit: 5000,
                remaining: 5000,
                reset_at: chrono::Utc::now().to_rfc3339(),
            })),
            auth_status: Arc::new(Mutex::new(AuthStatus {
                authenticated: true,
                user: Some("test-user".to_string()),
                scopes: vec!["repo".to_string()],
            })),
            simulate_errors: Arc::new(Mutex::new(None)),
        }
    }

    /// Configure mock to simulate rate limit exceeded.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rfd::adapters::github_mock::MockGitHubAdapter;
    /// let mock = MockGitHubAdapter::new().with_rate_limit_exceeded();
    /// // All operations will return RateLimit error
    /// ```
    pub fn with_rate_limit_exceeded(self) -> Self {
        *self.simulate_errors.lock().unwrap() = Some(GitHubError::RateLimit {
            reset_at: chrono::Utc::now().to_rfc3339(),
        });
        self
    }

    /// Configure mock to simulate authentication failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rfd::adapters::github_mock::MockGitHubAdapter;
    /// let mock = MockGitHubAdapter::new().with_auth_failure();
    /// // All operations will return Auth error
    /// ```
    pub fn with_auth_failure(self) -> Self {
        *self.simulate_errors.lock().unwrap() = Some(GitHubError::Auth {
            message: "Mock auth failure".to_string(),
            suggestion: "This is a simulated error for testing".to_string(),
        });
        self
    }

    /// Configure mock to simulate not found error.
    pub fn with_not_found(self) -> Self {
        *self.simulate_errors.lock().unwrap() = Some(GitHubError::NotFound {
            resource: "Mock resource".to_string(),
        });
        self
    }

    /// Get all issues created in this mock.
    ///
    /// Useful for verifying test behavior.
    pub fn get_all_issues(&self) -> Vec<GitHubIssue> {
        let issues = self.issues.lock().unwrap();
        let mut result: Vec<_> = issues.values().cloned().collect();
        result.sort_by_key(|i| i.number);
        result
    }

    /// Reset the mock state (clear all issues, reset counters).
    pub fn reset(&self) {
        self.issues.lock().unwrap().clear();
        *self.next_number.lock().unwrap() = 1;
        *self.simulate_errors.lock().unwrap() = None;
    }

    /// Check if we should simulate an error.
    fn check_error(&self) -> Result<(), GitHubError> {
        if let Some(error) = &*self.simulate_errors.lock().unwrap() {
            return Err(match error {
                GitHubError::RateLimit { reset_at } => GitHubError::RateLimit {
                    reset_at: reset_at.clone(),
                },
                GitHubError::Auth {
                    message,
                    suggestion,
                } => GitHubError::Auth {
                    message: message.clone(),
                    suggestion: suggestion.clone(),
                },
                GitHubError::NotFound { resource } => GitHubError::NotFound {
                    resource: resource.clone(),
                },
                other => GitHubError::Api {
                    message: format!("Simulated error: {:?}", other),
                    status_code: None,
                },
            });
        }
        Ok(())
    }
}

impl Default for MockGitHubAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GitHubPort for MockGitHubAdapter {
    async fn create_issue(&self, request: CreateIssueRequest) -> Result<GitHubIssue, GitHubError> {
        self.check_error()?;

        // Get next issue number
        let mut next = self.next_number.lock().unwrap();
        let number = *next;
        *next += 1;
        drop(next);

        // Create issue
        let issue = GitHubIssue {
            number,
            url: format!(
                "https://api.github.com/repos/{}/issues/{}",
                self.repo, number
            ),
            html_url: format!("https://github.com/{}/issues/{}", self.repo, number),
            state: IssueState::Open,
            title: request.title,
            body: Some(request.body),
            labels: request.labels,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // Store in memory
        self.issues.lock().unwrap().insert(number, issue.clone());

        Ok(issue)
    }

    async fn get_issue(&self, number: u32) -> Result<GitHubIssue, GitHubError> {
        self.check_error()?;

        self.issues
            .lock()
            .unwrap()
            .get(&number)
            .cloned()
            .ok_or_else(|| GitHubError::NotFound {
                resource: format!("Issue #{}", number),
            })
    }

    async fn add_comment(&self, number: u32, body: String) -> Result<GitHubComment, GitHubError> {
        self.check_error()?;

        // Check issue exists
        if !self.issues.lock().unwrap().contains_key(&number) {
            return Err(GitHubError::NotFound {
                resource: format!("Issue #{}", number),
            });
        }

        // Create comment (simplified - not stored)
        Ok(GitHubComment {
            id: number as u64 * 1000, // Fake ID
            url: format!("https://github.com/{}/issues/{}#comment", self.repo, number),
            body,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn set_labels(&self, number: u32, labels: Vec<String>) -> Result<(), GitHubError> {
        self.check_error()?;

        let mut issues = self.issues.lock().unwrap();
        let issue = issues
            .get_mut(&number)
            .ok_or_else(|| GitHubError::NotFound {
                resource: format!("Issue #{}", number),
            })?;

        issue.labels = labels;
        issue.updated_at = chrono::Utc::now().to_rfc3339();

        Ok(())
    }

    async fn update_state(&self, number: u32, state: IssueState) -> Result<(), GitHubError> {
        self.check_error()?;

        let mut issues = self.issues.lock().unwrap();
        let issue = issues
            .get_mut(&number)
            .ok_or_else(|| GitHubError::NotFound {
                resource: format!("Issue #{}", number),
            })?;

        issue.state = state;
        issue.updated_at = chrono::Utc::now().to_rfc3339();

        Ok(())
    }

    async fn rate_limit(&self) -> Result<RateLimit, GitHubError> {
        self.check_error()?;
        Ok(self.rate_limit.lock().unwrap().clone())
    }

    async fn auth_status(&self) -> Result<AuthStatus, GitHubError> {
        self.check_error()?;
        Ok(self.auth_status.lock().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_issue() {
        let mock = MockGitHubAdapter::new();

        let issue = mock
            .create_issue(CreateIssueRequest {
                title: "Test Issue".to_string(),
                body: "Test body".to_string(),
                labels: vec!["bug".to_string()],
                assignees: vec![],
            })
            .await
            .unwrap();

        assert_eq!(issue.number, 1);
        assert_eq!(issue.title, "Test Issue");
        assert_eq!(issue.labels, vec!["bug"]);
        assert_eq!(issue.state, IssueState::Open);
    }

    #[tokio::test]
    async fn test_issue_numbers_increment() {
        let mock = MockGitHubAdapter::new();

        let issue1 = mock
            .create_issue(CreateIssueRequest {
                title: "First".to_string(),
                body: "".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await
            .unwrap();

        let issue2 = mock
            .create_issue(CreateIssueRequest {
                title: "Second".to_string(),
                body: "".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await
            .unwrap();

        assert_eq!(issue1.number, 1);
        assert_eq!(issue2.number, 2);
    }

    #[tokio::test]
    async fn test_get_issue() {
        let mock = MockGitHubAdapter::new();

        let created = mock
            .create_issue(CreateIssueRequest {
                title: "Test".to_string(),
                body: "Body".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await
            .unwrap();

        let fetched = mock.get_issue(created.number).await.unwrap();

        assert_eq!(fetched.number, created.number);
        assert_eq!(fetched.title, "Test");
    }

    #[tokio::test]
    async fn test_get_nonexistent_issue() {
        let mock = MockGitHubAdapter::new();

        let result = mock.get_issue(999).await;

        assert!(matches!(result, Err(GitHubError::NotFound { .. })));
    }

    #[tokio::test]
    async fn test_set_labels() {
        let mock = MockGitHubAdapter::new();

        let issue = mock
            .create_issue(CreateIssueRequest {
                title: "Test".to_string(),
                body: "".to_string(),
                labels: vec!["initial".to_string()],
                assignees: vec![],
            })
            .await
            .unwrap();

        mock.set_labels(issue.number, vec!["new".to_string(), "labels".to_string()])
            .await
            .unwrap();

        let updated = mock.get_issue(issue.number).await.unwrap();
        assert_eq!(updated.labels, vec!["new", "labels"]);
    }

    #[tokio::test]
    async fn test_update_state() {
        let mock = MockGitHubAdapter::new();

        let issue = mock
            .create_issue(CreateIssueRequest {
                title: "Test".to_string(),
                body: "".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await
            .unwrap();

        assert_eq!(issue.state, IssueState::Open);

        mock.update_state(issue.number, IssueState::Closed)
            .await
            .unwrap();

        let updated = mock.get_issue(issue.number).await.unwrap();
        assert_eq!(updated.state, IssueState::Closed);
    }

    #[tokio::test]
    async fn test_add_comment() {
        let mock = MockGitHubAdapter::new();

        let issue = mock
            .create_issue(CreateIssueRequest {
                title: "Test".to_string(),
                body: "".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await
            .unwrap();

        let comment = mock
            .add_comment(issue.number, "Test comment".to_string())
            .await
            .unwrap();

        assert_eq!(comment.body, "Test comment");
    }

    #[tokio::test]
    async fn test_rate_limit_error() {
        let mock = MockGitHubAdapter::new().with_rate_limit_exceeded();

        let result = mock
            .create_issue(CreateIssueRequest {
                title: "Test".to_string(),
                body: "".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await;

        assert!(matches!(result, Err(GitHubError::RateLimit { .. })));
    }

    #[tokio::test]
    async fn test_auth_failure() {
        let mock = MockGitHubAdapter::new().with_auth_failure();

        let result = mock
            .create_issue(CreateIssueRequest {
                title: "Test".to_string(),
                body: "".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await;

        assert!(matches!(result, Err(GitHubError::Auth { .. })));
    }

    #[tokio::test]
    async fn test_get_all_issues() {
        let mock = MockGitHubAdapter::new();

        mock.create_issue(CreateIssueRequest {
            title: "First".to_string(),
            body: "".to_string(),
            labels: vec![],
            assignees: vec![],
        })
        .await
        .unwrap();

        mock.create_issue(CreateIssueRequest {
            title: "Second".to_string(),
            body: "".to_string(),
            labels: vec![],
            assignees: vec![],
        })
        .await
        .unwrap();

        let all_issues = mock.get_all_issues();
        assert_eq!(all_issues.len(), 2);
        assert_eq!(all_issues[0].title, "First");
        assert_eq!(all_issues[1].title, "Second");
    }

    #[tokio::test]
    async fn test_reset() {
        let mock = MockGitHubAdapter::new();

        mock.create_issue(CreateIssueRequest {
            title: "Test".to_string(),
            body: "".to_string(),
            labels: vec![],
            assignees: vec![],
        })
        .await
        .unwrap();

        assert_eq!(mock.get_all_issues().len(), 1);

        mock.reset();

        assert_eq!(mock.get_all_issues().len(), 0);

        // Next issue should be #1 again
        let issue = mock
            .create_issue(CreateIssueRequest {
                title: "After reset".to_string(),
                body: "".to_string(),
                labels: vec![],
                assignees: vec![],
            })
            .await
            .unwrap();

        assert_eq!(issue.number, 1);
    }
}
