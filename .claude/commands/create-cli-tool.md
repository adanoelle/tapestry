# Command: Create CLI Tool

**Name**: create-cli-tool
**Description**: Scaffold a new CLI tool following Tapestry's agent-friendly standards
**Parameters**: `$TOOL_NAME`, `$DESCRIPTION`
**Example**: `/create-cli-tool rfd "Manage RFD documents with structured operations"`

---

## Overview

This command scaffolds a fast, agent-friendly CLI tool. Unlike MCP tools, CLI tools use a simple flat architecture optimized for quick startup and JSON output.

**CLI Tools vs MCP Tools**:

| Aspect | CLI Tool | MCP Tool |
|--------|----------|----------|
| Startup | < 10ms | 100ms+ |
| Architecture | Flat, simple | Hexagonal |
| Invocation | Skills via Bash | MCP protocol |
| State | Stateless | Can be stateful |
| Output | JSON, pretty, quiet | MCP protocol |
| Use Case | Fast operations | Deep integration |

---

## Phase 1: Design & Planning

### Step 1.1: Validate CLI Tool is Appropriate

Ask yourself:
- ✅ Does this need fast startup (< 10ms)?
- ✅ Is this primarily CRUD operations on files?
- ✅ Should Skills be able to invoke this?
- ✅ Does this need JSON output for agents?
- ❌ Does this need stateful operations? (Use MCP instead)
- ❌ Does this need deep protocol integration? (Use MCP instead)

**If mostly ✅**: Proceed with CLI tool

**If mostly ❌**: Consider MCP tool instead

### Step 1.2: Define Commands

Plan the command structure:

```yaml
tool_name: {TOOL_NAME}
description: {DESCRIPTION}
commands:
  - name: {command1}
    description: {What it does}
    args: [{arg1}, {arg2}]
    options: [{opt1}, {opt2}]

  - name: {command2}
    description: {What it does}
    args: [{arg1}]
    options: [{opt1}]
```

### Step 1.3: Design Output Formats

Define JSON schema for each command:

```json
// Success response
{
  "status": "success",
  "data": {
    // Command-specific data
  }
}

// Error response
{
  "error": "ERROR_CODE",
  "message": "Human-readable message",
  "details": { /* context */ },
  "suggestion": {
    "command": "suggested command",
    "description": "why this might work"
  }
}
```

---

## Phase 2: Scaffold Structure

### Step 2.1: Create CLI Tool Directory

```bash
# Convert kebab-case to snake_case for Rust
TOOL_MODULE=$(echo "$TOOL_NAME" | tr '-' '_')

# Create directory structure
mkdir -p cli/$TOOL_MODULE/src/commands
mkdir -p cli/$TOOL_MODULE/tests

echo "✅ Created directory: cli/$TOOL_MODULE"
```

### Step 2.2: Create Cargo.toml

```bash
cat > cli/$TOOL_MODULE/Cargo.toml << EOF
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

echo "✅ Created Cargo.toml with size optimizations"
```

### Step 2.3: Create Main Entry Point

```bash
cat > cli/$TOOL_MODULE/src/main.rs << 'EOF'
use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod output;
mod error;

/// $DESCRIPTION
#[derive(Parser, Debug)]
#[command(name = "$TOOL_NAME")]
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
    /// TODO: Command 1 description
    Command1 {
        /// TODO: Argument description
        #[arg(short, long)]
        arg1: Option<String>,
    },

    /// TODO: Command 2 description
    Command2 {
        /// TODO: Positional argument
        id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Command1 { arg1 } => {
            commands::command1::execute(arg1, &cli.format)?;
        }
        Commands::Command2 { id } => {
            commands::command2::execute(&id, &cli.format)?;
        }
    }

    Ok(())
}
EOF

echo "✅ Created main.rs with clap structure"
```

### Step 2.4: Create Output Module

```bash
cat > cli/$TOOL_MODULE/src/output.rs << 'EOF'
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

echo "✅ Created output.rs with three output modes"
```

### Step 2.5: Create Error Module

```bash
cat > cli/$TOOL_MODULE/src/error.rs << 'EOF'
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

echo "✅ Created error.rs with typed errors"
```

### Step 2.6: Create Commands Module

```bash
mkdir -p cli/$TOOL_MODULE/src/commands

cat > cli/$TOOL_MODULE/src/commands/mod.rs << 'EOF'
pub mod command1;
pub mod command2;
EOF

cat > cli/$TOOL_MODULE/src/commands/command1.rs << 'EOF'
use anyhow::Result;
use crate::output;

pub fn execute(arg1: Option<String>, format: &str) -> Result<()> {
    // TODO: Implement command logic

    output::print_success(
        format,
        "Command 1 executed successfully",
        Some(serde_json::json!({
            "arg1": arg1,
        }))
    );

    Ok(())
}
EOF

cat > cli/$TOOL_MODULE/src/commands/command2.rs << 'EOF'
use anyhow::Result;
use crate::output;

pub fn execute(id: &str, format: &str) -> Result<()> {
    // TODO: Implement command logic

    output::print_success(
        format,
        &format!("Command 2 executed for: {}", id),
        Some(serde_json::json!({
            "id": id,
        }))
    );

    Ok(())
}
EOF

echo "✅ Created command modules"
```

### Step 2.7: Create README

```bash
cat > cli/$TOOL_MODULE/README.md << EOF
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

# Command 1
$TOOL_NAME command1 --arg1 value

# Command 2
$TOOL_NAME command2 <id>

# JSON output (agent-friendly)
$TOOL_NAME command1 --format json

# Quiet mode (errors only)
$TOOL_NAME command1 --format quiet
\`\`\`

## Output Formats

### Pretty (Human-Readable)

\`\`\`bash
$ $TOOL_NAME command1
✅ Command 1 executed successfully
{
  "arg1": "value"
}
\`\`\`

### JSON (Agent-Friendly)

\`\`\`bash
$ $TOOL_NAME command1 --format json
{"status":"success","message":"Command 1 executed successfully","data":{"arg1":"value"}}
\`\`\`

### Quiet (Errors Only)

\`\`\`bash
$ $TOOL_NAME command1 --format quiet
# No output on success
\`\`\`

## Error Handling

Errors return non-zero exit codes and structured messages:

\`\`\`bash
$ $TOOL_NAME command1 --invalid
❌ Error: Invalid input
💡 Try: $TOOL_NAME command1 --arg1 value
   Provide a valid argument value
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

## Contributing

See the RFC in \`docs/design/features/RFC-XXX-$TOOL_NAME.md\`
EOF

echo "✅ Created README.md"
```

---

## Phase 3: Testing

### Step 3.1: Create CLI Tests

```bash
cat > cli/$TOOL_MODULE/tests/cli_tests.rs << 'EOF'
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
fn test_command1_pretty() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("command1")
        .arg("--arg1")
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅"));
}

#[test]
fn test_command1_json() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("command1")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::is_json());
}

#[test]
fn test_command1_quiet() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("command1")
        .arg("--format")
        .arg("quiet")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}
EOF

echo "✅ Created CLI integration tests"
```

### Step 3.2: Run Tests

```bash
# Compile and run tests
cargo test -p $TOOL_MODULE

# Check binary size
cargo build --release --bin $TOOL_NAME
ls -lh target/release/$TOOL_NAME

# Measure startup time
hyperfine --warmup 3 "./target/release/$TOOL_NAME --version" || \
  time ./target/release/$TOOL_NAME --version

echo "✅ Tests complete"
```

---

## Phase 4: Optimization

### Step 4.1: Binary Size Optimization

Already configured in Cargo.toml:
- `opt-level = "z"`: Optimize for size
- `lto = true`: Link-time optimization
- `strip = true`: Remove debug symbols
- `codegen-units = 1`: Better optimization

### Step 4.2: Startup Time Optimization

Tips:
- Minimize dependencies
- Use `cargo bloat` to find large dependencies
- Lazy-load heavy operations
- Avoid regex compilation at startup

```bash
# Analyze binary bloat
cargo install cargo-bloat
cargo bloat --release -n 10

echo "✅ Optimization analysis complete"
```

---

## Phase 5: Documentation & Integration

### Step 5.1: Add to Workspace

The workspace Cargo.toml already includes `cli/*`, so the new tool is auto-discovered.

### Step 5.2: Update Project Documentation

```bash
# Update CLI tools README
if [ ! -f cli/README.md ]; then
    cat > cli/README.md << 'EOF'
# Tapestry CLI Tools

Fast, agent-friendly CLI tools optimized for Skills invocation.

## Available Tools

EOF
fi

cat >> cli/README.md << EOF

### $TOOL_NAME

**Description**: $DESCRIPTION
**Binary**: \`$TOOL_NAME\`
**Location**: \`cli/$TOOL_MODULE/\`
**Status**: Active

EOF
```

---

## Best Practices

### Command Design

**Do**:
- Use clear, descriptive command names
- Provide helpful --help text
- Include examples in help
- Support all three output formats
- Provide actionable error messages

**Don't**:
- Use prompts (non-interactive only)
- Print to stdout AND stderr
- Assume terminal capabilities
- Hardcode paths or values

### Error Handling

**Good Error**:
```
❌ Error: Configuration file not found: config.toml
💡 Try: Create config.toml or use --config path/to/config.toml
   The tool looks for config.toml in the current directory
```

**Bad Error**:
```
Error: file not found
```

### Output Design

**Pretty Output**: For humans
- Use colors and emojis
- Format for readability
- Include context

**JSON Output**: For agents
- Valid JSON always
- Consistent schema
- Include all data

**Quiet Output**: For scripts
- Nothing on success
- Errors to stderr only

---

## Examples

### Example 1: RFD CLI

```bash
/create-cli-tool rfd "Manage RFD documents"

# Commands:
- rfd create --title "..." --author "..."
- rfd list --status draft
- rfd show 003
- rfd status 003 --set review
```

### Example 2: Code Analyzer

```bash
/create-cli-tool code-analyzer "Analyze code complexity"

# Commands:
- code-analyzer check src/
- code-analyzer report --format json
- code-analyzer fix --auto
```

---

## Final Checklist

- [ ] Cargo.toml configured with size optimizations
- [ ] Main.rs with clap structure
- [ ] All three output formats implemented
- [ ] Error types defined with codes
- [ ] Command modules created
- [ ] CLI tests written
- [ ] README with examples
- [ ] Binary size < 3MB
- [ ] Startup time < 10ms
- [ ] JSON output validated

---

**Remember**: CLI tools should be simple, fast, and agent-friendly. No complex architecture needed!
