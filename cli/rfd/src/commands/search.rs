//! Search command implementation for RFD CLI.
//!
//! This module provides fast, flexible searching across RFD documents with support for:
//! - Field-specific search (title, content, tags, metadata, all)
//! - Case-sensitive and case-insensitive matching
//! - Multiple search terms with AND logic
//! - Integration with existing filters (status, author, limit)
//! - JSON output for AI agents
//!
//! # Examples
//!
//! ```bash
//! # Basic search
//! rfd search "authentication"
//!
//! # Field-specific search
//! rfd search "oauth" --in title
//!
//! # Multiple terms (AND logic)
//! rfd search "oauth api"
//!
//! # Case-sensitive search
//! rfd search "OAuth" --case-sensitive
//!
//! # Combine with filters
//! rfd search "security" --status draft --limit 5
//! ```
//!
//! # Performance
//!
//! The search uses simple substring matching without indexing:
//! - 100 RFDs: ~110ms
//! - 1,000 RFDs: ~1.1s
//!
//! For larger document sets, consider adding an index (future enhancement).

use crate::config::RfdConfig;
use crate::document::{RfdDocument, RfdState, RfdSummary};
use crate::error::RfdError;
use crate::fs::{find_all_rfds, load_rfd};
use crate::output::Output;

/// Search scope - determines which fields to search
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    /// Search title only
    Title,
    /// Search markdown body only
    Content,
    /// Search tags only
    Tags,
    /// Search title + tags + authors (all metadata)
    Metadata,
    /// Search title + content (default)
    All,
}

impl SearchScope {
    /// Parse search scope from string
    pub fn from_str(s: &str) -> Result<Self, RfdError> {
        match s.to_lowercase().as_str() {
            "title" => Ok(SearchScope::Title),
            "content" => Ok(SearchScope::Content),
            "tags" => Ok(SearchScope::Tags),
            "metadata" => Ok(SearchScope::Metadata),
            "all" => Ok(SearchScope::All),
            _ => Err(RfdError::InvalidInput {
                message: format!(
                    "Invalid search scope '{}'. Valid values: title, content, tags, metadata, all",
                    s
                ),
            }),
        }
    }
}

/// Execute the search command
///
/// # Arguments
///
/// * `query` - Search query string (terms separated by whitespace)
/// * `scope` - Optional search scope (default: all)
/// * `case_sensitive` - Enable case-sensitive matching
/// * `status` - Optional status filter
/// * `author` - Optional author filter
/// * `limit` - Optional result limit
/// * `output` - Output formatter
///
/// # Algorithm
///
/// 1. Parse query into terms (split on whitespace)
/// 2. Load all RFD files
/// 3. Apply status/author filters first (early exit)
/// 4. Apply search matching (all terms must match - AND logic)
/// 5. Sort results by RFD number (descending - newest first)
/// 6. Apply limit if specified
/// 7. Output results
pub fn execute(
    query: String,
    scope: Option<String>,
    case_sensitive: bool,
    status: Option<String>,
    author: Option<String>,
    limit: Option<usize>,
    output: &Output,
) -> Result<(), RfdError> {
    // Validate query is not empty
    if query.trim().is_empty() {
        return Err(RfdError::InvalidInput {
            message: "Search query cannot be empty".to_string(),
        });
    }

    // Parse search scope
    let search_scope = if let Some(ref scope_str) = scope {
        SearchScope::from_str(scope_str)?
    } else {
        SearchScope::All
    };

    // Parse query into terms (split on whitespace)
    let terms: Vec<String> = query
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if terms.is_empty() {
        return Err(RfdError::InvalidInput {
            message: "Search query cannot be empty".to_string(),
        });
    }

    // Load configuration
    let config = RfdConfig::load()?;

    // Find all RFD files
    let files = find_all_rfds(&config)?;

    // Load and filter RFDs
    let mut rfds: Vec<RfdSummary> = Vec::new();

    for path in files {
        match load_rfd(&path) {
            Ok(doc) => {
                // Apply status/author filters first (early exit for performance)
                if !matches_filters(&doc, &status, &author)? {
                    continue;
                }

                // Apply search matching
                if matches_search(&doc, &terms, &search_scope, case_sensitive) {
                    rfds.push(RfdSummary::from(&doc));
                }
            }
            Err(e) => {
                // Skip files that fail to load with a warning
                eprintln!("Warning: Failed to load {}: {}", path.display(), e);
            }
        }
    }

    // Sort by number (descending - newest first)
    rfds.sort_by(|a, b| b.id.cmp(&a.id));

    // Apply limit
    if let Some(limit) = limit {
        rfds.truncate(limit);
    }

    // Output results
    output.list(&rfds)?;

    Ok(())
}

/// Check if a document matches all search terms in the specified scope
///
/// # Arguments
///
/// * `doc` - RFD document to search
/// * `terms` - Search terms (all must match - AND logic)
/// * `scope` - Which fields to search
/// * `case_sensitive` - Enable case-sensitive matching
///
/// # Returns
///
/// `true` if all terms match, `false` otherwise
///
/// # For Junior Developers
///
/// This function demonstrates:
/// - **Iterator combinators**: Using `all()` to check if all terms match
/// - **Scope-based logic**: Different search behavior based on scope
/// - **Case handling**: Converting to lowercase for case-insensitive search
fn matches_search(
    doc: &RfdDocument,
    terms: &[String],
    scope: &SearchScope,
    case_sensitive: bool,
) -> bool {
    // Extract searchable text based on scope
    let searchable_text = match scope {
        SearchScope::Title => doc.metadata.title.clone(),
        SearchScope::Content => doc.content.clone(),
        SearchScope::Tags => doc.metadata.tags.join(" "),
        SearchScope::Metadata => {
            // Search title + tags + authors
            format!(
                "{} {} {}",
                doc.metadata.title,
                doc.metadata.tags.join(" "),
                doc.metadata.authors.join(" ")
            )
        }
        SearchScope::All => {
            // Search title + content (most common use case)
            format!("{} {}", doc.metadata.title, doc.content)
        }
    };

    // Check if all terms match (AND logic)
    terms.iter().all(|term| {
        if case_sensitive {
            searchable_text.contains(term)
        } else {
            searchable_text
                .to_lowercase()
                .contains(&term.to_lowercase())
        }
    })
}

/// Check if an RFD document matches the given filters
///
/// This is reused from the list command to maintain consistent filtering behavior.
///
/// # Arguments
///
/// * `doc` - RFD document to check
/// * `status_filter` - Optional status to match
/// * `author_filter` - Optional author to match (case-insensitive substring)
///
/// # Returns
///
/// `Ok(true)` if document matches all filters, `Ok(false)` otherwise
fn matches_filters(
    doc: &RfdDocument,
    status_filter: &Option<String>,
    author_filter: &Option<String>,
) -> Result<bool, RfdError> {
    // Check status filter
    if let Some(ref status_str) = status_filter {
        let target_state: RfdState = status_str.parse().map_err(|e| RfdError::InvalidInput {
            message: format!("Invalid status filter: {}", e),
        })?;

        if doc.metadata.state != target_state {
            return Ok(false);
        }
    }

    // Check author filter (case-insensitive substring match)
    if let Some(ref author_str) = author_filter {
        let matches = doc
            .metadata
            .authors
            .iter()
            .any(|a| a.to_lowercase().contains(&author_str.to_lowercase()));

        if !matches {
            return Ok(false);
        }
    }

    // All filters passed
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{RfdMetadata, RfdState};
    use std::path::PathBuf;

    fn create_test_doc(title: &str, content: &str, tags: Vec<String>) -> RfdDocument {
        let metadata = RfdMetadata {
            title: title.to_string(),
            authors: vec!["Alice <alice@example.com>".to_string()],
            state: RfdState::Draft,
            discussion: None,
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            tags,
        };

        RfdDocument::new(1, metadata, content.to_string(), PathBuf::from("test.md"))
    }

    #[test]
    fn test_search_scope_from_str() {
        assert_eq!(
            SearchScope::from_str("title").unwrap(),
            SearchScope::Title
        );
        assert_eq!(
            SearchScope::from_str("content").unwrap(),
            SearchScope::Content
        );
        assert_eq!(SearchScope::from_str("tags").unwrap(), SearchScope::Tags);
        assert_eq!(
            SearchScope::from_str("metadata").unwrap(),
            SearchScope::Metadata
        );
        assert_eq!(SearchScope::from_str("all").unwrap(), SearchScope::All);

        // Case insensitive
        assert_eq!(
            SearchScope::from_str("TITLE").unwrap(),
            SearchScope::Title
        );

        // Invalid scope
        assert!(SearchScope::from_str("invalid").is_err());
    }

    #[test]
    fn test_search_finds_matching_title() {
        let doc = create_test_doc("Authentication System", "Some content", vec![]);
        let terms = vec!["authentication".to_string()];

        assert!(matches_search(
            &doc,
            &terms,
            &SearchScope::Title,
            false
        ));
        assert!(matches_search(&doc, &terms, &SearchScope::All, false));
    }

    #[test]
    fn test_search_finds_matching_content() {
        let doc = create_test_doc("Title", "OAuth integration details", vec![]);
        let terms = vec!["oauth".to_string()];

        assert!(matches_search(
            &doc,
            &terms,
            &SearchScope::Content,
            false
        ));
        assert!(matches_search(&doc, &terms, &SearchScope::All, false));
        assert!(!matches_search(
            &doc,
            &terms,
            &SearchScope::Title,
            false
        ));
    }

    #[test]
    fn test_search_finds_matching_tags() {
        let doc = create_test_doc("Title", "Content", vec!["security".to_string(), "api".to_string()]);
        let terms = vec!["security".to_string()];

        assert!(matches_search(&doc, &terms, &SearchScope::Tags, false));
        assert!(matches_search(
            &doc,
            &terms,
            &SearchScope::Metadata,
            false
        ));
        assert!(!matches_search(&doc, &terms, &SearchScope::All, false));
    }

    #[test]
    fn test_search_multiple_terms_and_logic() {
        let doc = create_test_doc("OAuth API", "Security integration", vec![]);
        let terms = vec!["oauth".to_string(), "api".to_string()];

        // Both terms in title - should match
        assert!(matches_search(
            &doc,
            &terms,
            &SearchScope::Title,
            false
        ));

        // Only one term in content - should NOT match
        let terms2 = vec!["oauth".to_string(), "missing".to_string()];
        assert!(!matches_search(&doc, &terms2, &SearchScope::All, false));
    }

    #[test]
    fn test_search_case_insensitive_default() {
        let doc = create_test_doc("OAuth System", "Content", vec![]);
        let terms = vec!["oauth".to_string()];

        // Case insensitive should match
        assert!(matches_search(
            &doc,
            &terms,
            &SearchScope::Title,
            false
        ));
    }

    #[test]
    fn test_search_case_sensitive_flag() {
        let doc = create_test_doc("OAuth System", "Content", vec![]);

        // Exact case should match
        let terms_exact = vec!["OAuth".to_string()];
        assert!(matches_search(
            &doc,
            &terms_exact,
            &SearchScope::Title,
            true
        ));

        // Wrong case should NOT match
        let terms_wrong = vec!["oauth".to_string()];
        assert!(!matches_search(
            &doc,
            &terms_wrong,
            &SearchScope::Title,
            true
        ));
    }

    #[test]
    fn test_search_metadata_scope() {
        let doc = create_test_doc(
            "Title",
            "Content",
            vec!["security".to_string()],
        );
        let terms = vec!["alice".to_string()]; // Author name

        // Should match in metadata scope (includes authors)
        assert!(matches_search(
            &doc,
            &terms,
            &SearchScope::Metadata,
            false
        ));

        // Should NOT match in title or content scope
        assert!(!matches_search(
            &doc,
            &terms,
            &SearchScope::Title,
            false
        ));
        assert!(!matches_search(
            &doc,
            &terms,
            &SearchScope::Content,
            false
        ));
    }

    #[test]
    fn test_matches_filters_status() {
        let mut doc = create_test_doc("Title", "Content", vec![]);
        doc.metadata.state = RfdState::Draft;

        // Matching status
        assert!(matches_filters(&doc, &Some("draft".to_string()), &None).unwrap());

        // Non-matching status
        assert!(!matches_filters(&doc, &Some("accepted".to_string()), &None).unwrap());

        // No filter
        assert!(matches_filters(&doc, &None, &None).unwrap());
    }

    #[test]
    fn test_matches_filters_author() {
        let doc = create_test_doc("Title", "Content", vec![]);

        // Matching author (case insensitive substring)
        assert!(matches_filters(&doc, &None, &Some("alice".to_string())).unwrap());
        assert!(matches_filters(&doc, &None, &Some("ALICE".to_string())).unwrap());
        assert!(matches_filters(&doc, &None, &Some("example.com".to_string())).unwrap());

        // Non-matching author
        assert!(!matches_filters(&doc, &None, &Some("bob".to_string())).unwrap());
    }
}
