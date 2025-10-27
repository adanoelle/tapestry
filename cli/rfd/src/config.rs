//! Configuration management for the RFD CLI.
//!
//! This module handles loading and parsing configuration from `.rfd/config.toml` files,
//! with fallback to user-level config (`~/.config/rfd/config.toml`) and sensible defaults.
//!
//! # Configuration Hierarchy
//!
//! Configuration is loaded in priority order:
//! 1. **Project**: `.rfd/config.toml` (highest priority)
//! 2. **User**: `~/.config/rfd/config.toml`
//! 3. **Defaults**: Built-in sensible defaults
//!
//! # Configuration File Format
//!
//! ```toml
//! [rfd]
//! directory = "rfds"           # Where RFDs are stored
//! template = "default"         # Default template to use
//! id_format = "{:04d}"        # Number formatting (e.g., 0001, 0002)
//!
//! [metadata]
//! default_state = "draft"      # Initial state for new RFDs
//! required_fields = ["title", "authors", "summary"]
//!
//! [output]
//! default_format = "pretty"    # Output mode (pretty, json, quiet)
//! color = "auto"              # Color output (auto, always, never)
//! ```
//!
//! # Examples
//!
//! ```rust
//! use rfd::config::RfdConfig;
//!
//! // Load config (checks project, user, then uses defaults)
//! let config = RfdConfig::load()?;
//!
//! // Get RFD directory
//! let dir = config.rfd_directory();
//!
//! // Format an RFD number
//! let formatted = config.format_number(42)?;
//! assert_eq!(formatted, "0042");
//! ```
//!
//! # For Junior Developers
//!
//! This module demonstrates:
//! - **Config Hierarchies**: Multiple config sources with priority
//! - **Default Values**: Using functions for serde defaults
//! - **Path Manipulation**: Working with PathBuf and cross-platform paths
//! - **TOML Parsing**: Using serde for structured config files
//!
//! The pattern of checking project → user → defaults is common in CLI tools.
//! Notice how [`RfdConfig::load()`] handles this gracefully without errors
//! if config files don't exist.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::RfdError;

/// RFD tool configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RfdConfig {
    /// RFD configuration section
    #[serde(default)]
    pub rfd: RfdSection,

    /// Metadata configuration section
    #[serde(default)]
    pub metadata: MetadataSection,

    /// Output configuration section
    #[serde(default)]
    pub output: OutputSection,

    /// GitHub integration configuration section
    #[serde(default)]
    pub github: GitHubSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfdSection {
    /// Directory where RFDs are stored
    #[serde(default = "default_directory")]
    pub directory: String,

    /// Default template to use
    #[serde(default = "default_template")]
    pub template: String,

    /// ID format string (e.g., "{:04d}" for 0001, 0002, etc.)
    #[serde(default = "default_id_format")]
    pub id_format: String,

    /// Default author for new RFDs (e.g., "Name <email@example.com>")
    #[serde(default)]
    pub default_author: Option<String>,
}

impl Default for RfdSection {
    fn default() -> Self {
        Self {
            directory: default_directory(),
            template: default_template(),
            id_format: default_id_format(),
            default_author: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataSection {
    /// Default state for new RFDs
    #[serde(default = "default_state")]
    pub default_state: String,

    /// Required fields for validation
    #[serde(default = "default_required_fields")]
    pub required_fields: Vec<String>,
}

impl Default for MetadataSection {
    fn default() -> Self {
        Self {
            default_state: default_state(),
            required_fields: default_required_fields(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSection {
    /// Default output format
    #[serde(default = "default_output_format")]
    pub default_format: String,

    /// Color output setting
    #[serde(default = "default_color")]
    pub color: String,
}

impl Default for OutputSection {
    fn default() -> Self {
        Self {
            default_format: default_output_format(),
            color: default_color(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubSection {
    /// Repository in "owner/repo" format (e.g., "adanoelle/tapestry")
    #[serde(default = "default_github_repo")]
    pub repo: String,

    /// Auto-create GitHub issue when creating RFD
    #[serde(default)]
    pub auto_create_issue: bool,

    /// Auto-sync RFD status changes to GitHub
    #[serde(default)]
    pub auto_sync_status: bool,

    /// Label mapping for RFD states
    #[serde(default)]
    pub label_mapping: std::collections::HashMap<String, Vec<String>>,
}

impl Default for GitHubSection {
    fn default() -> Self {
        Self {
            repo: default_github_repo(),
            auto_create_issue: false,
            auto_sync_status: false,
            label_mapping: default_label_mapping(),
        }
    }
}

// Default value functions
fn default_directory() -> String {
    "rfds".to_string()
}

fn default_template() -> String {
    "default".to_string()
}

fn default_id_format() -> String {
    "{:04d}".to_string()
}

fn default_state() -> String {
    "draft".to_string()
}

fn default_required_fields() -> Vec<String> {
    vec![
        "title".to_string(),
        "authors".to_string(),
        "summary".to_string(),
    ]
}

fn default_output_format() -> String {
    "pretty".to_string()
}

fn default_color() -> String {
    "auto".to_string()
}

fn default_github_repo() -> String {
    "".to_string()
}

fn default_label_mapping() -> std::collections::HashMap<String, Vec<String>> {
    let mut map = std::collections::HashMap::new();
    map.insert("draft".to_string(), vec!["rfd:draft".to_string()]);
    map.insert("review".to_string(), vec!["rfd:review".to_string()]);
    map.insert("accepted".to_string(), vec!["rfd:accepted".to_string()]);
    map.insert("implemented".to_string(), vec!["rfd:implemented".to_string()]);
    map.insert("rejected".to_string(), vec!["rfd:rejected".to_string()]);
    map.insert("archived".to_string(), vec!["rfd:archived".to_string()]);
    map
}

impl RfdConfig {
    /// Load configuration from file, falling back to defaults
    pub fn load() -> Result<Self, RfdError> {
        // Try project-local config first
        let project_config = Path::new(".rfd/config.toml");
        if project_config.exists() {
            return Self::load_from_path(project_config);
        }

        // Try user config
        if let Some(user_config) = Self::user_config_path() {
            if user_config.exists() {
                return Self::load_from_path(&user_config);
            }
        }

        // Use defaults
        Ok(Self::default())
    }

    /// Load configuration from a specific path
    pub fn load_from_path(path: &Path) -> Result<Self, RfdError> {
        let contents = fs::read_to_string(path).map_err(|e| RfdError::ConfigError {
            message: format!("Failed to read config file: {}", e),
        })?;

        toml::from_str(&contents).map_err(|e| RfdError::ConfigError {
            message: format!("Failed to parse config file: {}", e),
        })
    }

    /// Get the user config path (~/.config/rfd/config.toml)
    pub fn user_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut p| {
            p.push("rfd");
            p.push("config.toml");
            p
        })
    }

    /// Get the RFD directory as a PathBuf
    pub fn rfd_directory(&self) -> PathBuf {
        PathBuf::from(&self.rfd.directory)
    }

    /// Format an RFD number using the configured format
    pub fn format_number(&self, number: u32) -> Result<String, RfdError> {
        // Parse format string (e.g., "{:04d}")
        if !self.rfd.id_format.contains("{") || !self.rfd.id_format.contains("}") {
            return Err(RfdError::ConfigError {
                message: format!("Invalid id_format: {}", self.rfd.id_format),
            });
        }

        // Simple implementation - assumes format like "{:04d}"
        // Extract padding from format string
        //
        // For junior developers: This parses the format string to extract the padding width.
        // For example, "{:04d}" means "pad with zeros to width 4".
        let format_str = &self.rfd.id_format;
        if format_str.starts_with("{:") && format_str.ends_with("d}") {
            // Example: "{:04d}" → extract "04" from position 2 to len-2
            // "{:04d}"
            //   ^^   <- skip these (position 0-1)
            //     ^^  <- extract this (position 2 to len-2)
            //       ^ <- skip this (position len-1)
            let padding_str = &format_str[2..format_str.len() - 2];

            // Parse "04" → 4 (as usize)
            let padding: usize = padding_str.parse().map_err(|_| RfdError::ConfigError {
                message: format!(
                    "Invalid padding in id_format '{}': '{}' is not a valid number. \
                         Expected format like '{{:04d}}' where '04' is the padding width.",
                    format_str, padding_str
                ),
            })?;

            // Format number with zero padding
            // Example: number=42, padding=4 → "0042"
            Ok(format!("{:0width$}", number, width = padding))
        } else {
            // Invalid format - return error instead of silent fallback
            Err(RfdError::ConfigError {
                message: format!(
                    "Invalid id_format '{}'. Expected format like '{{:04d}}' for zero-padded numbers.",
                    format_str
                ),
            })
        }
    }

    /// Get template directories to search (in priority order)
    pub fn template_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // 1. Project templates
        paths.push(PathBuf::from(".rfd/templates"));

        // 2. User templates
        if let Some(config_dir) = dirs::config_dir() {
            let mut p = config_dir;
            p.push("rfd");
            p.push("templates");
            paths.push(p);
        }

        // Built-in templates are embedded in the binary, not filesystem paths

        paths
    }
}

/// Helper to create .rfd directory structure.
///
/// Currently unused but kept for future use (e.g., `rfd init` command).
/// This will be useful when we add a setup command to initialize RFD directories.
#[allow(dead_code)]
pub fn initialize_rfd_directory(config: &RfdConfig) -> Result<(), RfdError> {
    let rfd_dir = config.rfd_directory();

    if !rfd_dir.exists() {
        fs::create_dir_all(&rfd_dir).map_err(|e| RfdError::FileError {
            message: format!("Failed to create RFD directory: {}", e),
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RfdConfig::default();
        assert_eq!(config.rfd.directory, "rfds");
        assert_eq!(config.rfd.template, "default");
        assert_eq!(config.rfd.id_format, "{:04d}");
        assert_eq!(config.metadata.default_state, "draft");
    }

    #[test]
    fn test_format_number() {
        let config = RfdConfig::default();
        assert_eq!(config.format_number(1).unwrap(), "0001");
        assert_eq!(config.format_number(42).unwrap(), "0042");
        assert_eq!(config.format_number(999).unwrap(), "0999");
        assert_eq!(config.format_number(1000).unwrap(), "1000");
    }

    #[test]
    fn test_template_paths() {
        let config = RfdConfig::default();
        let paths = config.template_paths();

        // Should have at least project path
        assert!(!paths.is_empty());
        assert_eq!(paths[0], PathBuf::from(".rfd/templates"));
    }
}
