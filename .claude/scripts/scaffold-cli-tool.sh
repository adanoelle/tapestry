#!/bin/bash
# Scaffold CLI Tool - Automated scaffolding for fast, agent-friendly CLI tools
# This script is invoked by the create-cli-tool command

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ️${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Must be run from Tapestry project root (where Cargo.toml exists)"
    exit 1
fi

# Parse arguments
TOOL_NAME=$1
DESCRIPTION=$2

if [ -z "$TOOL_NAME" ] || [ -z "$DESCRIPTION" ]; then
    echo "Usage: $0 <tool-name> <description>"
    echo "Example: $0 rfd \"Manage RFD documents with structured operations\""
    exit 1
fi

# Validate tool name format (kebab-case)
if ! echo "$TOOL_NAME" | grep -qE '^[a-z]+(-[a-z]+)*$'; then
    print_error "Tool name must be kebab-case (e.g., rfd, code-analyzer)"
    exit 1
fi

# Convert kebab-case to snake_case for Rust module
TOOL_MODULE=$(echo "$TOOL_NAME" | tr '-' '_')

print_info "Creating CLI tool: $TOOL_NAME"
print_info "Module name: $TOOL_MODULE"
print_info "Description: $DESCRIPTION"

# Phase 1: Create directory structure
print_status "Phase 1: Creating directory structure..."

TOOL_DIR="cli/$TOOL_MODULE"
mkdir -p $TOOL_DIR/src/commands
mkdir -p $TOOL_DIR/tests

print_status "Created directory structure"

# Phase 2: Create Cargo.toml
print_status "Phase 2: Creating Cargo.toml with size optimizations..."

cat > $TOOL_DIR/Cargo.toml << EOF
[package]
name = "$TOOL_MODULE"
version = "0.1.0"
edition = "2021"
authors = ["Tapestry Team"]
description = "$DESCRIPTION"
license = "MIT"

[[bin]]
name = "$TOOL_NAME"
path = "src/main.rs"

[dependencies]
# CLI framework
clap = { version = "4", features = ["derive", "cargo"] }

# Serialization for JSON output
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling
anyhow = "1"
thiserror = "1"

# Pretty output
colored = "2"

[dev-dependencies]
# CLI testing
assert_cmd = "2"
predicates = "3"
tempfile = "3"

[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
strip = true           # Strip symbols
panic = "abort"        # Smaller binary
EOF

print_status "Generated Cargo.toml"

# Phase 3: Create main.rs
print_status "Phase 3: Creating main.rs with clap structure..."

cat > $TOOL_DIR/src/main.rs << 'EOFMAIN'
use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod output;
mod error;

/// CLI_DESCRIPTION
#[derive(Parser, Debug)]
#[command(name = "CLI_NAME")]
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
    /// Example command 1
    Example {
        /// Example argument
        #[arg(short, long)]
        value: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Example { value } => {
            commands::example::execute(value, &cli.format, cli.verbose)?;
        }
    }

    Ok(())
}
EOFMAIN

# Replace placeholders
sed -i "s/CLI_DESCRIPTION/$DESCRIPTION/g" $TOOL_DIR/src/main.rs
sed -i "s/CLI_NAME/$TOOL_NAME/g" $TOOL_DIR/src/main.rs

print_status "Generated main.rs"

# Phase 4: Create output.rs
print_status "Phase 4: Creating output.rs with three output modes..."

cat > $TOOL_DIR/src/output.rs << 'EOF'
use colored::*;
use serde::Serialize;
use serde_json;

pub enum OutputFormat {
    Pretty,
    Json,
    Quiet,
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "quiet" => OutputFormat::Quiet,
            _ => OutputFormat::Pretty,
        }
    }
}

/// Print success message in the specified format
pub fn print_success<T: Serialize>(format: &str, message: &str, data: Option<T>) {
    let fmt = OutputFormat::from(format);

    match fmt {
        OutputFormat::Pretty => {
            println!("{} {}", "✅".green(), message);
            if let Some(d) = data {
                if let Ok(json) = serde_json::to_string_pretty(&d) {
                    println!("{}", json);
                }
            }
        }
        OutputFormat::Json => {
            let response = serde_json::json!({
                "status": "success",
                "message": message,
                "data": data,
            });
            println!("{}", serde_json::to_string(&response).unwrap());
        }
        OutputFormat::Quiet => {
            // No output on success in quiet mode
        }
    }
}

/// Print error message in the specified format
pub fn print_error(format: &str, error_code: &str, message: &str, suggestion: Option<(&str, &str)>) {
    let fmt = OutputFormat::from(format);

    match fmt {
        OutputFormat::Pretty => {
            eprintln!("{} Error: {}", "❌".red(), message);
            if let Some((cmd, desc)) = suggestion {
                eprintln!("{} Try: {}", "💡".yellow(), cmd);
                eprintln!("   {}", desc);
            }
        }
        OutputFormat::Json => {
            let mut response = serde_json::json!({
                "error": error_code,
                "message": message,
            });

            if let Some((cmd, desc)) = suggestion {
                response["suggestion"] = serde_json::json!({
                    "command": cmd,
                    "description": desc,
                });
            }

            eprintln!("{}", serde_json::to_string(&response).unwrap());
        }
        OutputFormat::Quiet => {
            eprintln!("Error: {}", message);
        }
    }
}
EOF

print_status "Generated output.rs"

# Phase 5: Create error.rs
print_status "Phase 5: Creating error.rs..."

cat > $TOOL_DIR/src/error.rs << 'EOF'
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    // TODO: Add tool-specific errors
}

impl CliError {
    pub fn code(&self) -> &str {
        match self {
            CliError::InvalidInput(_) => "INVALID_INPUT",
            CliError::NotFound(_) => "NOT_FOUND",
            CliError::OperationFailed(_) => "OPERATION_FAILED",
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::InvalidInput(_) => 1,
            CliError::NotFound(_) => 2,
            CliError::OperationFailed(_) => 3,
        }
    }
}
EOF

print_status "Generated error.rs"

# Phase 6: Create commands module
print_status "Phase 6: Creating commands module..."

cat > $TOOL_DIR/src/commands/mod.rs << 'EOF'
pub mod example;
EOF

cat > $TOOL_DIR/src/commands/example.rs << 'EOF'
use anyhow::Result;
use crate::output;

pub fn execute(value: Option<String>, format: &str, _verbose: bool) -> Result<()> {
    // TODO: Implement command logic

    output::print_success(
        format,
        "Example command executed successfully",
        Some(serde_json::json!({
            "value": value.unwrap_or_else(|| "default".to_string()),
        }))
    );

    Ok(())
}
EOF

print_status "Generated command modules"

# Phase 7: Create tests
print_status "Phase 7: Creating integration tests..."

cat > $TOOL_DIR/tests/cli_tests.rs << 'EOFTEST'
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--version")
        .assert()
        .success();
}

#[test]
fn test_example_pretty() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("example")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅"));
}

#[test]
fn test_example_json() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("example")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::is_json());
}

#[test]
fn test_example_quiet() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("example")
        .arg("--format")
        .arg("quiet")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}
EOFTEST

print_status "Generated integration tests"

# Phase 8: Create README
print_status "Phase 8: Creating README.md..."

cat > $TOOL_DIR/README.md << EOF
# $TOOL_NAME

$DESCRIPTION

## Installation

\`\`\`bash
# From source
cargo install --path cli/$TOOL_MODULE

# From repository root
cargo build --release --bin $TOOL_NAME
\`\`\`

## Usage

\`\`\`bash
# Show help
$TOOL_NAME --help

# Example command
$TOOL_NAME example --value test

# JSON output (agent-friendly)
$TOOL_NAME example --format json

# Quiet mode (errors only)
$TOOL_NAME example --format quiet
\`\`\`

## Output Formats

### Pretty (Human-Readable)

\`\`\`bash
$ $TOOL_NAME example
✅ Example command executed successfully
{
  "value": "test"
}
\`\`\`

### JSON (Agent-Friendly)

\`\`\`bash
$ $TOOL_NAME example --format json
{"status":"success","message":"Example command executed successfully","data":{"value":"test"}}
\`\`\`

### Quiet (Errors Only)

\`\`\`bash
$ $TOOL_NAME example --format quiet
# No output on success
\`\`\`

## Error Handling

Errors return non-zero exit codes and structured messages:

\`\`\`bash
$ $TOOL_NAME invalid
❌ Error: Unknown command
💡 Try: $TOOL_NAME --help
   See available commands
\`\`\`

## Performance

- **Startup time**: < 10ms (optimized for agent invocation)
- **Binary size**: < 3MB (stripped)
- **Memory usage**: < 10MB

## Development

\`\`\`bash
# Run tests
cargo test -p $TOOL_MODULE

# Build release
cargo build --release --bin $TOOL_NAME

# Check binary size
ls -lh target/release/$TOOL_NAME

# Measure startup time
time ./target/release/$TOOL_NAME --version
\`\`\`

## Architecture

Simple, flat structure optimized for speed:

- \`main.rs\`: CLI entry point with clap
- \`commands/\`: Command implementations
- \`output.rs\`: Output formatting (pretty/json/quiet)
- \`error.rs\`: Error types and codes

No hexagonal architecture needed - CLI tools should be simple!

## Next Steps

1. Replace the example command with your actual commands
2. Implement command logic in \`src/commands/\`
3. Add tests for all commands
4. Update this README with actual examples
5. Test startup time and binary size

## Contributing

See the RFC in \`docs/design/features/RFC-XXX-$TOOL_NAME.md\`
EOF

print_status "Generated README.md"

# Phase 9: Validation
print_status "Phase 9: Running validation..."

# Check if it compiles
if cargo check -p $TOOL_MODULE 2>/dev/null; then
    print_status "Code compiles successfully"
else
    print_warning "Code has compilation issues (this is expected, complete the implementation)"
fi

# Run cargo fmt
cargo fmt --package $TOOL_MODULE 2>/dev/null || true

# Final summary
echo ""
echo "======================================="
echo -e "${GREEN}✅ CLI tool '$TOOL_NAME' scaffolded successfully!${NC}"
echo "======================================="
echo ""
echo "📁 Structure created:"
echo "  - Binary: $TOOL_NAME"
echo "  - Location: cli/$TOOL_MODULE/"
echo "  - Main: cli/$TOOL_MODULE/src/main.rs"
echo "  - Commands: cli/$TOOL_MODULE/src/commands/"
echo "  - Tests: cli/$TOOL_MODULE/tests/"
echo ""
echo "📝 Next steps:"
echo "  1. Replace example command with actual commands"
echo "  2. Implement command logic in src/commands/"
echo "  3. Update main.rs with your Commands enum"
echo "  4. Run: cargo test -p $TOOL_MODULE"
echo "  5. Build release: cargo build --release --bin $TOOL_NAME"
echo "  6. Measure startup: time ./target/release/$TOOL_NAME --version"
echo "  7. Check size: ls -lh target/release/$TOOL_NAME"
echo ""
echo "🎯 Performance targets:"
echo "  - Startup time: < 10ms"
echo "  - Binary size: < 3MB (stripped)"
echo "  - Memory usage: < 10MB"
echo ""
echo "🔧 Testing:"
echo "  - Run tests: cargo test -p $TOOL_MODULE"
echo "  - Test JSON: $TOOL_NAME example --format json"
echo "  - Test quiet: $TOOL_NAME example --format quiet"
echo ""
echo "📚 Documentation:"
echo "  - Command guide: .claude/commands/create-cli-tool.md"
echo "  - Workflow: .claude/workflows/new-cli-tool.md"
echo ""
echo "Happy building! 🚀"
