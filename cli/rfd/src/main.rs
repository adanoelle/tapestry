//! RFD CLI - A tool for managing Request for Discussion (RFD) documents.
//!
//! # Overview
//!
//! This CLI tool helps teams manage technical documentation following the
//! [Oxide Computer RFD format](https://rfd.shared.oxide.computer/). It provides
//! structured commands for creating, updating, and managing RFD documents with
//! an agent-friendly design.
//!
//! # Quick Start
//!
//! ```bash
//! # Create a new RFD
//! rfd create --title "My Proposal" --author "Alice <alice@example.com>"
//!
//! # List all RFDs
//! rfd list
//!
//! # Update RFD status
//! rfd status 1 --set review
//!
//! # Get JSON output for agents
//! rfd list --format json
//! ```
//!
//! # Architecture
//!
//! The CLI is organized into modules:
//! - `commands` - Command implementations (create, list, show, etc.)
//! - `document` - RFD data models and state machine
//! - `config` - Configuration loading from .rfd/config.toml
//! - `template` - Jinja2 template rendering
//! - `fs` - File I/O with YAML frontmatter parsing
//! - `output` - Formatting for pretty/JSON/quiet output
//! - `error` - Error types with actionable suggestions
//!
//! # For Junior Developers
//!
//! This codebase demonstrates several important patterns:
//! - **State Machines**: See `document::RfdState` for document lifecycle
//! - **Error Handling**: Errors include suggestions for how to fix them
//! - **Agent-Friendly Design**: JSON output, idempotent operations
//! - **Template Rendering**: Dynamic document generation with Jinja2
//!
//! Start by reading `commands/create.rs` to see how a command is implemented
//! end-to-end, then explore the other modules.
//!
//! # Design Principles
//!
//! 1. **Agent-First**: JSON output mode, structured errors, no prompts
//! 2. **Idempotent**: Commands can be safely retried
//! 3. **Fast**: < 10ms startup time for CLI invocations
//! 4. **Simple**: Single binary, no dependencies to install
//!
//! # Exit Codes
//!
//! - `0` - Success
//! - `1` - General error (file not found, I/O error, etc.)
//! - `2` - Validation error (invalid RFD structure)
//! - `3` - State transition error (invalid state change)

use clap::{Parser, Subcommand};
use std::process;

mod commands;
mod config;
mod document;
mod error;
mod fs;
mod output;
mod template;

use output::{Output, OutputFormat};

/// CLI tool for managing RFD (Request for Discussion) documents
#[derive(Parser, Debug)]
#[command(name = "rfd")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format (pretty, json, quiet)
    #[arg(short, long, default_value = "pretty", global = true)]
    format: String,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new RFD document
    Create {
        /// Title of the RFD
        #[arg(short, long)]
        title: String,

        /// Author name and email (e.g., "Name <email@example.com>")
        #[arg(short, long)]
        author: String,

        /// Template to use (default: "default")
        #[arg(long, default_value = "default")]
        template: String,
    },

    /// List all RFDs
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,

        /// Filter by author
        #[arg(short, long)]
        author: Option<String>,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Show details of an RFD
    Show {
        /// RFD ID (e.g., "001" or "1")
        id: String,
    },

    /// Update RFD status
    Status {
        /// RFD ID
        id: String,

        /// New status
        #[arg(short, long)]
        set: String,
    },

    /// Update RFD metadata
    Update {
        /// RFD ID
        id: String,

        /// Field to update
        #[arg(long)]
        field: String,

        /// New value
        #[arg(short, long)]
        value: String,
    },

    /// Validate an RFD
    Validate {
        /// RFD ID
        id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Parse output format
    let format = match cli.format.parse::<OutputFormat>() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let output = Output::new(format);

    // Execute command
    let result = match cli.command {
        Commands::Create {
            title,
            author,
            template,
        } => commands::create::execute(title, author, template, &output),

        Commands::List {
            status,
            author,
            limit,
        } => commands::list::execute(status, author, limit, &output),

        Commands::Show { id } => commands::show::execute(id, &output),

        Commands::Status { id, set } => commands::status::execute(id, set, &output),

        Commands::Update { id, field, value } => {
            commands::update::execute(id, field, value, &output)
        }

        Commands::Validate { id } => commands::validate::execute(id, &output),
    };

    // Handle errors
    if let Err(e) = result {
        output.error(&e);
        process::exit(e.exit_code());
    }
}
