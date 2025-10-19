//! File system operations for RFD documents.
//!
//! This module handles all file I/O operations including:
//! - Finding and loading RFD files
//! - Parsing YAML frontmatter
//! - Atomic file writes (using temp files)
//! - Directory scanning and filtering
//!
//! # YAML Frontmatter Format
//!
//! RFD files use YAML frontmatter (similar to Jekyll):
//!
//! ```markdown
//! ---
//! title: "My Proposal"
//! authors: ["Alice <alice@example.com>"]
//! state: draft
//! created: 2025-10-19T00:00:00Z
//! updated: 2025-10-19T00:00:00Z
//! ---
//!
//! # Summary
//! Document content here...
//! ```
//!
//! # Atomic Writes
//!
//! To prevent file corruption, this module uses atomic writes:
//! 1. Write to temporary file (`file.md.tmp`)
//! 2. Rename to final name (`file.md`)
//!
//! The rename operation is atomic on POSIX systems, ensuring either
//! the old or new file exists, never a partial write.
//!
//! # Examples
//!
//! ```rust,no_run
//! use rfd::fs::{find_all_rfds, load_rfd, save_rfd};
//! use rfd::config::RfdConfig;
//!
//! let config = RfdConfig::default();
//!
//! // Find all RFD files
//! let files = find_all_rfds(&config)?;
//!
//! // Load an RFD
//! if let Some(path) = files.first() {
//!     let doc = load_rfd(path)?;
//!     println!("RFD {}: {}", doc.number, doc.metadata.title);
//! }
//! ```
//!
//! # For Junior Developers
//!
//! This module demonstrates:
//! - **File I/O Safety**: Atomic writes prevent corruption
//! - **YAML Parsing**: Extracting structured data from text files
//! - **Error Handling**: Providing context when file operations fail
//! - **Functional Patterns**: Using iterators and filters
//!
//! Key insight: Always use atomic writes for important data. The temp file + rename
//! pattern is a standard technique to ensure data integrity.

use gray_matter::engine::YAML;
use gray_matter::Matter;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::RfdConfig;
use crate::document::{RfdDocument, RfdMetadata};
use crate::error::{file_error, not_found, RfdError};

/// Find all RFD files in the configured directory
pub fn find_all_rfds(config: &RfdConfig) -> Result<Vec<PathBuf>, RfdError> {
    let rfd_dir = config.rfd_directory();

    if !rfd_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&rfd_dir).map_err(|e| file_error(format!(
        "Failed to read RFD directory '{}': {}",
        rfd_dir.display(),
        e
    )))?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| file_error(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();

        // Only include markdown files
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            files.push(path);
        }
    }

    // Sort by filename (which starts with number)
    files.sort();

    Ok(files)
}

/// Find an RFD by ID (handles various formats: "1", "001", "0001")
pub fn find_rfd_by_id(config: &RfdConfig, id: &str) -> Result<PathBuf, RfdError> {
    // Parse the ID as a number
    let number: u32 = id.parse().map_err(|_| RfdError::InvalidInput {
        message: format!("Invalid RFD ID: '{}'. Expected a number.", id),
    })?;

    // Get all RFDs and search for matching number
    let files = find_all_rfds(config)?;

    for path in files {
        if let Some(file_number) = extract_rfd_number(&path) {
            if file_number == number {
                return Ok(path);
            }
        }
    }

    Err(not_found(id))
}

/// Extract RFD number from filename (e.g., "0042-feature.md" -> 42)
pub fn extract_rfd_number(path: &Path) -> Option<u32> {
    path.file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| {
            // Split on first hyphen
            let parts: Vec<&str> = name.splitn(2, '-').collect();
            if parts.len() == 2 {
                parts[0].parse::<u32>().ok()
            } else {
                None
            }
        })
}

/// Get the next available RFD number
pub fn next_rfd_number(config: &RfdConfig) -> Result<u32, RfdError> {
    let files = find_all_rfds(config)?;

    let max_number = files
        .iter()
        .filter_map(|path| extract_rfd_number(path))
        .max()
        .unwrap_or(0);

    Ok(max_number + 1)
}

/// Load an RFD document from file
pub fn load_rfd(path: &Path) -> Result<RfdDocument, RfdError> {
    let content = fs::read_to_string(path).map_err(|e| file_error(format!(
        "Failed to read file '{}': {}",
        path.display(),
        e
    )))?;

    parse_rfd(&content, path)
}

/// Parse RFD document from string
pub fn parse_rfd(content: &str, path: &Path) -> Result<RfdDocument, RfdError> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(content);

    // Parse front matter as RfdMetadata
    let metadata: RfdMetadata = if let Some(_data) = result.data {
        // gray_matter stores YAML data - we need to extract it as a string and parse
        // The original content has the YAML in --- delimiters, parse it directly
        let yaml_str = extract_yaml_frontmatter(content)?;
        serde_yaml::from_str(&yaml_str).map_err(|e| RfdError::YamlError {
            message: format!("Failed to parse metadata: {}", e),
        })?
    } else {
        return Err(RfdError::FrontMatterError {
            message: "No front matter found".to_string(),
        });
    };

    // Extract number from filename
    let number = extract_rfd_number(path).ok_or_else(|| RfdError::FileError {
        message: format!("Could not extract RFD number from filename: {}", path.display()),
    })?;

    Ok(RfdDocument::new(
        number,
        metadata,
        result.content.to_string(),
        path.to_path_buf(),
    ))
}

/// Save an RFD document to file
pub fn save_rfd(doc: &RfdDocument, config: &RfdConfig) -> Result<(), RfdError> {
    // Ensure RFD directory exists
    let rfd_dir = config.rfd_directory();
    if !rfd_dir.exists() {
        fs::create_dir_all(&rfd_dir).map_err(|e| file_error(format!(
            "Failed to create directory '{}': {}",
            rfd_dir.display(),
            e
        )))?;
    }

    // Build file path
    let filename = doc.filename();
    let path = rfd_dir.join(&filename);

    // Serialize metadata to YAML
    let metadata_yaml = serde_yaml::to_string(&doc.metadata).map_err(|e| RfdError::YamlError {
        message: format!("Failed to serialize metadata: {}", e),
    })?;

    // Combine front matter and content
    let full_content = format!("---\n{}---\n\n{}", metadata_yaml, doc.content);

    // Write to file (atomic write via temp file)
    write_file_atomic(&path, &full_content)?;

    Ok(())
}

/// Update RFD metadata in file.
///
/// This is a lower-level utility function. Currently, commands use `save_rfd()` instead,
/// but this is kept for potential future use in batch operations or direct metadata updates.
#[allow(dead_code)]
pub fn update_rfd_metadata(path: &Path, metadata: &RfdMetadata) -> Result<(), RfdError> {
    // Load existing document
    let mut doc = load_rfd(path)?;

    // Update metadata
    doc.metadata = metadata.clone();

    // Save back (using the path from the document)
    let config = RfdConfig::load()?;
    save_rfd(&doc, &config)
}

/// Write file atomically using a temporary file
fn write_file_atomic(path: &Path, content: &str) -> Result<(), RfdError> {
    // Write to temp file first
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, content).map_err(|e| file_error(format!(
        "Failed to write temp file '{}': {}",
        temp_path.display(),
        e
    )))?;

    // Rename (atomic on POSIX systems)
    fs::rename(&temp_path, path).map_err(|e| file_error(format!(
        "Failed to rename temp file to '{}': {}",
        path.display(),
        e
    )))?;

    Ok(())
}

/// Extract YAML frontmatter from markdown content
///
/// For junior developers: This parses Jekyll-style frontmatter:
/// ```markdown
/// ---
/// title: My RFD
/// authors: [Alice]
/// ---
/// Content here...
/// ```
///
/// We extract the YAML between the two `---` delimiters.
fn extract_yaml_frontmatter(content: &str) -> Result<String, RfdError> {
    let lines: Vec<&str> = content.lines().collect();

    // Check for opening delimiter
    if lines.is_empty() || lines[0].trim() != "---" {
        return Err(RfdError::FrontMatterError {
            message: "No YAML frontmatter found (must start with ---)".to_string(),
        });
    }

    // Find the closing --- (skip the first line since that's the opening)
    // .position() returns the index in the skipped iterator, so we need to add 1
    let end_index = lines.iter().skip(1).position(|line| line.trim() == "---");

    if let Some(end) = end_index {
        // Extract lines between the two ---
        // Example: ["---", "title: Test", "---"] → lines[1..2] → ["title: Test"]
        let yaml_lines = &lines[1..end + 1];
        Ok(yaml_lines.join("\n"))
    } else {
        Err(RfdError::FrontMatterError {
            message: "Unclosed YAML frontmatter (missing closing ---)".to_string(),
        })
    }
}

/// Check if an RFD file exists for the given number
pub fn rfd_exists(config: &RfdConfig, number: u32) -> Result<bool, RfdError> {
    let files = find_all_rfds(config)?;
    Ok(files.iter().any(|path| extract_rfd_number(path) == Some(number)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rfd_number() {
        assert_eq!(
            extract_rfd_number(Path::new("0001-test.md")),
            Some(1)
        );
        assert_eq!(
            extract_rfd_number(Path::new("0042-feature-proposal.md")),
            Some(42)
        );
        assert_eq!(
            extract_rfd_number(Path::new("test.md")),
            None
        );
    }

    #[test]
    fn test_parse_rfd() {
        let content = r#"---
title: "Test RFD"
authors: ["Alice <alice@example.com>"]
state: draft
created: 2025-10-17T00:00:00Z
updated: 2025-10-17T00:00:00Z
tags: ["test"]
---

# Summary
This is a test
"#;

        let path = Path::new("rfds/0001-test.md");
        let doc = parse_rfd(content, path).unwrap();

        assert_eq!(doc.number, 1);
        assert_eq!(doc.metadata.title, "Test RFD");
        assert_eq!(doc.metadata.authors.len(), 1);
        assert_eq!(doc.content.trim(), "# Summary\nThis is a test");
    }
}
