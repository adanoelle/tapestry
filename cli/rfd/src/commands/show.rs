use crate::config::RfdConfig;
use crate::error::RfdError;
use crate::fs::{find_rfd_by_id, load_rfd};
use crate::output::Output;

pub fn execute(id: String, output: &Output) -> Result<(), RfdError> {
    let config = RfdConfig::load()?;

    // Find RFD by ID
    let path = find_rfd_by_id(&config, &id)?;

    // Load the RFD
    let doc = load_rfd(&path)?;

    // Output
    output.show(&doc)?;

    Ok(())
}
