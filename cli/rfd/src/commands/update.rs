use crate::config::RfdConfig;
use crate::error::{invalid_input, RfdError};
use crate::fs::{find_rfd_by_id, load_rfd, save_rfd};
use crate::output::Output;

pub fn execute(
    id: String,
    field: String,
    value: String,
    output: &Output,
) -> Result<(), RfdError> {
    let config = RfdConfig::load()?;

    // Find and load RFD
    let path = find_rfd_by_id(&config, &id)?;
    let mut doc = load_rfd(&path)?;

    // Update field
    match field.to_lowercase().as_str() {
        "title" => {
            doc.metadata.title = value;
        }
        "discussion" => {
            doc.metadata.discussion = if value.is_empty() {
                None
            } else {
                Some(value)
            };
        }
        "tags" => {
            // Parse comma-separated tags
            doc.metadata.tags = value
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
        _ => {
            return Err(invalid_input(format!(
                "Unknown field '{}'. Supported fields: title, discussion, tags",
                field
            )));
        }
    }

    // Update timestamp
    doc.metadata.touch();

    // Validate
    doc.metadata.validate().map_err(|issues| RfdError::ValidationFailed { issues })?;

    // Save
    save_rfd(&doc, &config)?;

    // Output
    output.success(&format!(
        "Updated RFD {} field '{}'",
        doc.formatted_number(),
        field
    ));

    Ok(())
}
