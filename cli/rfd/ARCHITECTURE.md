# RFD CLI Architecture

> **For Junior Developers**: This document explains how the RFD CLI is
> structured, why it's designed this way, and where to find things. Start here
> before diving into the code!

## Table of Contents

- [Overview](#overview)
- [Design Principles](#design-principles)
- [Directory Structure](#directory-structure)
- [Module Responsibilities](#module-responsibilities)
- [Key Design Patterns](#key-design-patterns)
- [Data Flow](#data-flow)
- [Testing Strategy](#testing-strategy)
- [Getting Started](#getting-started)

## Overview

The RFD CLI is a Rust-based command-line tool for managing RFD (Request for
Discussion) documents. It's designed to be:

- **Agent-friendly**: Structured JSON output, idempotent operations
- **Fast**: < 10ms startup time, optimized binary size
- **Simple**: Single binary, no runtime dependencies
- **Maintainable**: Clear separation of concerns, well-tested

### Technology Stack

- **Language**: Rust (2021 edition)
- **CLI Framework**: clap 4 (derive macros)
- **Templating**: minijinja 2 (Jinja2-compatible)
- **Serialization**: serde + serde_yaml + serde_json
- **Date/Time**: chrono
- **Error Handling**: thiserror (domain) + anyhow (application)
- **Testing**: assert_cmd, predicates, tempfile

## Design Principles

### 1. Agent-First Design

**Problem**: AI agents need structured, parseable output.

**Solution**:

- JSON output mode (`--format json`) for all commands
- Structured errors with suggestions
- Idempotent operations (safe to retry)
- Exit codes map to error types

**Example**:

```bash
# Human mode (default)
$ rfd list
Found 3 RFD(s)

RFD 0001 [draft] Feature Proposal
  Authors: Alice <alice@example.com>
  ...

# Agent mode
$ rfd list --format json
{
  "rfds": [...],
  "total": 3
}
```

### 2. Performance First

**Problem**: CLI tools are invoked frequently; slow startup kills productivity.

**Solution**:

- Optimized release profile (LTO, size optimization)
- Minimal dependencies
- No lazy initialization
- Binary size: 2.4MB (target < 3MB)
- Startup time: 1ms (target < 10ms)

### 3. Clear Separation of Concerns

**Problem**: Mixed responsibilities make code hard to understand and test.

**Solution**: Layered architecture with clear boundaries:

```
CLI Layer (main.rs)
    ↓
Command Layer (commands/*)
    ↓
Domain Layer (document.rs, error.rs)
    ↓
Infrastructure Layer (fs.rs, template.rs, config.rs)
```

Each layer only depends on layers below it, never above.

### 4. Junior Developer Friendly

**Problem**: Code can be intimidating for newcomers.

**Solution**:

- Comprehensive module documentation
- Inline comments explaining complex logic
- Type aliases for domain concepts
- "For Junior Developers" sections throughout
- Clear error messages with suggestions

## Directory Structure

```
rfd/
├── src/
│   ├── main.rs              # CLI entry point (100 lines)
│   ├── commands/            # Command implementations
│   │   ├── mod.rs           # Command module docs
│   │   ├── create.rs        # Create new RFDs
│   │   ├── list.rs          # List and filter RFDs
│   │   ├── search.rs        # Search RFDs by content
│   │   ├── show.rs          # Show RFD details
│   │   ├── status.rs        # Update RFD state
│   │   ├── update.rs        # Update metadata
│   │   └── validate.rs      # Validate RFD structure
│   ├── document.rs          # Domain models (state machine, metadata)
│   ├── error.rs             # Error types with suggestions
│   ├── fs.rs                # File I/O (atomic writes, YAML parsing)
│   ├── config.rs            # Configuration loading
│   ├── output.rs            # Output formatting (Pretty/JSON/Quiet)
│   └── template.rs          # Template rendering engine
├── tests/
│   └── integration_test.rs  # End-to-end tests (31 tests)
├── templates/
│   └── default.md.jinja     # Built-in RFD template
├── Cargo.toml               # Dependencies and release profile
├── README.md                # User documentation
├── ARCHITECTURE.md          # This file
└── CONTRIBUTING.md          # Contribution guide
```

## Module Responsibilities

### main.rs - CLI Entry Point

**Purpose**: Parse command-line arguments and dispatch to commands.

**Key Types**:

- `Cli` - Top-level CLI structure
- `Commands` - Subcommand enum
- `OutputFormat` - Output mode selection

**Responsibilities**:

- Argument parsing with clap
- Error handling and exit codes
- Output format selection

**For Junior Developers**: Start reading here to understand the program flow.
It's only ~100 lines and shows how everything connects.

### commands/\* - Command Implementations

**Purpose**: Implement business logic for each subcommand.

**Pattern**: All commands follow the same structure:

```rust
pub fn execute(
    /* command-specific args */,
    output: &Output,
) -> Result<(), RfdError> {
    // 1. Load configuration
    let config = RfdConfig::load()?;

    // 2. Perform operation
    let result = do_something(&config)?;

    // 3. Display results
    output.show_result(&result)?;

    Ok(())
}
```

**Idempotency**:

- `create`: Not idempotent (returns error if exists)
- `status`: Idempotent (setting same state twice succeeds)
- `update`: Idempotent (always succeeds if valid)
- `list/show/validate`: Read-only (always idempotent)
- `search`: Read-only (always idempotent)

**Search Command** (`search.rs`):

The search command provides fast, flexible searching across RFD documents:

**Key Features**:
- Field-specific search via `--in` flag (title, content, tags, metadata, all)
- Multiple search terms with AND logic (all terms must match)
- Case-sensitive and case-insensitive modes
- Integration with existing filters (--status, --author, --limit)
- JSON output for AI agents

**Search Scopes** (`SearchScope` enum):
```rust
pub enum SearchScope {
    Title,       // Search title field only
    Content,     // Search markdown body only
    Tags,        // Search tags array only
    Metadata,    // Search title + tags + authors
    All,         // Search title + content (default)
}
```

**Algorithm**:
1. Parse query into terms (split on whitespace)
2. Load all RFD files via `find_all_rfds()`
3. Apply status/author filters first (early exit for performance)
4. Apply search matching (all terms must match - AND logic)
5. Sort results by RFD number (descending - newest first)
6. Apply limit if specified
7. Output via `Output::list()` (reuses list display)

**Performance**: Sequential search with no indexing
- 100 RFDs: ~110ms
- 1,000 RFDs: ~1.1s
- Future: Add `.rfd/index.json` for instant search when needed

**Example Usage**:
```bash
# Basic search
rfd search "authentication"

# Field-specific
rfd search "oauth" --in title

# Multiple terms (AND)
rfd search "oauth api"

# With filters
rfd search "security" --status draft --author alice
```

### document.rs - Domain Models

**Purpose**: Define core RFD types and business rules.

**Key Types**:

- `RfdState` - State machine enum (draft/review/accepted/etc.)
- `RfdMetadata` - YAML frontmatter structure
- `RfdDocument` - Complete document (metadata + content)
- `RfdSummary` - Lightweight list representation
- `RfdNumber` - Type alias for u32
- `RfdId` - Type alias for formatted string

**State Machine**:

```
draft ──> review ──> accepted ──> implemented
  │         │           │
  └─────────┴───────────┴────> rejected ──> archived
```

**Validation**:

- `can_transition_to()` - Check if state transition is valid
- `valid_next_states()` - Get allowed next states
- `validate()` - Check metadata completeness

**For Junior Developers**: This is the heart of the domain logic. The state
machine uses exhaustive pattern matching to ensure all transitions are
explicitly handled - no implicit behavior!

### error.rs - Error Types

**Purpose**: Structured errors with actionable suggestions.

**Key Types**:

- `RfdError` - Domain error enum
- `ErrorResponse` - JSON-serializable error with suggestions
- `Suggestion` - Command to fix the error

**Design**:

- Each error variant includes a helpful message
- `to_response()` method adds suggestions
- Exit codes map to error types:
  - 0 = success
  - 1 = general error
  - 2 = validation error
  - 3 = state transition error

**For Junior Developers**: Good error messages are a feature! Always include
enough context for the user (or agent) to fix the problem.

### fs.rs - File System Operations

**Purpose**: All file I/O for RFD documents.

**Key Functions**:

- `find_all_rfds()` - Scan directory for RFD files
- `find_rfd_by_id()` - Find specific RFD by number
- `load_rfd()` - Parse file into RfdDocument
- `save_rfd()` - Write RfdDocument to file (atomic!)
- `next_rfd_number()` - Get next available ID

**Atomic Writes**:

```rust
// Write to temp file first
fs::write("file.md.tmp", content)?;

// Rename (atomic on POSIX!)
fs::rename("file.md.tmp", "file.md")?;
```

**For Junior Developers**: Atomic writes prevent file corruption. The rename
operation is atomic on POSIX systems, so you never have a partially-written
file. This is a standard technique for safe file updates!

### config.rs - Configuration

**Purpose**: Load and manage RFD configuration.

**Hierarchy**:

1. Project config (`.rfd/config.toml`) - highest priority
2. User config (`~/.config/rfd/config.toml`)
3. Built-in defaults - fallback

**Key Settings**:

- `rfd_directory` - Where to store RFDs
- `template_directory` - Custom templates
- `id_format` - Number formatting (e.g., `{:04d}`)

**For Junior Developers**: The configuration hierarchy follows the "principle of
least surprise" - project settings override user settings override defaults.

### output.rs - Output Formatting

**Purpose**: Format output in three modes for different consumers.

**Modes**:

- **Pretty**: Human-readable with colors (default)
- **JSON**: Structured data for agents
- **Quiet**: Errors only

**Key Methods**:

- `success()` - Print success message
- `error()` - Print error with suggestions
- `list()` - Format RFD list
- `show()` - Format RFD details
- `created()` - Format create result

**For Junior Developers**: The Output abstraction lets us add new formats (like
YAML or CSV) without changing any command code!

### template.rs - Template Engine

**Purpose**: Render RFD documents from Jinja2 templates.

**Key Design**:

- Templates stored as owned Strings (not 'static)
- Fresh Environment created per render (simple, fast enough)
- Built-in default template + custom template support

**Memory Management**: Previously used `Box::leak` which caused memory leaks.
Now uses:

```rust
pub struct TemplateEngine {
    templates: HashMap<String, String>,  // Owned!
}

pub fn render(&self, name: &str, ...) -> Result<String> {
    let content = self.templates.get(name)?;
    let mut env = Environment::new();  // Fresh each time
    env.add_template(name, content)?;
    // ...render...
}
```

**For Junior Developers**: The performance cost of creating a fresh Environment
is < 1ms, which is negligible for a CLI tool. The simplicity gain is worth it!

## Key Design Patterns

### 1. State Machine

**Location**: `document.rs:69-128`

**Purpose**: Enforce valid RFD lifecycle transitions.

**Implementation**:

```rust
impl RfdState {
    pub fn can_transition_to(&self, target: &RfdState) -> bool {
        use RfdState::*;
        match (self, target) {
            // Idempotent - same state is always allowed
            (a, b) if a == b => true,

            // Draft can go to review, accepted, or rejected
            (Draft, Review) | (Draft, Accepted) | (Draft, Rejected) => true,

            // ...exhaustive matching...

            // All other transitions are invalid
            _ => false,
        }
    }
}
```

**Why Exhaustive Matching**: Forces us to explicitly handle every case. Adding a
new state? The compiler will tell you everywhere you need to update!

### 2. Builder Pattern

**Location**: `document.rs:187-203`

**Purpose**: Create RfdMetadata with sensible defaults.

**Implementation**:

```rust
impl RfdMetadata {
    pub fn new(title: String, authors: Vec<String>) -> Self {
        Self {
            title,
            authors,
            state: RfdState::Draft,  // Default
            discussion: None,
            created: Utc::now(),
            updated: Utc::now(),
            tags: Vec::new(),
        }
    }
}
```

**For Junior Developers**: Builder pattern = required fields as parameters,
optional fields get defaults. Clear and hard to misuse!

### 3. Error Propagation with ?

**Location**: Throughout, e.g., `commands/create.rs`

**Pattern**:

```rust
pub fn execute(...) -> Result<(), RfdError> {
    let config = RfdConfig::load()?;  // Propagate error
    let number = next_rfd_number(&config)?;  // Propagate error
    // ...
    Ok(())
}
```

**For Junior Developers**: The `?` operator is shorthand for:

```rust
match result {
    Ok(value) => value,
    Err(e) => return Err(e.into()),
}
```

This keeps code clean while properly handling errors!

### 4. Type Aliases for Domain Concepts

**Location**: `document.rs:74-83`

**Purpose**: Make code self-documenting.

**Implementation**:

```rust
pub type RfdNumber = u32;  // The numeric ID (1, 2, 3...)
pub type RfdId = String;   // Formatted string ("0001", "0042"...)
```

**Usage**:

```rust
// Before: What does u32 mean here?
pub fn create(number: u32) -> String { ... }

// After: Crystal clear!
pub fn create(number: RfdNumber) -> RfdId { ... }
```

## Data Flow

### Creating an RFD

```
User Input
  ↓
clap parsing (main.rs)
  ↓
Commands::Create { title, author, template }
  ↓
create::execute()
  ├─ Load config (config.rs)
  ├─ Get next number (fs.rs)
  ├─ Create metadata (document.rs)
  ├─ Validate metadata (document.rs)
  ├─ Render template (template.rs)
  ├─ Create document (document.rs)
  └─ Save to file (fs.rs - atomic write!)
  ↓
output.created() (output.rs)
  ↓
Pretty/JSON/Quiet output
```

### Loading an RFD

```
File Path
  ↓
fs::read_to_string()
  ↓
extract_yaml_frontmatter()
  ├─ Find "---" delimiters
  ├─ Extract YAML between them
  └─ Return YAML string
  ↓
serde_yaml::from_str()
  ↓
RfdMetadata
  ↓
gray_matter.parse()
  ↓
Markdown content
  ↓
RfdDocument { number, metadata, content, path }
```

### Updating RFD State

```
User Input: rfd status 42 --set review
  ↓
status::execute(id: "42", new_state: "review")
  ├─ Find RFD file by ID (fs.rs)
  ├─ Load RFD (fs.rs)
  ├─ Parse new state (document.rs)
  ├─ Check if transition is valid (document.rs)
  ├─ Update state and timestamp
  └─ Save RFD (fs.rs - atomic write!)
  ↓
output.status_updated()
  ↓
"✓ Updated RFD 0042 state: [draft] → [review]"
```

## Testing Strategy

We use a **hybrid approach** following Rust conventions:

### Unit Tests (Inline in Modules)

**Location**: `#[cfg(test)] mod tests { ... }` at end of each module

**Purpose**: Test logic in isolation

**What to test**:

- State machine transitions (document.rs)
- Parsing functions (fs.rs, config.rs)
- Validation logic (document.rs)
- Template rendering (template.rs)
- Helper functions

**Example**:

```rust
// In document.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        assert!(RfdState::Draft.can_transition_to(&RfdState::Review));
        assert!(!RfdState::Draft.can_transition_to(&RfdState::Implemented));
    }
}
```

**Current coverage**: 16 unit tests across 7 modules

### Integration Tests (Dedicated Directory)

**Location**: `tests/integration_test.rs`

**Purpose**: Test end-to-end workflows

**What to test**:

- Complete command execution
- File I/O with real temp directories
- JSON output parsing
- Error handling and exit codes
- Multi-step workflows

**Example**:

```rust
// In tests/integration_test.rs
#[test]
fn test_create_rfd_success() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args(&["create", "--title", "Test", "--author", "Alice <a@test.com>"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created RFD 0001"));

    // Verify file exists
    assert!(temp_dir.path().join("rfds/0001-test.md").exists());
}
```

**Current coverage**: 31 integration tests (15 for search command)

### When to Write Which Test

**Unit Test** if:

- Testing pure functions
- Testing domain logic
- No file I/O required
- Fast (< 1ms per test)

**Integration Test** if:

- Testing command execution
- Testing file I/O
- Testing multiple components together
- Testing error messages and exit codes

**For Junior Developers**: When in doubt, write an integration test! They catch
more bugs and are easier to understand. Unit tests are for complex logic that's
hard to set up in integration tests.

## Getting Started

### Recommended Reading Order

1. **main.rs** (100 lines) - See how everything connects
2. **document.rs** (450 lines) - Understand the domain model
3. **commands/create.rs** (90 lines) - See a complete command
4. **fs.rs** (340 lines) - Understand file operations
5. **Other modules** as needed

### Common Tasks

#### Adding a New Command

1. Create `src/commands/your_command.rs`
2. Add `pub mod your_command;` to `src/commands/mod.rs`
3. Add command struct to `main.rs`:
   ```rust
   #[derive(Subcommand)]
   enum Commands {
       // ...existing commands...
       YourCommand {
           #[arg(short, long)]
           your_arg: String,
       },
   }
   ```
4. Add match arm in `main.rs`:
   ```rust
   Commands::YourCommand { your_arg } => {
       commands::your_command::execute(your_arg, &output)?;
   }
   ```
5. Implement `execute()` function following the pattern
6. Add integration test in `tests/integration_test.rs`

#### Adding a New RFD State

**WARNING**: This requires changes in multiple places!

1. Add enum variant to `RfdState` in `document.rs`
2. Update `can_transition_to()` - compiler will show what to add
3. Update `valid_next_states()`
4. Update `Display` impl
5. Update `FromStr` impl
6. Add tests for new transitions
7. Update documentation

#### Modifying the Template

1. Edit `templates/default.md.jinja`
2. Test with: `cargo run -- create --title "Test" --author "Me <me@test.com>"`
3. Or add custom template to `.rfd/templates/`

### Common Pitfalls

**Problem**: "File not found" when running tests **Solution**: Tests use
temporary directories. Never hardcode paths!

**Problem**: State transition fails silently **Solution**: Check
`can_transition_to()` - state machine is strict!

**Problem**: Template rendering fails **Solution**: Check template syntax. Use
`{{ variable }}` not `$variable`

**Problem**: Binary size exploded **Solution**: Check dependencies. We optimize
for size (`opt-level = "z"`)

### Getting Help

- Read module documentation (`//!` at top of files)
- Check inline comments for complex logic
- Look at existing tests for examples
- See CONTRIBUTING.md for contribution guidelines
- Ask in project discussions!

## Future Enhancements

Potential areas for improvement (RFCs welcome!):

1. **Search**: Full-text search across RFD content
2. **Export**: Generate HTML/PDF from RFDs
3. **Git Integration**: Auto-commit on state changes
4. **Dependencies**: Track dependencies between RFDs
5. **Comments**: Inline comments on RFD sections
6. **Diff**: Show changes between RFD versions

## License

MIT OR Apache-2.0
