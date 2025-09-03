# Tapestry Team Conventions

## Rust Code Style

### General Principles

- Clarity over cleverness
- Explicit over implicit
- Safe over unsafe (never use `unsafe` without RFC)
- Async by default for I/O operations

### Naming Conventions

```rust
// Types: PascalCase
pub struct ToolRegistry { }
pub enum ToolError { }
pub trait ToolPort { }

// Functions and methods: snake_case
pub fn create_tool() -> Tool { }
pub async fn execute_tool_async() -> Result<()> { }

// Constants: SCREAMING_SNAKE_CASE
pub const MAX_TOOL_NAME_LENGTH: usize = 64;
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

// Module files: lowercase with underscores
// src/tool_registry.rs
// src/mcp_adapter.rs

// Type parameters: Single capital letter or PascalCase
fn process<T: ToolPort>(tool: T) { }
fn transform<Input, Output>(data: Input) -> Output { }
```

### Error Handling

```rust
// Always use Result for fallible operations
pub fn risky_operation() -> Result<Value, Error> { }

// Custom error types with thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolError {
    #[error("Tool not found: {name}")]
    NotFound { name: String },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// Provide context with anyhow in application layer
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?
        .parse()
        .context("Failed to parse config")
}

// Never panic in production code
// Replace this:
let value = map.get("key").unwrap();

// With this:
let value = map.get("key")
    .ok_or_else(|| Error::MissingKey("key".to_string()))?;
```

### Async Patterns

```rust
// Use async-trait for trait definitions
use async_trait::async_trait;

#[async_trait]
pub trait ToolPort {
    async fn execute(&self, input: Input) -> Result<Output>;
}

// Prefer tokio::spawn for concurrent operations
use tokio::task;

let handles: Vec<_> = tools
    .into_iter()
    .map(|tool| task::spawn(async move {
        tool.execute().await
    }))
    .collect();

// Use tokio::select! for racing operations
tokio::select! {
    result = operation() => handle_result(result),
    _ = tokio::time::sleep(timeout) => handle_timeout(),
}
```

### Testing Conventions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test names describe behavior, not implementation
    #[test]
    fn should_return_error_when_tool_not_found() { }

    // Use given_when_then or arrange_act_assert
    #[test]
    fn test_tool_execution() {
        // Arrange (Given)
        let tool = create_test_tool();
        let input = TestInput::default();

        // Act (When)
        let result = tool.execute(input);

        // Assert (Then)
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, Status::Success);
    }

    // Use #[tokio::test] for async tests
    #[tokio::test]
    async fn should_handle_concurrent_requests() {
        // Test implementation
    }

    // Property-based tests for complex logic
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn doesnt_crash(input: String) {
            let _ = parse_tool_name(&input);
        }
    }
}
```

## Documentation Standards

### Code Documentation

````rust
/// Brief one-line description of the item.
///
/// More detailed explanation if needed. This can span
/// multiple paragraphs.
///
/// # Arguments
///
/// * `input` - Description of the input parameter
///
/// # Returns
///
/// Description of the return value
///
/// # Errors
///
/// Returns `ToolError::NotFound` if the tool doesn't exist
///
/// # Examples
///
/// ```
/// use tapestry::Tool;
///
/// let tool = Tool::new("example");
/// let result = tool.execute(input)?;
/// ```
///
/// # Panics
///
/// Panics if the runtime is not initialized (only mention if applicable)
pub fn execute_tool(input: Input) -> Result<Output> {
    // Implementation
}

// Use //! for module-level documentation
//! This module handles tool registration and discovery.
//!
//! The tool registry maintains a catalog of all available
//! MCP tools and their metadata.

// Use // for inline comments explaining WHY, not WHAT
// We use a BTreeMap here instead of HashMap because we need
// consistent iteration order for reproducible tool discovery
let tools: BTreeMap<String, Tool> = BTreeMap::new();
````

### Commit Messages

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semicolons, etc
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding missing tests
- `chore`: Changes to build process or auxiliary tools

Example:

```
feat(tools): add code review MCP tool

Implements a new MCP tool for automated code review using
Claude's analysis capabilities. The tool supports Rust,
Python, and TypeScript.

Closes #123
RFC: docs/design/features/code-review-tool.md
```

## Project Structure

### File Organization

```
src/
├── domain/              # Core business logic (no external deps)
│   ├── mod.rs          # Public API exports
│   ├── entities.rs     # Domain entities
│   └── services.rs     # Domain services
│
├── application/         # Use cases and orchestration
│   ├── mod.rs
│   ├── ports/          # Trait definitions (interfaces)
│   └── use_cases/      # Application services
│
├── infrastructure/      # External concerns
│   ├── mod.rs
│   └── adapters/       # Implementations of ports
│
└── tools/              # Individual MCP tools
    ├── mod.rs
    └── tool_name/      # Each tool in its own module
        ├── mod.rs
        ├── domain.rs
        ├── port.rs
        └── adapter.rs
```

### Import Order

```rust
// 1. Standard library
use std::collections::HashMap;
use std::io;

// 2. External crates
use anyhow::Result;
use tokio::task;
use tracing::info;

// 3. Internal crates
use crate::domain::Tool;
use crate::application::ToolPort;

// 4. Super and self
use super::ToolRegistry;
use self::internal::Helper;
```

## Git Workflow

### Branch Naming

- `feat/tool-name` - New tool implementation
- `fix/issue-description` - Bug fixes
- `docs/what-documented` - Documentation updates
- `refactor/what-refactored` - Code refactoring

### Pull Request Process

1. Create RFC for non-trivial changes
2. Implement with tests
3. Update documentation
4. Run full test suite
5. Request review
6. Address feedback
7. Squash and merge

### PR Description Template

```markdown
## Summary

Brief description of changes

## Motivation

Why these changes are needed

## Changes

- Bullet points of specific changes

## Testing

How the changes were tested

## Documentation

- [ ] Updated relevant docs
- [ ] Added inline documentation
- [ ] Updated CHANGELOG.md

## Related

- RFC: #123
- Closes: #456
```

## Performance Guidelines

### Benchmarking

```rust
#[bench]
fn bench_tool_execution(b: &mut Bencher) {
    let tool = create_tool();
    b.iter(|| {
        tool.execute(black_box(input))
    });
}
```

### Optimization Rules

1. Measure first, optimize second
2. Document why optimization was needed
3. Keep non-optimized version in comments if complex
4. Add benchmarks proving improvement

## Security Practices

### Input Validation

```rust
// Always validate external input
pub fn process_user_input(input: &str) -> Result<ProcessedInput> {
    // Validate length
    if input.len() > MAX_INPUT_LENGTH {
        return Err(Error::InputTooLong);
    }

    // Validate content
    if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::InvalidCharacters);
    }

    // Process validated input
    Ok(ProcessedInput::from(input))
}
```

### Secret Handling

- Never log secrets
- Use environment variables for configuration
- Use dedicated secret management in production
- Clear sensitive data from memory when done

## Review Checklist

Before submitting PR, ensure:

- [ ] Tests pass (`cargo test`)
- [ ] Lints pass (`cargo clippy`)
- [ ] Formatted (`cargo fmt`)
- [ ] Documentation updated
- [ ] No unwrap() in production code
- [ ] Errors are actionable
- [ ] Public APIs have examples
- [ ] CHANGELOG.md updated
- [ ] Performance impact considered
- [ ] Security implications reviewed

---

_These conventions are living guidelines. Propose changes via RFC._
