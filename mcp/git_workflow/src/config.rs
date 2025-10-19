//! Configuration for git-workflow

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Configuration for git-workflow
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitWorkflowConfig {
    /// Enable debug logging
    pub debug: bool,

    /// Timeout in seconds
    pub timeout_seconds: u64,
    // TODO: Add tool-specific configuration
}

impl Default for GitWorkflowConfig {
    fn default() -> Self {
        Self {
            debug: false,
            timeout_seconds: 30,
        }
    }
}

impl GitWorkflowConfig {
    /// Loads configuration from environment
    pub fn from_env() -> Result<Self> {
        let debug = std::env::var("GIT_WORKFLOW_DEBUG")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);

        let timeout_seconds = std::env::var("GIT_WORKFLOW_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        Ok(Self {
            debug,
            timeout_seconds,
        })
    }
}
