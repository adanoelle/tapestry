# Contributing to RFD CLI

> **Welcome!** This guide is designed for developers of all experience levels.
> If you're new to Rust or CLI development, you're in the right place!

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Understanding the Codebase](#understanding-the-codebase)
- [Common Tasks](#common-tasks)
- [Testing Guidelines](#testing-guidelines)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [For Junior Developers](#for-junior-developers)
- [Getting Help](#getting-help)

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Git**: For version control

  ```bash
  # macOS
  brew install git

  # Linux
  sudo apt install git  # Debian/Ubuntu
  sudo dnf install git  # Fedora
  ```

- **A text editor**: VS Code, Vim, Emacs, or your favorite editor

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/tapestry.git
cd tapestry/cli/rfd

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

### Verify Your Setup

```bash
# Should show version info
cargo run -- --version

# Should create an RFD in current directory
cargo run -- create --title "Test" --author "You <you@example.com>"

# Should list the RFD you just created
cargo run -- list
```

If all commands work, you're ready to contribute!

## Development Workflow

### 1. Pick an Issue

- Check [GitHub Issues](https://github.com/yourusername/tapestry/issues)
- Look for `good first issue` or `help wanted` labels
- Comment on the issue to claim it

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Changes

- Write code (see [Common Tasks](#common-tasks))
- Add tests (see [Testing Guidelines](#testing-guidelines))
- Update documentation

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_create_rfd_success

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy

# Build release binary
cargo build --release
```

### 5. Commit Your Changes

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Feature
git commit -m "feat: add search command for RFDs"

# Bug fix
git commit -m "fix: handle empty RFD directory gracefully"

# Documentation
git commit -m "docs: add examples for bulk operations"

# Tests
git commit -m "test: add integration tests for status command"

# Refactoring
git commit -m "refactor: extract filter logic in list command"
```

### 6. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Understanding the Codebase

### Architecture Overview

The RFD CLI follows a layered architecture. See
[ARCHITECTURE.md](ARCHITECTURE.md) for detailed explanation.

```
┌─────────────────────────────────────┐
│   CLI Layer (main.rs)               │  ← Entry point
├─────────────────────────────────────┤
│   Command Layer (commands/*)        │  ← Business logic
├─────────────────────────────────────┤
│   Domain Layer (document.rs)        │  ← Core types
├─────────────────────────────────────┤
│   Infrastructure (fs.rs, config.rs) │  ← File I/O, etc.
└─────────────────────────────────────┘
```

### Where to Find Things

| What You Want               | Where to Look               |
| --------------------------- | --------------------------- |
| Command-line parsing        | `src/main.rs`               |
| Command implementations     | `src/commands/*.rs`         |
| RFD types and state machine | `src/document.rs`           |
| File operations             | `src/fs.rs`                 |
| Error types                 | `src/error.rs`              |
| Output formatting           | `src/output.rs`             |
| Template rendering          | `src/template.rs`           |
| Configuration               | `src/config.rs`             |
| Integration tests           | `tests/integration_test.rs` |

### Recommended Reading Order

1. **Start with [README.md](README.md)** - Understand what the tool does
2. **Read [ARCHITECTURE.md](ARCHITECTURE.md)** - Understand how it's built
3. **Read `src/main.rs`** - See how everything connects (only 100 lines!)
4. **Pick a command** - Read `src/commands/create.rs` or `src/commands/list.rs`
5. **Understand the domain** - Read `src/document.rs` for the state machine

## Common Tasks

### Adding a New Command

Let's add a hypothetical `search` command as an example.

**Step 1**: Create the command module

```bash
touch src/commands/search.rs
```

**Step 2**: Implement the execute function

```rust
// src/commands/search.rs
use crate::config::RfdConfig;
use crate::document::RfdSummary;
use crate::error::RfdError;
use crate::fs::{find_all_rfds, load_rfd};
use crate::output::Output;

pub fn execute(
    query: String,
    output: &Output,
) -> Result<(), RfdError> {
    // 1. Load configuration
    let config = RfdConfig::load()?;

    // 2. Perform operation
    let files = find_all_rfds(&config)?;
    let mut results = Vec::new();

    for path in files {
        let doc = load_rfd(&path)?;
        // Search in title and content
        if doc.metadata.title.contains(&query) || doc.content.contains(&query) {
            results.push(RfdSummary::from(&doc));
        }
    }

    // 3. Display results
    output.list(&results)?;

    Ok(())
}
```

**Step 3**: Register the command in `src/commands/mod.rs`

```rust
// Add to the module list
pub mod search;
```

**Step 4**: Add CLI argument in `src/main.rs`

```rust
#[derive(Subcommand)]
enum Commands {
    // ...existing commands...

    /// Search RFDs by title or content
    Search {
        /// Search query
        query: String,
    },
}
```

**Step 5**: Add dispatch logic in `main()`

```rust
match cli.command {
    // ...existing commands...

    Commands::Search { query } => {
        commands::search::execute(query, &output)?;
    }
}
```

**Step 6**: Add integration test

```rust
// tests/integration_test.rs
#[test]
fn test_search_finds_rfd() {
    let temp_dir = setup_test_env();

    // Create an RFD
    rfd_cmd(&temp_dir)
        .args(&["create", "--title", "Searchable RFD", "--author", "Alice <a@test.com>"])
        .assert()
        .success();

    // Search should find it
    rfd_cmd(&temp_dir)
        .args(&["search", "Searchable"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Searchable RFD"));

    // Search should not find non-existent term
    rfd_cmd(&temp_dir)
        .args(&["search", "NotFound"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Found 0"));
}
```

**Step 7**: Test it!

```bash
cargo test test_search_finds_rfd
cargo run -- search "test"
```

### Adding a New RFD State

**WARNING**: Modifying the state machine affects the entire system!

**Step 1**: Add enum variant

```rust
// src/document.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RfdState {
    Draft,
    Review,
    Accepted,
    Rejected,
    Implemented,
    Archived,
    YourNewState,  // ← Add here
}
```

**Step 2**: Update state transitions

The compiler will tell you what to update! But here's the checklist:

```rust
// Update can_transition_to()
pub fn can_transition_to(&self, target: &RfdState) -> bool {
    use RfdState::*;
    match (self, target) {
        // ...existing transitions...

        // Add transitions to/from your new state
        (YourNewState, SomeOtherState) => true,

        // Don't forget idempotency!
        (YourNewState, YourNewState) => true,

        _ => false,
    }
}

// Update valid_next_states()
pub fn valid_next_states(&self) -> Vec<RfdState> {
    match self {
        // ...existing states...
        YourNewState => vec![SomeOtherState, Archived],
    }
}

// Update Display implementation
impl fmt::Display for RfdState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // ...existing states...
            RfdState::YourNewState => "yournewstate",
        };
        write!(f, "{}", s)
    }
}

// Update FromStr implementation
impl std::str::FromStr for RfdState {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // ...existing states...
            "yournewstate" => Ok(RfdState::YourNewState),
            _ => Err(format!("Invalid state: '{}'...", s)),
        }
    }
}
```

**Step 3**: Update colorization in `src/output.rs`

```rust
fn colorize_state(&self, state: &RfdState) -> ColoredString {
    let state_str = format!("[{}]", state);
    match state {
        // ...existing states...
        RfdState::YourNewState => state_str.magenta(),  // Pick a color
    }
}
```

**Step 4**: Add comprehensive tests

```rust
#[test]
fn test_new_state_transitions() {
    use RfdState::*;

    // Test valid transitions to new state
    assert!(Review.can_transition_to(&YourNewState));

    // Test valid transitions from new state
    assert!(YourNewState.can_transition_to(&Archived));

    // Test idempotency
    assert!(YourNewState.can_transition_to(&YourNewState));

    // Test invalid transitions
    assert!(!Draft.can_transition_to(&YourNewState));
    assert!(!YourNewState.can_transition_to(&Draft));
}
```

**Step 5**: Update documentation

- Update README.md state diagram
- Update ARCHITECTURE.md state machine section
- Add changelog entry

### Modifying Templates

**Built-in template** (`templates/default.md.jinja`):

```jinja2
---
title: "{{ metadata.title }}"
authors: {{ metadata.authors | tojson }}
state: {{ metadata.state }}
created: {{ metadata.created | date(format="%Y-%m-%dT%H:%M:%SZ") }}
updated: {{ metadata.updated | date(format="%Y-%m-%dT%H:%M:%SZ") }}
{% if metadata.tags %}tags: {{ metadata.tags | tojson }}{% endif %}
{% if metadata.discussion %}discussion: "{{ metadata.discussion }}"{% endif %}
---

# Summary

Brief overview of the proposal.

# Motivation

Why does this RFD exist? What problem are we solving?

# Proposal

Detailed technical proposal.

# Implementation

How will this be built?

# Alternatives

What other approaches were considered?

# Open Questions

What remains unresolved?
```

**Custom template** (`.rfd/templates/custom.md.jinja`):

1. Create directory: `mkdir -p .rfd/templates`
2. Create template file with `.md.jinja` extension
3. Use with: `rfd create --template custom --title "..." --author "..."`

**Variables available in templates**:

- `metadata.title` - RFD title
- `metadata.authors` - List of authors
- `metadata.state` - Current state
- `metadata.created` - Creation timestamp
- `metadata.updated` - Last update timestamp
- `metadata.tags` - List of tags (optional)
- `metadata.discussion` - Discussion URL (optional)
- `number` - RFD number (e.g., 42)

## Testing Guidelines

### When to Write Unit Tests

Unit tests go in `#[cfg(test)] mod tests { ... }` at the end of the module file.

**Write unit tests for**:

- Pure functions (no I/O)
- Domain logic (state machine, validation)
- Parsing and formatting functions
- Helper functions

**Example**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_rfd_number_from_filename() {
        assert_eq!(extract_rfd_number(Path::new("0042-feature.md")), Some(42));
        assert_eq!(extract_rfd_number(Path::new("invalid.md")), None);
    }
}
```

### When to Write Integration Tests

Integration tests go in `tests/integration_test.rs`.

**Write integration tests for**:

- Complete command execution
- File I/O operations
- JSON output parsing
- Error handling and exit codes
- Multi-step workflows

**Example**:

```rust
#[test]
fn test_create_and_list_workflow() {
    let temp_dir = setup_test_env();

    // Create two RFDs
    rfd_cmd(&temp_dir)
        .args(&["create", "--title", "First", "--author", "Alice <a@test.com>"])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args(&["create", "--title", "Second", "--author", "Bob <b@test.com>"])
        .assert()
        .success();

    // List should show both
    rfd_cmd(&temp_dir)
        .args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Found 2"));
}
```

### Test Naming Conventions

```rust
// Good: Describes behavior
#[test]
fn should_return_error_when_rfd_not_found() { }

// Better: Uses given_when_then pattern
#[test]
fn given_empty_directory_when_listing_then_shows_no_rfds() { }

// Also good: Simple and clear
#[test]
fn test_state_transition_from_draft_to_review() { }
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_create_rfd_success

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_test

# With output
cargo test -- --nocapture

# Quiet mode (only failures)
cargo test --quiet
```

## Code Style

### Formatting

```bash
# Format code
cargo fmt

# Check without modifying
cargo fmt --check
```

We use default `rustfmt` settings. Let the formatter handle style!

### Linting

```bash
# Run clippy
cargo clippy

# Clippy with pedantic checks
cargo clippy -- -W clippy::pedantic
```

Fix all clippy warnings before submitting PR.

### Documentation

**Module documentation** (`//!` at top of file):

```rust
//! Brief module description.
//!
//! More detailed explanation of what this module does,
//! key types it exports, and how to use it.
```

**Function documentation** (`///` before function):

````rust
/// Brief function description.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `arg1` - Description of arg1
/// * `arg2` - Description of arg2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this function returns an error and why
///
/// # Examples
///
/// ```
/// let result = my_function(arg1, arg2);
/// ```
pub fn my_function(arg1: String, arg2: u32) -> Result<String, Error> {
    // Implementation
}
````

**Inline comments** (for complex logic):

```rust
// Step 1: Convert to lowercase
let lower = title.to_lowercase();

// Step 2: Replace non-alphanumeric with hyphens
let with_hyphens = lower.chars()
    .map(|c| if c.is_alphanumeric() { c } else { '-' })
    .collect::<String>();
```

### Error Handling

**DO**:

```rust
// Use Result for fallible operations
pub fn load_config() -> Result<Config, RfdError> { ... }

// Provide context
let config = load_config()
    .map_err(|e| RfdError::ConfigError {
        message: format!("Failed to load config: {}", e),
    })?;

// Use ? operator for propagation
let file = File::open(path)?;
```

**DON'T**:

```rust
// Don't panic in library code
let value = map.get("key").unwrap();  // ❌

// Don't ignore errors
let _ = file.write_all(data);  // ❌

// Don't use generic error messages
return Err("error".into());  // ❌
```

## Pull Request Process

### Before Submitting

**Checklist**:

- [ ] Code compiles (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation updated (if applicable)
- [ ] Integration test added (for new commands/features)
- [ ] CHANGELOG.md updated (if applicable)

### PR Template

```markdown
## Description

Brief description of what this PR does.

## Motivation

Why is this change needed? What problem does it solve?

## Changes

- List of specific changes made
- Another change
- etc.

## Testing

How was this tested? What test cases were added?

## Related Issues

Fixes #123 Relates to #456
```

### Review Process

1. **Submit PR**: Create PR on GitHub
2. **CI checks**: Automated tests and linting
3. **Code review**: Maintainer reviews code
4. **Address feedback**: Make requested changes
5. **Approval**: Once approved, PR is merged

### For Non-Trivial Changes

For significant features or architectural changes:

1. **Create an RFC** first (see `docs/design/features/` for examples)
2. **Discuss approach** with maintainers
3. **Get consensus** before implementing
4. **Then follow** normal PR process

## For Junior Developers

### I'm New to Rust!

Welcome! Here are some learning resources:

- [The Rust Book](https://doc.rust-lang.org/book/) - Start here!
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### I'm New to CLI Development!

- Read our [ARCHITECTURE.md](ARCHITECTURE.md) - explains design decisions
- Start with simple tasks: documentation fixes, test additions
- Look for `good first issue` label on GitHub issues

### I'm New to Open Source!

That's great! Here's what to know:

1. **It's okay to ask questions** - Use GitHub Discussions or issues
2. **Start small** - Fix typos, add tests, improve docs
3. **Read existing code** - Best way to learn the patterns
4. **Don't be afraid of mistakes** - Code review is for learning!

### Glossary

- **RFD**: Request for Discussion - design document format
- **Frontmatter**: YAML metadata at top of file (between `---` delimiters)
- **State Machine**: System where objects have states and valid transitions
- **Idempotent**: Operation that can be repeated without changing the result
- **Atomic Write**: File write that either fully succeeds or fully fails
- **LTO**: Link-Time Optimization - compiler optimization across all code
- **Integration Test**: Test that verifies complete workflows
- **Unit Test**: Test that verifies individual functions

### Common Questions

**Q: Where should I start reading the code?** A: Start with `src/main.rs` (100
lines), then `src/commands/create.rs`.

**Q: How do I run just one test?** A: `cargo test test_name`

**Q: What if I break something?** A: That's what tests are for! Run `cargo test`
before committing.

**Q: How do I debug?** A: Add `dbg!(variable)` or use
`println!("{:?}", variable)`. Run with `cargo run`.

**Q: My PR failed CI. What do I do?** A: Check the error, run `cargo fmt` and
`cargo clippy`, fix issues, push again.

## Getting Help

- **GitHub Discussions**: For questions and discussions
- **GitHub Issues**: For bugs and feature requests
- **Code Comments**: Read inline comments for complex logic
- **Documentation**: See [ARCHITECTURE.md](ARCHITECTURE.md) and
  [README.md](README.md)

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to learn and build
something useful together!

## License

By contributing, you agree that your contributions will be licensed under the
project's MIT OR Apache-2.0 license.

---

**Thank you for contributing!** 🎉

Your time and effort make this project better for everyone.
