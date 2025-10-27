//! Adapter implementations (hexagonal architecture).
//!
//! Adapters implement port interfaces to connect to real systems.
//!
//! # Available Adapters
//!
//! - `github_api` - Production adapter using GitHub REST API
//! - `github_mock` - Testing adapter with in-memory storage

pub mod github_api;
pub mod github_mock;
