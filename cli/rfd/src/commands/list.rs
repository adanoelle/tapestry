use crate::config::RfdConfig;
use crate::document::{RfdDocument, RfdState, RfdSummary};
use crate::error::RfdError;
use crate::fs::{find_all_rfds, load_rfd};
use crate::output::Output;

pub fn execute(
    status: Option<String>,
    author: Option<String>,
    limit: Option<usize>,
    verbose: bool,
    output: &Output,
) -> Result<(), RfdError> {
    let config = RfdConfig::load()?;

    // Find all RFD files
    let files = find_all_rfds(&config)?;

    // Load and filter RFDs
    let mut rfds: Vec<RfdSummary> = Vec::new();

    for path in files {
        match load_rfd(&path) {
            Ok(doc) => {
                // Apply filters - skip RFDs that don't match
                if !matches_filters(&doc, &status, &author)? {
                    continue;
                }

                rfds.push(RfdSummary::from(&doc));
            }
            Err(e) => {
                // Skip files that fail to load - only log in verbose mode
                if verbose {
                    eprintln!("Warning: Failed to load {}: {}", path.display(), e);
                }
            }
        }
    }

    // Sort by number (descending - newest first)
    rfds.sort_by(|a, b| b.id.cmp(&a.id));

    // Apply limit
    if let Some(limit) = limit {
        rfds.truncate(limit);
    }

    // Output
    output.list(&rfds)?;

    Ok(())
}

/// Check if an RFD document matches the given filters
///
/// For junior developers: This function encapsulates filter logic to keep
/// the main execute function readable. It returns true if the document should
/// be included in the results.
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
