use crate::config::RfdConfig;
use crate::document::RfdState;
use crate::error::RfdError;
use crate::fs::{find_rfd_by_id, load_rfd, save_rfd};
use crate::output::Output;

pub fn execute(id: String, set: String, output: &Output) -> Result<(), RfdError> {
    let config = RfdConfig::load()?;

    // Parse target state
    let target_state: RfdState = set.parse().map_err(|e| RfdError::InvalidInput {
        message: e,
    })?;

    // Find and load RFD
    let path = find_rfd_by_id(&config, &id)?;
    let mut doc = load_rfd(&path)?;

    let old_state = doc.metadata.state;

    // Check if transition is valid
    if !old_state.can_transition_to(&target_state) {
        return Err(RfdError::InvalidTransition {
            current: old_state,
            target: target_state,
        });
    }

    // Update state (idempotent - ok if already in target state)
    doc.metadata.state = target_state;

    // Update timestamp if state changed
    if old_state != target_state {
        doc.metadata.touch();
    }

    // Save
    save_rfd(&doc, &config)?;

    // Output
    output.status_updated(&doc, old_state)?;

    Ok(())
}
