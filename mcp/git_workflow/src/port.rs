//! Port definitions for git-workflow

use super::domain::{GitWorkflowInput, GitWorkflowOutput};
use anyhow::Result;
use async_trait::async_trait;

/// Port interface for git-workflow
#[async_trait]
pub trait GitWorkflowPort: Send + Sync {
    /// Executes the tool operation
    async fn execute(&self, input: GitWorkflowInput) -> Result<GitWorkflowOutput>;

    /// Returns tool metadata
    async fn metadata(&self) -> ToolMetadata;
}

/// Metadata about the tool
#[derive(Debug, Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}

impl Default for ToolMetadata {
    fn default() -> Self {
        Self {
            name: "git-workflow".to_string(),
            description: "Automates git workflow with conventional commits, smart staging, and change analysis based on RFC-001".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: "Tapestry Team".to_string(),
        }
    }
}
