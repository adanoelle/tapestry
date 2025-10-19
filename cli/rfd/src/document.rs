//! RFD document models and state machine.
//!
//! This module contains the core domain types for representing RFD documents,
//! including metadata, state transitions, and validation logic.
//!
//! # Type Aliases
//!
//! For junior developers: Type aliases help make code self-documenting by
//! giving meaningful names to primitive types.
//!
//! - [`RfdNumber`] - The numeric ID of an RFD (1, 2, 3, etc.)
//! - [`RfdId`] - Formatted ID string (e.g., "0001", "0042")
//!
//! # Key Types
//!
//! - [`RfdState`] - Document lifecycle states (draft → review → accepted → implemented)
//! - [`RfdMetadata`] - YAML front matter (title, authors, dates, tags, etc.)
//! - [`RfdDocument`] - Complete document (metadata + markdown content)
//! - [`RfdSummary`] - Lightweight representation for list display
//!
//! # State Machine
//!
//! RFDs follow a well-defined lifecycle with validated state transitions:
//!
//! ```text
//! draft ──> review ──> accepted ──> implemented
//!   │         │           │
//!   └─────────┴───────────┴────> rejected ──> archived
//! ```
//!
//! The state machine enforces business rules:
//! - You can't skip from `draft` directly to `implemented`
//! - `archived` is a terminal state (no transitions out)
//! - All transitions are idempotent (setting the same state twice succeeds)
//!
//! # Examples
//!
//! ```rust
//! use rfd::document::{RfdMetadata, RfdState};
//!
//! // Create new RFD metadata
//! let metadata = RfdMetadata::new(
//!     "Authentication System".to_string(),
//!     vec!["Alice <alice@example.com>".to_string()],
//! );
//!
//! assert_eq!(metadata.state, RfdState::Draft);
//!
//! // Check valid transitions
//! assert!(RfdState::Draft.can_transition_to(&RfdState::Review));
//! assert!(!RfdState::Draft.can_transition_to(&RfdState::Archived));
//! ```
//!
//! # For Junior Developers
//!
//! This module demonstrates:
//! - **State Machines**: Using enums and match to enforce rules
//! - **Domain Modeling**: Separating data (RfdMetadata) from logic (validation)
//! - **Serialization**: Using serde for JSON/YAML conversion
//! - **Builder Patterns**: RfdMetadata::new() with sensible defaults
//!
//! Start by reading [`RfdState::can_transition_to()`] to see how state
//! transitions are validated using Rust's pattern matching.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

// Type aliases for domain concepts
//
// For junior developers: These make code more self-documenting. When you see
// `RfdNumber`, you immediately know it's an RFD ID, not just any u32.

/// The numeric ID of an RFD (e.g., 1, 42, 123)
///
/// RFD numbers are sequential and start at 1. They never change once assigned.
pub type RfdNumber = u32;

/// Formatted RFD ID string (e.g., "0001", "0042", "0123")
///
/// This is the zero-padded representation used in filenames and display.
/// The padding width is configurable via RfdConfig.
pub type RfdId = String;

/// RFD state representing the document lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RfdState {
    Draft,
    Review,
    Accepted,
    Rejected,
    Implemented,
    Archived,
}

impl RfdState {
    /// Check if a state transition is valid
    pub fn can_transition_to(&self, target: &RfdState) -> bool {
        use RfdState::*;

        match (self, target) {
            // Same state is always valid (idempotent)
            (a, b) if a == b => true,

            // Draft can go to review, accepted, or rejected
            (Draft, Review) | (Draft, Accepted) | (Draft, Rejected) => true,

            // Review can go to accepted, rejected, or back to draft
            (Review, Accepted) | (Review, Rejected) | (Review, Draft) => true,

            // Accepted can go to implemented or rejected (circumstances change)
            (Accepted, Implemented) | (Accepted, Rejected) => true,

            // Rejected can only go to archived
            (Rejected, Archived) => true,

            // Implemented can only go to archived
            (Implemented, Archived) => true,

            // Archived is terminal
            (Archived, _) => false,

            // All other transitions are invalid
            _ => false,
        }
    }

    /// Get valid next states for this state
    pub fn valid_next_states(&self) -> Vec<RfdState> {
        use RfdState::*;

        match self {
            Draft => vec![Review, Accepted, Rejected],
            Review => vec![Draft, Accepted, Rejected],
            Accepted => vec![Implemented, Rejected],
            Rejected => vec![Archived],
            Implemented => vec![Archived],
            Archived => vec![],
        }
    }

    /// Check if this is a terminal state.
    ///
    /// Terminal states (like Archived) cannot transition to any other state.
    /// Kept for future validation logic and documentation purposes.
    #[allow(dead_code)]
    pub fn is_terminal(&self) -> bool {
        matches!(self, RfdState::Archived)
    }
}

impl fmt::Display for RfdState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RfdState::Draft => "draft",
            RfdState::Review => "review",
            RfdState::Accepted => "accepted",
            RfdState::Rejected => "rejected",
            RfdState::Implemented => "implemented",
            RfdState::Archived => "archived",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for RfdState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(RfdState::Draft),
            "review" => Ok(RfdState::Review),
            "accepted" => Ok(RfdState::Accepted),
            "rejected" => Ok(RfdState::Rejected),
            "implemented" => Ok(RfdState::Implemented),
            "archived" => Ok(RfdState::Archived),
            _ => Err(format!("Invalid state: '{}'. Valid states are: draft, review, accepted, rejected, implemented, archived", s)),
        }
    }
}

/// RFD metadata (YAML front matter)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfdMetadata {
    /// Document title
    pub title: String,

    /// Authors in format "Name <email@example.com>"
    pub authors: Vec<String>,

    /// Current state
    pub state: RfdState,

    /// Discussion link (GitHub issue, PR, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion: Option<String>,

    /// Creation date
    pub created: DateTime<Utc>,

    /// Last update date
    pub updated: DateTime<Utc>,

    /// Tags for categorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl RfdMetadata {
    /// Create new RFD metadata with defaults
    pub fn new(title: String, authors: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            title,
            authors,
            state: RfdState::Draft,
            discussion: None,
            created: now,
            updated: now,
            tags: Vec::new(),
        }
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated = Utc::now();
    }

    /// Validate required fields
    ///
    /// For junior developers: This function collects all validation errors
    /// instead of failing on the first one. This provides better UX - users
    /// can fix all issues at once rather than playing "whack-a-mole".
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut issues = Vec::new();

        // Check for empty title (after trimming whitespace)
        if self.title.trim().is_empty() {
            issues.push("Title is empty".to_string());
        }

        // RFDs must have at least one author
        if self.authors.is_empty() {
            issues.push("No authors specified".to_string());
        }

        // Validate author format: "Name <email@example.com>"
        // This format is standard in Git commits and makes attribution clear
        for author in &self.authors {
            if !author.contains('<') || !author.contains('>') {
                issues.push(format!(
                    "Author '{}' should be in format 'Name <email@example.com>'",
                    author
                ));
            }
        }

        // Return all issues at once (better UX than failing fast)
        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }
}

/// Complete RFD document (metadata + content)
#[derive(Debug, Clone)]
pub struct RfdDocument {
    /// RFD number (e.g., 1, 2, 3)
    pub number: RfdNumber,

    /// Metadata from YAML front matter
    pub metadata: RfdMetadata,

    /// Markdown content (body after front matter)
    pub content: String,

    /// File path
    pub path: std::path::PathBuf,
}

impl RfdDocument {
    /// Create new RFD document
    pub fn new(
        number: RfdNumber,
        metadata: RfdMetadata,
        content: String,
        path: std::path::PathBuf,
    ) -> Self {
        Self {
            number,
            metadata,
            content,
            path,
        }
    }

    /// Get formatted RFD number (e.g., "0001")
    pub fn formatted_number(&self) -> RfdId {
        format!("{:04}", self.number)
    }

    /// Get slug from title (lowercase, hyphenated)
    ///
    /// For junior developers: This converts a title like "My Great Idea!"
    /// into a slug like "my-great-idea" suitable for filenames and URLs.
    pub fn slug(&self) -> String {
        self.metadata
            .title
            .to_lowercase()
            // Step 1: Convert non-alphanumeric chars to hyphens
            // "My Great Idea!" → "my-great-idea-"
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            // Step 2: Split on hyphens to get words
            // "my-great-idea-" → ["my", "great", "idea", ""]
            .split('-')
            // Step 3: Remove empty segments (from trailing punctuation)
            // ["my", "great", "idea", ""] → ["my", "great", "idea"]
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            // Step 4: Join back with single hyphens
            // ["my", "great", "idea"] → "my-great-idea"
            .join("-")
    }

    /// Get expected filename (e.g., "0001-feature-proposal.md")
    pub fn filename(&self) -> String {
        format!("{}-{}.md", self.formatted_number(), self.slug())
    }

    /// Validate the entire document
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut issues = Vec::new();

        // Validate metadata
        if let Err(metadata_issues) = self.metadata.validate() {
            issues.extend(metadata_issues);
        }

        // Check if content has required sections
        let required_sections = ["Summary", "Motivation"];
        for section in required_sections {
            if !self.content.contains(&format!("# {}", section))
                && !self.content.contains(&format!("## {}", section))
            {
                issues.push(format!("Missing required section: {}", section));
            }
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }
}

/// Summary information for list display
#[derive(Debug, Clone, Serialize)]
pub struct RfdSummary {
    pub id: RfdId,
    pub title: String,
    pub state: RfdState,
    pub authors: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl From<&RfdDocument> for RfdSummary {
    fn from(doc: &RfdDocument) -> Self {
        Self {
            id: doc.formatted_number(),
            title: doc.metadata.title.clone(),
            state: doc.metadata.state,
            authors: doc.metadata.authors.clone(),
            created: doc.metadata.created,
            updated: doc.metadata.updated,
            path: doc.path.to_string_lossy().to_string(),
            discussion: doc.metadata.discussion.clone(),
            tags: doc.metadata.tags.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        use RfdState::*;

        // Valid transitions
        assert!(Draft.can_transition_to(&Review));
        assert!(Draft.can_transition_to(&Accepted));
        assert!(Draft.can_transition_to(&Rejected));
        assert!(Review.can_transition_to(&Accepted));
        assert!(Review.can_transition_to(&Rejected));
        assert!(Review.can_transition_to(&Draft));
        assert!(Accepted.can_transition_to(&Implemented));
        assert!(Accepted.can_transition_to(&Rejected));
        assert!(Rejected.can_transition_to(&Archived));
        assert!(Implemented.can_transition_to(&Archived));

        // Idempotent (same state)
        assert!(Draft.can_transition_to(&Draft));
        assert!(Accepted.can_transition_to(&Accepted));

        // Invalid transitions
        assert!(!Draft.can_transition_to(&Implemented));
        assert!(!Draft.can_transition_to(&Archived));
        assert!(!Review.can_transition_to(&Implemented));
        assert!(!Archived.can_transition_to(&Draft));
        assert!(!Archived.can_transition_to(&Accepted));
    }

    #[test]
    fn test_terminal_states() {
        assert!(!RfdState::Draft.is_terminal());
        assert!(!RfdState::Accepted.is_terminal());
        assert!(RfdState::Archived.is_terminal());
    }

    #[test]
    fn test_metadata_validation() {
        let mut meta = RfdMetadata::new(
            "Test RFD".to_string(),
            vec!["Alice <alice@example.com>".to_string()],
        );
        assert!(meta.validate().is_ok());

        // Empty title
        meta.title = "".to_string();
        assert!(meta.validate().is_err());

        // Invalid author format
        meta.title = "Test".to_string();
        meta.authors = vec!["Alice".to_string()];
        assert!(meta.validate().is_err());
    }

    #[test]
    fn test_slug_generation() {
        let meta = RfdMetadata::new(
            "Feature Proposal: Authentication".to_string(),
            vec!["Alice <alice@example.com>".to_string()],
        );
        let doc = RfdDocument::new(
            3,
            meta,
            "Content".to_string(),
            std::path::PathBuf::from("rfds/0003-feature-proposal-authentication.md"),
        );

        assert_eq!(doc.slug(), "feature-proposal-authentication");
        assert_eq!(doc.formatted_number(), "0003");
        assert_eq!(doc.filename(), "0003-feature-proposal-authentication.md");
    }
}
