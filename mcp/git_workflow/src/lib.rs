//! git-workflow - Automates git workflow with conventional commits, smart staging, and change analysis based on RFC-001
//!
//! This module implements an MCP tool that automates git workflow with conventional commits, smart staging, and change analysis.

pub mod adapter;
pub mod config;
pub mod domain;
pub mod port;

// Re-export main types from domain
pub use domain::{
    ChangeAnalysis, ChangeGroup, CommitOptions, CommitPlan, CommitType, FileChange, FileStatus,
    GitCommand, GitWorkflowError, GitWorkflowInput, GitWorkflowOutput, GitWorkflowService,
};

// Re-export from other modules (will be implemented)
// pub use port::{GitWorkflowPort, ToolMetadata};
// pub use adapter::GitWorkflowAdapter;
// pub use config::GitWorkflowConfig;

/// Creates a new instance of the git-workflow tool
pub fn create_tool() -> GitWorkflowService {
    GitWorkflowService::new()
}
