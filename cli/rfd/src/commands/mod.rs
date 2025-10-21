//! Command implementations for the RFD CLI.
//!
//! This module contains all command handlers, organized by command name.
//! Each command is responsible for:
//! 1. Validating input parameters
//! 2. Loading configuration
//! 3. Performing the operation
//! 4. Formatting and displaying results
//!
//! # Available Commands
//!
//! - [`create`] - Create a new RFD from a template
//! - [`list`] - List RFDs with optional filtering
//! - [`search`] - Search RFDs by content
//! - [`show`] - Display a specific RFD's details
//! - [`status`] - Update an RFD's state
//! - [`update`] - Modify RFD metadata fields
//! - [`validate`] - Check if an RFD is well-formed
//!
//! # Command Pattern
//!
//! All commands follow a consistent pattern:
//!
//! ```rust,ignore
//! pub fn execute(
//!     /* command-specific args */,
//!     output: &Output,
//! ) -> Result<(), RfdError> {
//!     // 1. Load configuration
//!     let config = RfdConfig::load()?;
//!
//!     // 2. Perform operation
//!     let result = do_something(&config)?;
//!
//!     // 3. Display results
//!     output.show_result(&result)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Idempotency
//!
//! Commands are designed to be idempotent where possible:
//! - **create**: Returns error if RFD already exists (not idempotent by design)
//! - **status**: Succeeds even if already in target state (idempotent)
//! - **update**: Always succeeds if values are valid (idempotent)
//! - **list/show/validate**: Read-only, always idempotent
//!
//! # Error Handling
//!
//! Commands use the `?` operator to propagate errors up to `main()`,
//! which handles formatting and exit codes. This keeps command code clean
//! and focused on business logic.
//!
//! # For Junior Developers
//!
//! This module demonstrates:
//! - **Separation of Concerns**: Commands orchestrate, they don't implement
//! - **Dependency Injection**: Output formatter is passed in
//! - **Error Propagation**: Using `Result<>` and `?` for clean error handling
//! - **Consistent Patterns**: All commands have similar structure
//!
//! When adding a new command, follow the pattern in [`create`] - it's the
//! most complete example of validation, operation, and output.

pub mod create;
pub mod list;
pub mod search;
pub mod show;
pub mod status;
pub mod update;
pub mod validate;
