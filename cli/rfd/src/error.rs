//! Error types with actionable suggestions for AI agents.
//!
//! This module defines all error types used by the RFD CLI, along with
//! structured error responses that include suggestions for how to fix problems.
//!
//! # Design Philosophy
//!
//! Errors in this CLI are designed to be agent-friendly:
//! 1. **Structured**: Each error has a code, message, details, and suggestion
//! 2. **Actionable**: Suggestions include commands to run or actions to take
//! 3. **Context-Rich**: Error details provide all information needed to debug
//! 4. **Exit Codes**: Different error types map to different exit codes
//!
//! # Error Categories
//!
//! - **NotFound** (exit 1): RFD doesn't exist
//! - **InvalidTransition** (exit 3): State transition not allowed
//! - **ValidationFailed** (exit 2): RFD structure invalid
//! - **InvalidInput** (exit 1): Bad user input
//! - **FileError** (exit 1): Filesystem operation failed
//! - **TemplateError** (exit 1): Template rendering failed
//! - **ConfigError** (exit 1): Configuration file invalid
//!
//! # Examples
//!
//! ```rust
//! use rfd::error::{RfdError, not_found};
//! use rfd::document::RfdState;
//!
//! // Create a "not found" error
//! let err = not_found("42");
//!
//! // Get structured error response for JSON output
//! let response = err.to_response();
//! assert_eq!(response.error, "NOT_FOUND");
//! assert!(response.suggestion.is_some());
//!
//! // Create invalid transition error
//! let err = RfdError::InvalidTransition {
//!     current: RfdState::Draft,
//!     target: RfdState::Archived,
//! };
//!
//! // Error includes suggestions for valid transitions
//! let response = err.to_response();
//! assert!(response.details.is_some());
//! ```
//!
//! # For Junior Developers
//!
//! This module demonstrates:
//! - **thiserror**: Deriving error types with custom messages
//! - **Error Context**: Including enough information to diagnose problems
//! - **API Design**: Making errors helpful for both humans and machines
//! - **Exit Codes**: Using process exit codes to communicate error types
//!
//! The key insight: errors should tell users *how to fix the problem*, not just
//! what went wrong. Notice how [`ErrorResponse`] includes a `suggestion` field.

use serde::Serialize;
use thiserror::Error;

use crate::document::RfdState;

/// Error types for RFD operations
#[derive(Error, Debug)]
pub enum RfdError {
    #[error("RFD {id} not found")]
    NotFound { id: String },

    #[error("Invalid state transition from '{current}' to '{target}'")]
    InvalidTransition { current: RfdState, target: RfdState },

    #[error("Validation failed")]
    ValidationFailed { issues: Vec<String> },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("File operation failed: {message}")]
    FileError { message: String },

    #[error("Failed to parse front matter: {message}")]
    FrontMatterError { message: String },

    #[error("Failed to parse YAML: {message}")]
    YamlError { message: String },

    #[error("Template error: {message}")]
    TemplateError { message: String },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Error response with suggestions for agents
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<Suggestion>,
}

/// Suggested action to fix the error
#[derive(Debug, Serialize)]
pub struct Suggestion {
    pub command: String,
    pub description: String,
}

impl RfdError {
    /// Convert error to structured response for agents
    pub fn to_response(&self) -> ErrorResponse {
        match self {
            RfdError::NotFound { id } => ErrorResponse {
                error: "NOT_FOUND".to_string(),
                message: format!("RFD {} not found", id),
                details: Some(serde_json::json!({
                    "id": id,
                })),
                suggestion: Some(Suggestion {
                    command: "rfd list --format json".to_string(),
                    description: "List all available RFDs".to_string(),
                }),
            },

            RfdError::InvalidTransition { current, target } => {
                let valid_next = current.valid_next_states();
                let suggestion_state = valid_next.first();

                ErrorResponse {
                    error: "INVALID_TRANSITION".to_string(),
                    message: format!("Cannot transition from '{}' to '{}'", current, target),
                    details: Some(serde_json::json!({
                        "current_state": current.to_string(),
                        "target_state": target.to_string(),
                        "valid_next_states": valid_next.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    })),
                    suggestion: suggestion_state.map(|state| Suggestion {
                        command: format!("rfd status <ID> --set {}", state),
                        description: format!("Transition to '{}' instead", state),
                    }),
                }
            }

            RfdError::ValidationFailed { issues } => ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: "RFD validation failed".to_string(),
                details: Some(serde_json::json!({
                    "issues": issues,
                })),
                suggestion: Some(Suggestion {
                    command: "rfd validate <ID> --format json".to_string(),
                    description: "See detailed validation issues".to_string(),
                }),
            },

            RfdError::InvalidInput { message } => ErrorResponse {
                error: "INVALID_INPUT".to_string(),
                message: message.clone(),
                details: None,
                suggestion: Some(Suggestion {
                    command: "rfd --help".to_string(),
                    description: "See usage instructions".to_string(),
                }),
            },

            RfdError::FileError { message } => ErrorResponse {
                error: "FILE_ERROR".to_string(),
                message: message.clone(),
                details: None,
                suggestion: Some(Suggestion {
                    command: "Check file permissions and path".to_string(),
                    description: "Ensure the rfds/ directory exists and is writable".to_string(),
                }),
            },

            RfdError::FrontMatterError { message } => ErrorResponse {
                error: "FRONT_MATTER_ERROR".to_string(),
                message: format!("Failed to parse front matter: {}", message),
                details: None,
                suggestion: Some(Suggestion {
                    command: "Check YAML front matter syntax".to_string(),
                    description: "Ensure the front matter is valid YAML between --- delimiters"
                        .to_string(),
                }),
            },

            RfdError::YamlError { message } => ErrorResponse {
                error: "YAML_ERROR".to_string(),
                message: format!("Failed to parse YAML: {}", message),
                details: None,
                suggestion: Some(Suggestion {
                    command: "Validate YAML syntax".to_string(),
                    description: "Check for proper indentation and formatting".to_string(),
                }),
            },

            RfdError::TemplateError { message } => ErrorResponse {
                error: "TEMPLATE_ERROR".to_string(),
                message: format!("Template error: {}", message),
                details: None,
                suggestion: Some(Suggestion {
                    command: "Check template file".to_string(),
                    description: "Ensure the template file exists and is valid Jinja2".to_string(),
                }),
            },

            RfdError::ConfigError { message } => ErrorResponse {
                error: "CONFIG_ERROR".to_string(),
                message: format!("Configuration error: {}", message),
                details: None,
                suggestion: Some(Suggestion {
                    command: "Check .rfd/config.toml".to_string(),
                    description: "Ensure configuration file is valid TOML".to_string(),
                }),
            },

            RfdError::Io(err) => ErrorResponse {
                error: "IO_ERROR".to_string(),
                message: format!("IO error: {}", err),
                details: None,
                suggestion: Some(Suggestion {
                    command: "Check file permissions".to_string(),
                    description: "Ensure files and directories are accessible".to_string(),
                }),
            },
        }
    }

    /// Get exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            RfdError::NotFound { .. } => 1,
            RfdError::InvalidTransition { .. } => 3,
            RfdError::ValidationFailed { .. } => 2,
            RfdError::InvalidInput { .. } => 1,
            RfdError::FileError { .. } => 1,
            RfdError::FrontMatterError { .. } => 2,
            RfdError::YamlError { .. } => 2,
            RfdError::TemplateError { .. } => 1,
            RfdError::ConfigError { .. } => 1,
            RfdError::Io(_) => 1,
        }
    }
}

/// Helper to create NotFound error
pub fn not_found(id: impl Into<String>) -> RfdError {
    RfdError::NotFound { id: id.into() }
}

/// Helper to create InvalidInput error
pub fn invalid_input(message: impl Into<String>) -> RfdError {
    RfdError::InvalidInput {
        message: message.into(),
    }
}

/// Helper to create FileError
pub fn file_error(message: impl Into<String>) -> RfdError {
    RfdError::FileError {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_serialization() {
        let err = RfdError::InvalidTransition {
            current: RfdState::Draft,
            target: RfdState::Archived,
        };

        let response = err.to_response();
        assert_eq!(response.error, "INVALID_TRANSITION");
        assert!(response.suggestion.is_some());
        assert!(response.details.is_some());
    }

    #[test]
    fn test_exit_codes() {
        assert_eq!(RfdError::ValidationFailed { issues: vec![] }.exit_code(), 2);
        assert_eq!(
            RfdError::InvalidTransition {
                current: RfdState::Draft,
                target: RfdState::Archived
            }
            .exit_code(),
            3
        );
        assert_eq!(RfdError::NotFound { id: "1".into() }.exit_code(), 1);
    }
}
