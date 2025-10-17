//! MCP adapter for git-workflow

use anyhow::{Context, Result};
use async_trait::async_trait;
use tracing::{debug, info};

use super::domain::{GitWorkflowInput, GitWorkflowOutput, GitWorkflowService};
use super::port::{GitWorkflowPort, ToolMetadata};

/// MCP adapter for git-workflow
pub struct GitWorkflowAdapter {
    service: GitWorkflowService,
    metadata: ToolMetadata,
}

impl GitWorkflowAdapter {
    /// Creates a new MCP adapter
    pub fn new() -> Self {
        Self {
            service: GitWorkflowService::new(),
            metadata: ToolMetadata::default(),
        }
    }
}

impl Default for GitWorkflowAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GitWorkflowPort for GitWorkflowAdapter {
    async fn execute(&self, input: GitWorkflowInput) -> Result<GitWorkflowOutput> {
        debug!("Executing git-workflow with input: {:?}", input);

        let result = self
            .service
            .execute(input)
            .context("Failed to execute git-workflow")?;

        info!("git-workflow execution successful");
        Ok(result)
    }

    async fn metadata(&self) -> ToolMetadata {
        self.metadata.clone()
    }
}

// TODO: Implement rmcp::Tool trait when rmcp is added as dependency
// #[rmcp::tool(
//     name = "git-workflow",
//     description = "Automates git workflow with conventional commits, smart staging, and change analysis based on RFC-001"
// )]
// impl Tool for GitworkflowAdapter {
//     type Input = GitworkflowInput;
//     type Output = GitworkflowOutput;
//
//     async fn run(&self, input: Self::Input) -> ToolResult<Self::Output> {
//         self.execute(input)
//             .await
//             .map_err(|e| rmcp::Error::Tool(e.to_string()))
//     }
// }
