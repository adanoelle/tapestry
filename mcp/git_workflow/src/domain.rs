//! Domain logic for git-workflow
//!
//! Automates git workflow with conventional commits, smart staging, and change analysis based on RFC-001

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use thiserror::Error;

/// Conventional commit types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommitType {
    Feat,     // New feature
    Fix,      // Bug fix
    Docs,     // Documentation only
    Style,    // Code style (formatting, semicolons, etc)
    Refactor, // Code change that neither fixes bug nor adds feature
    Test,     // Adding missing tests
    Chore,    // Maintain (deps, config, etc)
    Perf,     // Performance improvement
    Ci,       // CI/CD changes
    Build,    // Build system changes
    Revert,   // Revert previous commit
}

impl CommitType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Docs => "docs",
            CommitType::Style => "style",
            CommitType::Refactor => "refactor",
            CommitType::Test => "test",
            CommitType::Chore => "chore",
            CommitType::Perf => "perf",
            CommitType::Ci => "ci",
            CommitType::Build => "build",
            CommitType::Revert => "revert",
        }
    }
}

/// A single file change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: PathBuf,
    pub status: FileStatus,
    pub additions: usize,
    pub deletions: usize,
}

/// Git file status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Untracked,
}

/// Group of related changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeGroup {
    pub name: String,
    pub description: String,
    pub files: Vec<FileChange>,
    pub suggested_type: CommitType,
    pub suggested_scope: Option<String>,
}

/// Analysis of repository changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAnalysis {
    pub groups: Vec<ChangeGroup>,
    pub ungrouped_files: Vec<FileChange>,
    pub breaking_changes: Vec<String>,
    pub suggested_order: Vec<usize>, // Indices into groups
}

/// Options for preparing a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitOptions {
    pub commit_type: Option<CommitType>,
    pub scope: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub breaking: bool,
    pub issues: Vec<String>,
    pub staged_files: Vec<PathBuf>,
}

/// A planned commit with all details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitPlan {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub breaking_change: Option<String>,
    pub issues: Vec<String>,
    pub files_to_stage: Vec<PathBuf>,
    pub formatted_message: String,
}

/// Input for the git-workflow tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitWorkflowInput {
    pub command: GitCommand,
    pub repository_path: Option<PathBuf>,
}

/// Commands the tool can execute
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GitCommand {
    AnalyzeChanges,
    SuggestStaging,
    PrepareCommit(CommitOptions),
    ValidateCommit { message: String },
}

/// Output from the git-workflow tool
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GitWorkflowOutput {
    Analysis(ChangeAnalysis),
    StagingGroups(Vec<ChangeGroup>),
    CommitPlan(CommitPlan),
    ValidationResult { valid: bool, issues: Vec<String> },
}

/// Errors specific to git-workflow
#[derive(Error, Debug)]
pub enum GitWorkflowError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("No changes to commit")]
    NoChanges,

    #[error("Invalid commit message: {0}")]
    InvalidCommitMessage(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Core domain service for git-workflow
#[derive(Debug, Clone)]
pub struct GitWorkflowService {
    // Domain service is pure - no external dependencies
}

impl GitWorkflowService {
    /// Creates a new instance of the service
    pub fn new() -> Self {
        Self {}
    }

    /// Analyzes changes and groups them intelligently
    pub fn analyze_changes(
        &self,
        files: Vec<FileChange>,
    ) -> Result<ChangeAnalysis, GitWorkflowError> {
        if files.is_empty() {
            return Err(GitWorkflowError::NoChanges);
        }

        let mut groups = Vec::new();
        let mut grouped_files = HashSet::new();

        // Group 1: Documentation changes
        let doc_files: Vec<FileChange> = files
            .iter()
            .filter(|f| {
                f.path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == "md" || e == "txt" || e == "rst")
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if !doc_files.is_empty() {
            for f in &doc_files {
                grouped_files.insert(f.path.clone());
            }
            groups.push(ChangeGroup {
                name: "Documentation".to_string(),
                description: "Documentation updates".to_string(),
                files: doc_files,
                suggested_type: CommitType::Docs,
                suggested_scope: None,
            });
        }

        // Group 2: Test files
        let test_files: Vec<FileChange> = files
            .iter()
            .filter(|f| {
                f.path
                    .to_str()
                    .map(|p| p.contains("test") || p.contains("spec"))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if !test_files.is_empty() {
            for f in &test_files {
                grouped_files.insert(f.path.clone());
            }
            groups.push(ChangeGroup {
                name: "Tests".to_string(),
                description: "Test updates".to_string(),
                files: test_files,
                suggested_type: CommitType::Test,
                suggested_scope: None,
            });
        }

        // Group 3: CI/CD files
        let ci_files: Vec<FileChange> = files
            .iter()
            .filter(|f| {
                f.path
                    .to_str()
                    .map(|p| p.contains(".github") || p.contains("ci") || p.contains("gitlab"))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if !ci_files.is_empty() {
            for f in &ci_files {
                grouped_files.insert(f.path.clone());
            }
            groups.push(ChangeGroup {
                name: "CI/CD".to_string(),
                description: "CI/CD configuration".to_string(),
                files: ci_files,
                suggested_type: CommitType::Ci,
                suggested_scope: None,
            });
        }

        // Collect ungrouped files
        let ungrouped_files: Vec<FileChange> = files
            .into_iter()
            .filter(|f| !grouped_files.contains(&f.path))
            .collect();

        // Detect breaking changes (simplified heuristic)
        let breaking_changes = self.detect_breaking_changes(&groups, &ungrouped_files);

        // Suggest commit order (docs last, tests before features)
        let mut suggested_order: Vec<usize> = (0..groups.len()).collect();
        suggested_order.sort_by_key(|&i| match groups[i].suggested_type {
            CommitType::Fix => 0,
            CommitType::Feat => 1,
            CommitType::Test => 2,
            CommitType::Docs => 3,
            _ => 4,
        });

        Ok(ChangeAnalysis {
            groups,
            ungrouped_files,
            breaking_changes,
            suggested_order,
        })
    }

    /// Prepares a commit plan
    pub fn prepare_commit(&self, options: CommitOptions) -> Result<CommitPlan, GitWorkflowError> {
        if options.subject.is_empty() {
            return Err(GitWorkflowError::InvalidInput(
                "Subject cannot be empty".to_string(),
            ));
        }

        let commit_type = options.commit_type.unwrap_or(CommitType::Feat);

        // Format the commit message
        let mut message = String::from(commit_type.as_str());

        if let Some(ref scope) = options.scope {
            message.push('(');
            message.push_str(scope);
            message.push(')');
        }

        message.push_str(": ");
        message.push_str(&options.subject);

        if let Some(ref body) = options.body {
            message.push_str("\n\n");
            message.push_str(body);
        }

        let breaking_change = if options.breaking {
            message.push_str("\n\nBREAKING CHANGE: ");
            if let Some(ref body) = options.body {
                message.push_str(body);
            }
            options.body.clone()
        } else {
            None
        };

        if !options.issues.is_empty() {
            message.push_str("\n\n");
            for issue in &options.issues {
                message.push_str("Closes #");
                message.push_str(issue);
                message.push('\n');
            }
        }

        Ok(CommitPlan {
            commit_type,
            scope: options.scope,
            subject: options.subject,
            body: options.body,
            breaking_change,
            issues: options.issues,
            files_to_stage: options.staged_files,
            formatted_message: message,
        })
    }

    /// Validates a commit message
    pub fn validate_commit_message(&self, message: &str) -> Result<bool, GitWorkflowError> {
        if message.is_empty() {
            return Ok(false);
        }

        // Check if it follows conventional commit format
        let first_line = message.lines().next().unwrap_or("");

        // Simple regex-like check (in real implementation, use regex crate)
        let valid_types = [
            "feat", "fix", "docs", "style", "refactor", "test", "chore", "perf", "ci", "build",
            "revert",
        ];
        let has_valid_type = valid_types.iter().any(|&t| first_line.starts_with(t));

        Ok(has_valid_type)
    }

    /// Executes the core logic
    pub fn execute(&self, input: GitWorkflowInput) -> Result<GitWorkflowOutput, GitWorkflowError> {
        match input.command {
            GitCommand::AnalyzeChanges => {
                // In real implementation, this would get files from the port
                // For now, return empty analysis
                Err(GitWorkflowError::InvalidInput(
                    "AnalyzeChanges requires file list from adapter".to_string(),
                ))
            }
            GitCommand::SuggestStaging => {
                // In real implementation, this would get files from the port
                Err(GitWorkflowError::InvalidInput(
                    "SuggestStaging requires file list from adapter".to_string(),
                ))
            }
            GitCommand::PrepareCommit(options) => {
                let plan = self.prepare_commit(options)?;
                Ok(GitWorkflowOutput::CommitPlan(plan))
            }
            GitCommand::ValidateCommit { message } => {
                let valid = self.validate_commit_message(&message)?;
                let issues = if !valid {
                    vec!["Message doesn't follow conventional commit format".to_string()]
                } else {
                    vec![]
                };
                Ok(GitWorkflowOutput::ValidationResult { valid, issues })
            }
        }
    }

    // Helper method to detect breaking changes
    fn detect_breaking_changes(
        &self,
        _groups: &[ChangeGroup],
        ungrouped: &[FileChange],
    ) -> Vec<String> {
        let mut breaking = Vec::new();

        // Check for API changes (simplified)
        for file in ungrouped {
            if file
                .path
                .to_str()
                .map(|p| p.contains("api") || p.contains("public"))
                .unwrap_or(false)
            {
                breaking.push(format!("Potential API change in {:?}", file.path));
            }
        }

        breaking
    }
}

impl Default for GitWorkflowService {
    fn default() -> Self {
        Self::new()
    }
}
