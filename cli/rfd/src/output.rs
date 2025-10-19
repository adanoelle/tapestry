//! Output formatting for the RFD CLI.
//!
//! This module handles formatting output in three modes:
//! - **Pretty**: Human-readable with colors and formatting
//! - **JSON**: Structured data for AI agents and automation
//! - **Quiet**: Minimal output (errors only)
//!
//! # Output Modes
//!
//! The output mode is selected via the `--format` flag:
//!
//! ```bash
//! rfd list --format pretty  # Default: colorized, human-friendly
//! rfd list --format json    # Structured JSON for parsing
//! rfd list --format quiet   # Silent unless there's an error
//! ```
//!
//! # Design for AI Agents
//!
//! JSON output is carefully designed for AI agents:
//! - **Consistent structure**: All commands return predictable JSON schemas
//! - **Complete data**: No information loss compared to pretty mode
//! - **Machine-readable errors**: Errors include error codes and suggestions
//! - **No ANSI codes**: JSON output never includes terminal escape sequences
//!
//! # Examples
//!
//! ```rust
//! use rfd::output::{Output, OutputFormat};
//! use rfd::document::RfdSummary;
//!
//! let output = Output::new(OutputFormat::Pretty);
//!
//! // Print success message
//! output.success("Operation completed");
//!
//! // Print error with suggestions
//! // output.error(&some_error);
//!
//! // Format list as JSON
//! let rfds: Vec<RfdSummary> = vec![];
//! // output.list(&rfds)?;
//! ```
//!
//! # Color Output
//!
//! Pretty mode uses colors to improve readability:
//! - **Green** (✓): Success messages
//! - **Red** (✗): Errors
//! - **Yellow**: Warnings and suggestions
//! - **Blue**: Draft state
//! - **Green**: Accepted/Implemented states
//! - **Cyan**: Review state
//! - **Gray**: Archived state
//!
//! # For Junior Developers
//!
//! This module demonstrates:
//! - **Format Abstraction**: Single API for multiple output formats
//! - **Builder Pattern**: Output struct encapsulates formatting logic
//! - **Colorization**: Using the `colored` crate for terminal colors
//! - **JSON Serialization**: Converting domain types to JSON
//!
//! Key insight: By abstracting output formatting, we can add new formats
//! (like YAML or CSV) without changing any command code.

use colored::*;
use serde::Serialize;
use std::fmt;

use crate::document::{RfdDocument, RfdState, RfdSummary};
use crate::error::RfdError;

/// Output format for CLI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable output with colors
    Pretty,
    /// JSON output for agents
    Json,
    /// Minimal output (errors only)
    Quiet,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(OutputFormat::Pretty),
            "json" => Ok(OutputFormat::Json),
            "quiet" => Ok(OutputFormat::Quiet),
            _ => Err(format!(
                "Invalid format: '{}'. Valid formats: pretty, json, quiet",
                s
            )),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Pretty => write!(f, "pretty"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Quiet => write!(f, "quiet"),
        }
    }
}

/// Output handler for formatting responses
pub struct Output {
    format: OutputFormat,
}

impl Output {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Print a success message
    pub fn success(&self, message: &str) {
        match self.format {
            OutputFormat::Pretty => {
                println!("{} {}", "✓".green().bold(), message);
            }
            OutputFormat::Json => {
                // Success messages in JSON mode are typically part of the response
            }
            OutputFormat::Quiet => {
                // Silent in quiet mode
            }
        }
    }

    /// Print an error
    pub fn error(&self, error: &RfdError) {
        match self.format {
            OutputFormat::Pretty => {
                eprintln!("{} {}", "✗".red().bold(), error);

                // Show suggestion if available
                let response = error.to_response();
                if let Some(suggestion) = response.suggestion {
                    eprintln!();
                    eprintln!("{}", "Suggestion:".yellow().bold());
                    eprintln!("  {}", suggestion.description);
                    eprintln!("  {}", suggestion.command.bright_black());
                }
            }
            OutputFormat::Json => {
                let response = error.to_response();
                if let Ok(json) = serde_json::to_string_pretty(&response) {
                    eprintln!("{}", json);
                }
            }
            OutputFormat::Quiet => {
                eprintln!("{}", error);
            }
        }
    }

    /// Print a JSON response
    pub fn json<T: Serialize>(&self, data: &T) -> Result<(), RfdError> {
        let json = serde_json::to_string_pretty(data).map_err(|e| RfdError::InvalidInput {
            message: format!("Failed to serialize to JSON: {}", e),
        })?;
        println!("{}", json);
        Ok(())
    }

    /// Print RFD list
    pub fn list(&self, rfds: &[RfdSummary]) -> Result<(), RfdError> {
        match self.format {
            OutputFormat::Pretty => {
                if rfds.is_empty() {
                    println!("{}", "No RFDs found".bright_black());
                    return Ok(());
                }

                println!("{}", format!("Found {} RFD(s)", rfds.len()).bold());
                println!();

                for rfd in rfds {
                    self.print_rfd_summary(rfd);
                    println!();
                }
            }
            OutputFormat::Json => {
                let response = serde_json::json!({
                    "rfds": rfds,
                    "total": rfds.len(),
                });
                self.json(&response)?;
            }
            OutputFormat::Quiet => {
                // Silent
            }
        }
        Ok(())
    }

    /// Print a single RFD summary
    fn print_rfd_summary(&self, rfd: &RfdSummary) {
        let state_colored = self.colorize_state(&rfd.state);

        println!(
            "{} {} {}",
            format!("RFD {}", rfd.id).bold(),
            state_colored,
            rfd.title
        );

        println!(
            "  {} {}",
            "Authors:".bright_black(),
            rfd.authors.join(", ")
        );

        println!(
            "  {} {}  {} {}",
            "Created:".bright_black(),
            rfd.created.format("%Y-%m-%d"),
            "Updated:".bright_black(),
            rfd.updated.format("%Y-%m-%d")
        );

        if !rfd.tags.is_empty() {
            println!("  {} {}", "Tags:".bright_black(), rfd.tags.join(", "));
        }

        if let Some(ref discussion) = rfd.discussion {
            println!("  {} {}", "Discussion:".bright_black(), discussion);
        }

        println!("  {} {}", "Path:".bright_black(), rfd.path);
    }

    /// Print RFD details
    pub fn show(&self, doc: &RfdDocument) -> Result<(), RfdError> {
        match self.format {
            OutputFormat::Pretty => {
                let state_colored = self.colorize_state(&doc.metadata.state);

                println!(
                    "{} {} {}",
                    format!("RFD {}", doc.formatted_number()).bold(),
                    state_colored,
                    doc.metadata.title
                );
                println!();

                println!("{}", "Metadata".bold().underline());
                println!(
                    "  {:<12} {}",
                    "Authors:", doc.metadata.authors.join(", ")
                );
                println!(
                    "  {:<12} {}",
                    "State:",
                    self.colorize_state(&doc.metadata.state)
                );
                println!(
                    "  {:<12} {}",
                    "Created:", doc.metadata.created.format("%Y-%m-%d %H:%M UTC")
                );
                println!(
                    "  {:<12} {}",
                    "Updated:", doc.metadata.updated.format("%Y-%m-%d %H:%M UTC")
                );

                if !doc.metadata.tags.is_empty() {
                    println!("  {:<12} {}", "Tags:", doc.metadata.tags.join(", "));
                }

                if let Some(ref discussion) = doc.metadata.discussion {
                    println!("  {:<12} {}", "Discussion:", discussion);
                }

                println!("  {:<12} {}", "Path:", doc.path.display());

                println!();
                println!("{}", "Content".bold().underline());
                println!("{}", doc.content);
            }
            OutputFormat::Json => {
                let response = serde_json::json!({
                    "id": doc.formatted_number(),
                    "metadata": doc.metadata,
                    "content": doc.content,
                    "path": doc.path,
                });
                self.json(&response)?;
            }
            OutputFormat::Quiet => {
                // Just print content in quiet mode
                println!("{}", doc.content);
            }
        }
        Ok(())
    }

    /// Print validation results
    pub fn validation(&self, rfd_id: &str, issues: &[String]) -> Result<(), RfdError> {
        match self.format {
            OutputFormat::Pretty => {
                if issues.is_empty() {
                    println!(
                        "{} RFD {} is valid",
                        "✓".green().bold(),
                        rfd_id.bold()
                    );
                } else {
                    println!(
                        "{} RFD {} has {} issue(s)",
                        "✗".red().bold(),
                        rfd_id.bold(),
                        issues.len()
                    );
                    println!();
                    for (i, issue) in issues.iter().enumerate() {
                        println!("  {}. {}", i + 1, issue);
                    }
                }
            }
            OutputFormat::Json => {
                let response = serde_json::json!({
                    "valid": issues.is_empty(),
                    "issues": issues,
                });
                self.json(&response)?;
            }
            OutputFormat::Quiet => {
                if !issues.is_empty() {
                    for issue in issues {
                        eprintln!("{}", issue);
                    }
                }
            }
        }
        Ok(())
    }

    /// Print create result
    pub fn created(&self, doc: &RfdDocument) -> Result<(), RfdError> {
        match self.format {
            OutputFormat::Pretty => {
                println!(
                    "{} Created RFD {} {}",
                    "✓".green().bold(),
                    doc.formatted_number().bold(),
                    doc.metadata.title
                );
                println!("  {} {}", "Path:".bright_black(), doc.path.display());
            }
            OutputFormat::Json => {
                let response = serde_json::json!({
                    "id": doc.formatted_number(),
                    "title": doc.metadata.title,
                    "path": doc.path,
                    "state": doc.metadata.state,
                });
                self.json(&response)?;
            }
            OutputFormat::Quiet => {
                // Silent
            }
        }
        Ok(())
    }

    /// Print status update result
    pub fn status_updated(&self, doc: &RfdDocument, old_state: RfdState) -> Result<(), RfdError> {
        match self.format {
            OutputFormat::Pretty => {
                if old_state == doc.metadata.state {
                    println!(
                        "{} RFD {} already in {} state",
                        "✓".green().bold(),
                        doc.formatted_number().bold(),
                        self.colorize_state(&doc.metadata.state)
                    );
                } else {
                    println!(
                        "{} Updated RFD {} state: {} → {}",
                        "✓".green().bold(),
                        doc.formatted_number().bold(),
                        self.colorize_state(&old_state),
                        self.colorize_state(&doc.metadata.state)
                    );
                }
            }
            OutputFormat::Json => {
                let response = serde_json::json!({
                    "id": doc.formatted_number(),
                    "old_state": old_state,
                    "new_state": doc.metadata.state,
                    "changed": old_state != doc.metadata.state,
                });
                self.json(&response)?;
            }
            OutputFormat::Quiet => {
                // Silent
            }
        }
        Ok(())
    }

    /// Colorize RFD state for display
    fn colorize_state(&self, state: &RfdState) -> ColoredString {
        let state_str = format!("[{}]", state);
        match state {
            RfdState::Draft => state_str.blue(),
            RfdState::Review => state_str.yellow(),
            RfdState::Accepted => state_str.green(),
            RfdState::Rejected => state_str.red(),
            RfdState::Implemented => state_str.cyan(),
            RfdState::Archived => state_str.bright_black(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_parsing() {
        assert_eq!("pretty".parse::<OutputFormat>().unwrap(), OutputFormat::Pretty);
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("quiet".parse::<OutputFormat>().unwrap(), OutputFormat::Quiet);
        assert!("invalid".parse::<OutputFormat>().is_err());
    }
}
