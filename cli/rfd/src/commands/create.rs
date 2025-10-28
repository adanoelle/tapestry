use crate::config::RfdConfig;
use crate::document::{RfdDocument, RfdMetadata};
use crate::error::RfdError;
use crate::fs::{next_rfd_number, rfd_exists, save_rfd};
use crate::output::Output;
use crate::template::TemplateEngine;

pub fn execute(
    title: String,
    author: Option<String>,
    template: String,
    output: &Output,
) -> Result<(), RfdError> {
    let config = RfdConfig::load()?;

    // Determine author: use provided value or fall back to config default
    let author = match author {
        Some(a) => a,
        None => config
            .rfd
            .default_author
            .clone()
            .ok_or_else(|| RfdError::InvalidInput {
                message: "No author provided and no default_author configured.\n\
                     Either provide --author flag or set default_author in .rfd/config.toml"
                    .to_string(),
            })?,
    };

    // Get next RFD number
    let number = next_rfd_number(&config)?;

    // Check if RFD already exists (idempotency check)
    if rfd_exists(&config, number)? {
        return Err(RfdError::FileError {
            message: format!("RFD {} already exists", config.format_number(number)?),
        });
    }

    // Create metadata
    let metadata = RfdMetadata::new(title.clone(), vec![author]);

    // Validate metadata
    metadata
        .validate()
        .map_err(|issues| RfdError::ValidationFailed { issues })?;

    // Render template
    let template_engine = TemplateEngine::new(&config)?;
    let content = template_engine.render(&template, number, &metadata)?;

    // Create document
    let filename = format!(
        "{}-{}.md",
        config.format_number(number)?,
        slug_from_title(&title)
    );
    let path = config.rfd_directory().join(&filename);

    let doc = RfdDocument::new(number, metadata, content, path);

    // Save to file
    save_rfd(&doc, &config)?;

    // Output result
    output.created(&doc)?;

    Ok(())
}

/// Create a slug from title (lowercase, hyphenated)
///
/// For junior developers: Converts "My Great Idea!" → "my-great-idea"
/// This is the same logic as RfdDocument::slug() but used during creation
/// before we have a full document.
fn slug_from_title(title: &str) -> String {
    title
        .to_lowercase()
        // Replace non-alphanumeric with hyphens
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        // Split and filter to remove consecutive hyphens
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slug_from_title() {
        assert_eq!(slug_from_title("Feature Proposal"), "feature-proposal");
        assert_eq!(
            slug_from_title("RFD: Authentication System"),
            "rfd-authentication-system"
        );
        assert_eq!(
            slug_from_title("Multi-Region   Support!!!"),
            "multi-region-support"
        );
    }
}
