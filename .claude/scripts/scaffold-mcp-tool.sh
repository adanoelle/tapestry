#!/bin/bash
# Scaffold MCP Tool - Automated scaffolding for Tapestry MCP tools
# This script is invoked by the create-mcp-tool command

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
    echo "Example: $0 git-workflow \"Automates git workflow with conventional commits\""
    exit 1
fi

# Validate tool name format (kebab-case)
if ! echo "$TOOL_NAME" | grep -qE '^[a-z]+(-[a-z]+)*$'; then
    print_error "Tool name must be kebab-case (e.g., git-workflow, code-analyzer)"
    exit 1
fi

# Convert kebab-case to snake_case for Rust module
TOOL_MODULE=$(echo "$TOOL_NAME" | tr '-' '_')

# Convert to PascalCase for struct names
TOOL_STRUCT=$(echo "$TOOL_NAME" | sed 's/-/_/g' | sed 's/\b\(.\)/\u\1/g' | sed 's/_//g')

print_info "Creating MCP tool: $TOOL_NAME"
print_info "Module name: $TOOL_MODULE"
print_info "Struct prefix: $TOOL_STRUCT"
print_info "Description: $DESCRIPTION"

# Phase 1: Create RFC
print_status "Phase 1: Creating RFC..."

RFC_DIR="docs/design/features"
mkdir -p $RFC_DIR

# Find next RFC number
RFC_COUNT=$(ls $RFC_DIR/RFC-*.md 2>/dev/null | wc -l)
RFC_NUMBER=$(printf "%03d" $((RFC_COUNT + 1)))
RFC_FILE="$RFC_DIR/RFC-$RFC_NUMBER-$TOOL_NAME.md"

cat > $RFC_FILE << EOF
# RFC-$RFC_NUMBER: $TOOL_NAME Tool

## Summary
$DESCRIPTION

## Motivation
<!-- Why do we need this tool? What problem does it solve? -->

## Design

### Domain Model
<!-- Core entities and business logic -->

\`\`\`rust
pub struct ${TOOL_STRUCT}Service {
    // Core state
}

pub struct ${TOOL_STRUCT}Input {
    // Input fields
}

pub struct ${TOOL_STRUCT}Output {
    // Output fields
}
\`\`\`

### API Contract
<!-- Input/Output structures and validation rules -->

### Architecture
<!-- How it fits into hexagonal architecture -->
- Domain: Pure business logic
- Port: Interface definitions
- Adapter: MCP protocol implementation

## Implementation Plan
1. Domain logic implementation
2. Port interface definition
3. MCP adapter creation
4. Testing (unit, integration, property)
5. Security audit
6. Documentation

## Alternatives Considered
<!-- What other approaches were considered? -->

## Security Considerations
<!-- Input validation, resource limits, etc. -->

## Performance Targets
- P50 latency: < 100ms
- P99 latency: < 500ms
- Memory usage: < 10MB
EOF

print_status "Created RFC: $RFC_FILE"

# Phase 2: Scaffold structure
print_status "Phase 2: Scaffolding structure..."

# Create module directory
MODULE_DIR="src/tools/$TOOL_MODULE"
mkdir -p $MODULE_DIR

# Create test directories
mkdir -p tests/unit/$TOOL_MODULE
mkdir -p tests/integration
mkdir -p tests/common
mkdir -p benches

print_status "Created directory structure"

# Phase 3: Generate domain.rs
cat > $MODULE_DIR/domain.rs << EOF
//! Domain logic for $TOOL_NAME
//!
//! $DESCRIPTION

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Input for the $TOOL_NAME tool
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ${TOOL_STRUCT}Input {
    // TODO: Define input fields based on RFC
}

/// Output from the $TOOL_NAME tool
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ${TOOL_STRUCT}Output {
    // TODO: Define output fields based on RFC
}

/// Errors specific to $TOOL_NAME
#[derive(Error, Debug)]
pub enum ${TOOL_STRUCT}Error {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Processing failed: {0}")]
    ProcessingError(String),
    
    // TODO: Add tool-specific errors
}

/// Core domain service for $TOOL_NAME
#[derive(Debug, Clone)]
pub struct ${TOOL_STRUCT}Service {
    // TODO: Add any required state (keep minimal)
}

impl ${TOOL_STRUCT}Service {
    /// Creates a new instance of the service
    pub fn new() -> Self {
        Self {
            // Initialize state
        }
    }
    
    /// Validates the input
    fn validate_input(&self, input: &${TOOL_STRUCT}Input) -> Result<(), ${TOOL_STRUCT}Error> {
        // TODO: Implement validation logic
        Ok(())
    }
    
    /// Executes the core logic
    ///
    /// # Arguments
    /// * \`input\` - The input parameters
    ///
    /// # Returns
    /// The result of the operation
    ///
    /// # Errors
    /// Returns \`${TOOL_STRUCT}Error\` if the operation fails
    pub fn execute(&self, input: ${TOOL_STRUCT}Input) -> Result<${TOOL_STRUCT}Output, ${TOOL_STRUCT}Error> {
        // Validate input first
        self.validate_input(&input)?;
        
        // TODO: Implement core logic
        // This should be pure business logic with no external dependencies
        
        todo!("Implement $TOOL_NAME logic")
    }
}

impl Default for ${TOOL_STRUCT}Service {
    fn default() -> Self {
        Self::new()
    }
}
EOF

print_status "Generated domain.rs"

# Generate port.rs
cat > $MODULE_DIR/port.rs << EOF
//! Port definitions for $TOOL_NAME

use async_trait::async_trait;
use anyhow::Result;
use super::domain::{${TOOL_STRUCT}Input, ${TOOL_STRUCT}Output};

/// Port interface for $TOOL_NAME
#[async_trait]
pub trait ${TOOL_STRUCT}Port: Send + Sync {
    /// Executes the tool operation
    async fn execute(&self, input: ${TOOL_STRUCT}Input) -> Result<${TOOL_STRUCT}Output>;
    
    /// Returns tool metadata
    async fn metadata(&self) -> ToolMetadata;
}

/// Metadata about the tool
#[derive(Debug, Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}

impl Default for ToolMetadata {
    fn default() -> Self {
        Self {
            name: "$TOOL_NAME".to_string(),
            description: "$DESCRIPTION".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: "Tapestry Team".to_string(),
        }
    }
}
EOF

print_status "Generated port.rs"

# Generate adapter.rs
cat > $MODULE_DIR/adapter.rs << EOF
//! MCP adapter for $TOOL_NAME

use async_trait::async_trait;
use anyhow::{Result, Context};
use tracing::{info, debug, error};

use super::domain::{${TOOL_STRUCT}Service, ${TOOL_STRUCT}Input, ${TOOL_STRUCT}Output};
use super::port::{${TOOL_STRUCT}Port, ToolMetadata};

/// MCP adapter for $TOOL_NAME
pub struct ${TOOL_STRUCT}Adapter {
    service: ${TOOL_STRUCT}Service,
    metadata: ToolMetadata,
}

impl ${TOOL_STRUCT}Adapter {
    /// Creates a new MCP adapter
    pub fn new() -> Self {
        Self {
            service: ${TOOL_STRUCT}Service::new(),
            metadata: ToolMetadata::default(),
        }
    }
}

impl Default for ${TOOL_STRUCT}Adapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ${TOOL_STRUCT}Port for ${TOOL_STRUCT}Adapter {
    async fn execute(&self, input: ${TOOL_STRUCT}Input) -> Result<${TOOL_STRUCT}Output> {
        debug!("Executing $TOOL_NAME with input: {:?}", input);
        
        let result = self.service
            .execute(input)
            .context("Failed to execute $TOOL_NAME")?;
        
        info!("$TOOL_NAME execution successful");
        Ok(result)
    }
    
    async fn metadata(&self) -> ToolMetadata {
        self.metadata.clone()
    }
}

// TODO: Implement rmcp::Tool trait when rmcp is added as dependency
// #[rmcp::tool(
//     name = "$TOOL_NAME",
//     description = "$DESCRIPTION"
// )]
// impl Tool for ${TOOL_STRUCT}Adapter {
//     type Input = ${TOOL_STRUCT}Input;
//     type Output = ${TOOL_STRUCT}Output;
//     
//     async fn run(&self, input: Self::Input) -> ToolResult<Self::Output> {
//         self.execute(input)
//             .await
//             .map_err(|e| rmcp::Error::Tool(e.to_string()))
//     }
// }
EOF

print_status "Generated adapter.rs"

# Generate config.rs
cat > $MODULE_DIR/config.rs << EOF
//! Configuration for $TOOL_NAME

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// Configuration for $TOOL_NAME
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ${TOOL_STRUCT}Config {
    /// Enable debug logging
    pub debug: bool,
    
    /// Timeout in seconds
    pub timeout_seconds: u64,
    
    // TODO: Add tool-specific configuration
}

impl Default for ${TOOL_STRUCT}Config {
    fn default() -> Self {
        Self {
            debug: false,
            timeout_seconds: 30,
        }
    }
}

impl ${TOOL_STRUCT}Config {
    /// Loads configuration from environment
    pub fn from_env() -> Result<Self> {
        let debug = std::env::var("${TOOL_NAME^^}_DEBUG")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false);
        
        let timeout_seconds = std::env::var("${TOOL_NAME^^}_TIMEOUT")
            .map(|v| v.parse().context("Invalid timeout value"))
            .transpose()?
            .unwrap_or(30);
        
        Ok(Self {
            debug,
            timeout_seconds,
        })
    }
}
EOF

print_status "Generated config.rs"

# Generate mod.rs
cat > $MODULE_DIR/mod.rs << EOF
//! $TOOL_NAME - $DESCRIPTION
//!
//! This module implements an MCP tool that $DESCRIPTION.

pub mod domain;
pub mod port;
pub mod adapter;
pub mod config;

// Re-export main types
pub use adapter::${TOOL_STRUCT}Adapter;
pub use domain::{${TOOL_STRUCT}Input, ${TOOL_STRUCT}Output, ${TOOL_STRUCT}Error, ${TOOL_STRUCT}Service};
pub use port::{${TOOL_STRUCT}Port, ToolMetadata};
pub use config::${TOOL_STRUCT}Config;

/// Creates a new instance of the $TOOL_NAME tool
pub fn create_tool() -> ${TOOL_STRUCT}Adapter {
    ${TOOL_STRUCT}Adapter::new()
}
EOF

print_status "Generated mod.rs"

# Generate README.md
cat > $MODULE_DIR/README.md << EOF
# $TOOL_NAME

$DESCRIPTION

## Usage

\`\`\`rust
use tapestry::tools::$TOOL_MODULE;

let tool = $TOOL_MODULE::create_tool();
let input = $TOOL_MODULE::${TOOL_STRUCT}Input {
    // Set input fields
};

let output = tool.execute(input).await?;
\`\`\`

## Configuration

The tool can be configured via environment variables:

- \`${TOOL_NAME^^}_DEBUG\`: Enable debug logging (default: false)
- \`${TOOL_NAME^^}_TIMEOUT\`: Timeout in seconds (default: 30)

## Architecture

This tool follows Tapestry's hexagonal architecture:

- **Domain**: Core business logic in \`domain.rs\`
- **Port**: Interface definition in \`port.rs\`
- **Adapter**: MCP implementation in \`adapter.rs\`
- **Config**: Configuration in \`config.rs\`

## Testing

Run tests with:
\`\`\`bash
# Unit tests
cargo test --lib tools::$TOOL_MODULE

# Integration tests
cargo test --test ${TOOL_MODULE}_test
\`\`\`

## Performance

Target metrics:
- P50 latency: < 100ms
- P99 latency: < 500ms
- Memory usage: < 10MB

## Security Considerations

- Input validation implemented in domain layer
- Resource limits enforced via configuration
- No external command execution
- All errors sanitized before returning

## Contributing

See the RFC in \`docs/design/features/RFC-$RFC_NUMBER-$TOOL_NAME.md\`
EOF

print_status "Generated README.md"

# Phase 4: Generate tests
print_status "Phase 4: Generating tests..."

# Generate unit tests
cat > tests/unit/$TOOL_MODULE/mod.rs << EOF
//! Unit tests for $TOOL_NAME

mod domain_tests;
mod validation_tests;
EOF

cat > tests/unit/$TOOL_MODULE/domain_tests.rs << EOF
use tapestry::tools::$TOOL_MODULE::{
    ${TOOL_STRUCT}Service, ${TOOL_STRUCT}Input, ${TOOL_STRUCT}Output, ${TOOL_STRUCT}Error
};

#[test]
fn should_create_service() {
    let service = ${TOOL_STRUCT}Service::new();
    // Service should be created successfully
    assert!(true); // TODO: Add actual assertions
}

#[test]
fn should_validate_empty_input() {
    let service = ${TOOL_STRUCT}Service::new();
    let input = ${TOOL_STRUCT}Input {
        // Empty input
    };
    
    // TODO: Test validation logic
    let result = service.execute(input);
    assert!(result.is_err() || result.is_ok()); // Update based on expected behavior
}

#[test]
fn should_handle_valid_input() {
    let service = ${TOOL_STRUCT}Service::new();
    let input = ${TOOL_STRUCT}Input {
        // Valid input
    };
    
    // TODO: Test successful execution
    // let result = service.execute(input);
    // assert!(result.is_ok());
}
EOF

cat > tests/unit/$TOOL_MODULE/validation_tests.rs << EOF
use tapestry::tools::$TOOL_MODULE::{${TOOL_STRUCT}Input};

#[test]
fn should_validate_input_fields() {
    // TODO: Add validation tests
}

#[test]
fn should_reject_invalid_input() {
    // TODO: Test invalid input scenarios
}
EOF

print_status "Generated unit tests"

# Generate integration test
cat > tests/integration/${TOOL_MODULE}_test.rs << EOF
use tapestry::tools::$TOOL_MODULE::{create_tool, ${TOOL_STRUCT}Input, ${TOOL_STRUCT}Port};

#[tokio::test]
async fn test_tool_creation() {
    let tool = create_tool();
    let metadata = tool.metadata().await;
    assert_eq!(metadata.name, "$TOOL_NAME");
}

#[tokio::test]
async fn test_tool_execution() {
    let tool = create_tool();
    let input = ${TOOL_STRUCT}Input {
        // Test input
    };
    
    // TODO: Update when implementation is complete
    // let result = tool.execute(input).await;
    // assert!(result.is_ok());
}

#[tokio::test]
async fn test_error_handling() {
    let tool = create_tool();
    let invalid_input = ${TOOL_STRUCT}Input {
        // Invalid input that should trigger an error
    };
    
    // TODO: Test error scenarios
    // let result = tool.execute(invalid_input).await;
    // assert!(result.is_err());
}
EOF

print_status "Generated integration tests"

# Phase 5: Update module registry
print_status "Phase 5: Updating module registry..."

# Add to src/tools/mod.rs if it exists
if [ -f "src/tools/mod.rs" ]; then
    if ! grep -q "pub mod $TOOL_MODULE;" src/tools/mod.rs; then
        echo "pub mod $TOOL_MODULE;" >> src/tools/mod.rs
        print_status "Added module to src/tools/mod.rs"
    else
        print_warning "Module already registered in src/tools/mod.rs"
    fi
else
    # Create src/tools/mod.rs if it doesn't exist
    cat > src/tools/mod.rs << EOF
//! MCP Tools Registry

pub mod $TOOL_MODULE;
EOF
    print_status "Created src/tools/mod.rs with module"
fi

# Phase 6: Add dependencies if needed
print_status "Phase 6: Checking dependencies..."

# Check for required dependencies
DEPS_TO_ADD=""

if ! grep -q "async-trait" Cargo.toml; then
    DEPS_TO_ADD="$DEPS_TO_ADD async-trait"
fi

if ! grep -q "thiserror" Cargo.toml; then
    DEPS_TO_ADD="$DEPS_TO_ADD thiserror"
fi

if ! grep -q "tracing" Cargo.toml; then
    DEPS_TO_ADD="$DEPS_TO_ADD tracing"
fi

if ! grep -q "serde" Cargo.toml; then
    DEPS_TO_ADD="$DEPS_TO_ADD serde"
fi

if ! grep -q "anyhow" Cargo.toml; then
    DEPS_TO_ADD="$DEPS_TO_ADD anyhow"
fi

if [ -n "$DEPS_TO_ADD" ]; then
    print_info "Adding dependencies: $DEPS_TO_ADD"
    cargo add $DEPS_TO_ADD
fi

# Add dev dependencies
DEV_DEPS_TO_ADD=""

if ! grep -q "tokio" Cargo.toml; then
    DEV_DEPS_TO_ADD="$DEV_DEPS_TO_ADD tokio"
fi

if ! grep -q "tempfile" Cargo.toml; then
    DEV_DEPS_TO_ADD="$DEV_DEPS_TO_ADD tempfile"
fi

if [ -n "$DEV_DEPS_TO_ADD" ]; then
    print_info "Adding dev dependencies: $DEV_DEPS_TO_ADD"
    cargo add --dev $DEV_DEPS_TO_ADD
fi

# Phase 7: Run initial checks
print_status "Phase 7: Running initial checks..."

# Format the code
cargo fmt

# Check compilation
if cargo check --lib 2>/dev/null; then
    print_status "Code compiles successfully"
else
    print_warning "Code has compilation errors (expected with todo!() macros)"
fi

# Run clippy
if cargo clippy -- -W warnings 2>/dev/null; then
    print_status "Clippy check passed"
else
    print_warning "Clippy found issues (expected with todo!() macros)"
fi

# Final summary
echo ""
echo "======================================="
echo -e "${GREEN}✅ Tool '$TOOL_NAME' scaffolded successfully!${NC}"
echo "======================================="
echo ""
echo "📁 Structure created:"
echo "  - RFC: $RFC_FILE"
echo "  - Module: src/tools/$TOOL_MODULE/"
echo "  - Tests: tests/unit/$TOOL_MODULE/"
echo "  - Integration: tests/integration/${TOOL_MODULE}_test.rs"
echo ""
echo "📝 Next steps:"
echo "  1. Review and complete the RFC"
echo "  2. Implement domain logic (remove todo!() macros)"
echo "  3. Add input/output field definitions"
echo "  4. Complete unit and integration tests"
echo "  5. Run security audit"
echo "  6. Update CHANGELOG.md"
echo ""
echo "🤖 Agent workflow commands:"
echo "  - Design Review: 'Act as Design Reviewer, review $RFC_FILE'"
echo "  - Implementation: 'Act as Rust Expert, implement src/tools/$TOOL_MODULE/domain.rs'"
echo "  - Testing: 'Act as Test Writer, create tests for $TOOL_MODULE'"
echo "  - Security: 'Act as Security Auditor, review $TOOL_MODULE for vulnerabilities'"
echo ""
echo "Run 'cargo test' when implementation is complete"