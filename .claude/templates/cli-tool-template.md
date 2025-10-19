# CLI Tool Template

## Tool Information

**Name**: {{tool_name}}
**Description**: {{description}}
**Version**: 0.1.0
**Type**: CLI Tool (Fast, Agent-Friendly)

## Design Principles

This CLI tool follows Tapestry's CLI tool principles:

- **Fast Startup**: < 10ms cold start
- **Agent-Friendly**: JSON output, idempotent operations, actionable errors
- **Simple Architecture**: No hexagonal architecture, flat structure
- **Non-Interactive**: All data via flags, no prompts
- **Three Output Modes**: pretty (human), json (agents), quiet (errors only)

## Architecture

```
cli/{{tool_name}}/
├── src/
│   ├── main.rs           # CLI entry point with clap
│   ├── commands/         # Command implementations
│   │   ├── mod.rs
│   │   ├── {{command1}}.rs
│   │   └── {{command2}}.rs
│   ├── output.rs         # Output formatting (pretty/json/quiet)
│   └── error.rs          # Error types and handling
├── tests/
│   ├── cli_tests.rs      # CLI integration tests
│   └── output_tests.rs   # Output format tests
├── Cargo.toml
└── README.md
```

## API Design

### Commands

```bash
# Command structure
{{tool_name}} <COMMAND> [OPTIONS] [ARGS]

# Global flags
--format, -f <FORMAT>     Output format: pretty, json, quiet [default: pretty]
--verbose, -v             Enable verbose output
--help, -h                Print help
--version, -V             Print version
```

### Command: {{command1}}

```bash
{{tool_name}} {{command1}} [OPTIONS]

Options:
  --{{option1}} <VALUE>    {{option1_description}}
  --{{option2}} <VALUE>    {{option2_description}}
```

**JSON Output**:
```json
{
  "{{output_field1}}": "{{value1}}",
  "{{output_field2}}": "{{value2}}"
}
```

**Error Output**:
```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable message",
  "details": {
    "{{detail_key}}": "{{detail_value}}"
  },
  "suggestion": {
    "command": "{{suggested_command}}",
    "description": "{{why_this_might_work}}"
  }
}
```

### Command: {{command2}}

```bash
{{tool_name}} {{command2}} <ARG> [OPTIONS]

Arguments:
  <{{arg_name}}>           {{arg_description}}

Options:
  --{{option1}} <VALUE>    {{option1_description}}
```

## Implementation Checklist

### Phase 1: Setup
- [ ] Create `cli/{{tool_name}}/` directory
- [ ] Initialize Cargo.toml with dependencies
- [ ] Create basic main.rs with clap

### Phase 2: Core Implementation
- [ ] Define CLI structure (Parser, Subcommand)
- [ ] Implement command handlers
- [ ] Add output formatting (pretty/json/quiet)
- [ ] Implement error types

### Phase 3: Testing
- [ ] Write CLI integration tests
- [ ] Test all output formats
- [ ] Test error scenarios
- [ ] Validate JSON schema

### Phase 4: Polish
- [ ] Add help text and examples
- [ ] Optimize for startup time
- [ ] Document all commands
- [ ] Binary size optimization

## Error Codes

| Code | Description | Exit Code |
|------|-------------|-----------|
| `SUCCESS` | Operation completed | 0 |
| `INVALID_INPUT` | Invalid arguments | 1 |
| `NOT_FOUND` | Resource not found | 2 |
| `{{ERROR_CODE_3}}` | {{description}} | 3 |

## Performance Targets

- **Startup time**: < 10ms (cold start)
- **Execution time**: < 100ms (P50), < 500ms (P99)
- **Binary size**: < 3MB (stripped with LTO)
- **Memory usage**: < 10MB peak

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive", "cargo"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"
colored = "2"  # For pretty output

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

## Usage Examples

### Example 1: Basic Usage (Pretty Output)

```bash
$ {{tool_name}} {{command1}} --{{option1}} value
✅ {{success_message}}
{{pretty_output}}
```

### Example 2: JSON Output (Agent-Friendly)

```bash
$ {{tool_name}} {{command1}} --{{option1}} value --format json
{"{{field}}":"{{value}}","status":"success"}
```

### Example 3: Quiet Mode (Errors Only)

```bash
$ {{tool_name}} {{command1}} --{{option1}} value --format quiet
# No output on success, only errors
```

### Example 4: Error Handling

```bash
$ {{tool_name}} {{command1}} --invalid
❌ Error: Invalid input
Suggestion: Try '{{tool_name}} {{command1}} --{{option1}} value'
```

## Testing Strategy

### CLI Integration Tests

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_{{command1}}_success() {
    let mut cmd = Command::cargo_bin("{{tool_name}}").unwrap();
    cmd.arg("{{command1}}")
        .arg("--{{option1}}")
        .arg("value")
        .assert()
        .success()
        .stdout(predicate::str::contains("{{expected_output}}"));
}

#[test]
fn test_json_output() {
    let mut cmd = Command::cargo_bin("{{tool_name}}").unwrap();
    cmd.arg("{{command1}}")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::is_json());
}
```

## Distribution

### Binary Builds

```bash
# Development build
cargo build --bin {{tool_name}}

# Release build (optimized)
cargo build --release --bin {{tool_name}}

# Stripped release (minimal size)
cargo build --release --bin {{tool_name}}
strip target/release/{{tool_name}}
```

### Installation

```bash
# Install locally
cargo install --path cli/{{tool_name}}

# Install from crates.io (future)
cargo install {{tool_name}}
```

## Maintenance

### When to Update

- New commands needed
- Output format changes
- Performance improvements
- Bug fixes
- Dependency updates

### Versioning

Follow semantic versioning:
- **Major**: Breaking CLI changes
- **Minor**: New commands/features
- **Patch**: Bug fixes, performance

---

**Template Version**: 1.0
**Last Updated**: {{date}}
**Maintainer**: Tapestry Team
