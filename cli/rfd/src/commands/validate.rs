use crate::config::RfdConfig;
use crate::error::RfdError;
use crate::fs::{find_rfd_by_id, load_rfd};
use crate::output::Output;

pub fn execute(id: String, output: &Output) -> Result<(), RfdError> {
    let config = RfdConfig::load()?;

    // Find and load RFD
    let path = find_rfd_by_id(&config, &id)?;
    let doc = load_rfd(&path)?;

    // Validate
    let issues = match doc.validate() {
        Ok(()) => vec![],
        Err(issues) => issues,
    };

    // Output
    output.validation(&doc.formatted_number(), &issues)?;

    // Return error exit code if validation failed
    if !issues.is_empty() {
        return Err(RfdError::ValidationFailed { issues });
    }

    Ok(())
}
