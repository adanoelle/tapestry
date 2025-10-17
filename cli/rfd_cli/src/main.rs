use anyhow::Result;
use clap::{Parser, Subcommand};

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
        #[arg(short = 't', long, default_value = "default")]
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
        #[arg(short, long)]
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            title,
            author,
            template,
        } => {
            println!("Creating RFD: {}", title);
            println!("Author: {}", author);
            println!("Template: {}", template);
            println!("\n⚠️  Not yet implemented");
        }
        Commands::List {
            status,
            author,
            limit,
        } => {
            println!("Listing RFDs");
            if let Some(s) = status {
                println!("Status filter: {}", s);
            }
            if let Some(a) = author {
                println!("Author filter: {}", a);
            }
            if let Some(l) = limit {
                println!("Limit: {}", l);
            }
            println!("\n⚠️  Not yet implemented");
        }
        Commands::Show { id } => {
            println!("Showing RFD: {}", id);
            println!("\n⚠️  Not yet implemented");
        }
        Commands::Status { id, set } => {
            println!("Updating RFD {} status to: {}", id, set);
            println!("\n⚠️  Not yet implemented");
        }
        Commands::Update { id, field, value } => {
            println!("Updating RFD {} field '{}' to: {}", id, field, value);
            println!("\n⚠️  Not yet implemented");
        }
        Commands::Validate { id } => {
            println!("Validating RFD: {}", id);
            println!("\n⚠️  Not yet implemented");
        }
    }

    Ok(())
}
