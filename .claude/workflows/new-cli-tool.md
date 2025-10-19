# Workflow: Create New CLI Tool

## Workflow Metadata

**Workflow ID**: new-cli-tool
**Version**: 1.0
**Purpose**: Systematic process for creating fast, agent-friendly CLI tools
**Duration**: 2-4 hours
**Agents Required**: Rust Expert (optional)

## Prerequisites

- [ ] Tool concept validated (fast operations vs stateful MCP)
- [ ] Commands defined
- [ ] JSON schema designed
- [ ] Performance targets identified

## Workflow Phases

### Phase 1: Design & API Contract (45 min)

**Lead**: Design reviewer or primary developer
**Duration**: 30-45 minutes

#### Step 1.1: Define CLI Structure

```yaml
tool_design:
  name: {TOOL_NAME}
  description: {DESCRIPTION}

  commands:
    - name: {command1}
      description: {What it does}
      args:
        - name: {arg1}
          type: {String|Int|Path}
          required: {true|false}
      options:
        - name: {opt1}
          type: {String}
          default: {value}

    - name: {command2}
      description: {What it does}
      args: []
      options: []

  output_formats:
    - pretty: {Human-readable}
    - json: {Agent-friendly}
    - quiet: {Errors only}
```

#### Step 1.2: Design JSON Schema

For each command, define JSON output:

```json
// Success schema
{
  "status": "success",
  "message": "Operation description",
  "data": {
    // Command-specific fields
  }
}

// Error schema
{
  "error": "ERROR_CODE",
  "message": "Human-readable error",
  "details": {},
  "suggestion": {
    "command": "suggested fix",
    "description": "why it might work"
  }
}
```

#### Step 1.3: Define Performance Targets

```yaml
performance:
  startup_time: < 10ms
  execution_p50: < 100ms
  execution_p99: < 500ms
  binary_size: < 3MB
  memory_usage: < 10MB
```

**Gate**: API contract must be clear before implementation

---

### Phase 2: Implementation (1-2 hours)

**Lead**: Rust expert or primary developer
**Duration**: 1-2 hours

#### Step 2.1: Scaffold Structure

```bash
# Use scaffolding script
./.claude/scripts/scaffold-cli-tool.sh "$TOOL_NAME" "$DESCRIPTION"

# Or manual:
TOOL_MODULE=$(echo "$TOOL_NAME" | tr '-' '_')
mkdir -p cli/$TOOL_MODULE/src/commands
mkdir -p cli/$TOOL_MODULE/tests
```

**Output**:
- Directory structure created
- Cargo.toml with optimizations
- Basic main.rs scaffold

#### Step 2.2: Implement Core CLI

```yaml
agent: Rust Expert
input:
  - API contract from Phase 1
  - Command specifications
output:
  - cli/{tool}/src/main.rs
  - Complete clap structure
  - Command routing
artifacts:
  - Compilable main.rs
```

**Rust Code Pattern**:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tool-name")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "pretty", global = true)]
    format: String,
}

#[derive(Subcommand)]
enum Commands {
    Command1 { /* fields */ },
    Command2 { /* fields */ },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Command1 { .. } => { /* handler */ }
        Commands::Command2 { .. } => { /* handler */ }
    }
    Ok(())
}
```

#### Step 2.3: Implement Output Formatting

```yaml
input:
  - Output format specifications
output:
  - cli/{tool}/src/output.rs
  - Pretty, JSON, Quiet implementations
```

**Key Functions**:
- `print_success(format, message, data)`
- `print_error(format, code, message, suggestion)`

#### Step 2.4: Implement Commands

For each command:

```yaml
input:
  - Command specification
  - Business logic requirements
output:
  - cli/{tool}/src/commands/{command}.rs
  - Command implementation
  - Error handling
```

**Pattern**:
```rust
pub fn execute(args: Args, format: &str) -> Result<()> {
    // 1. Validate input
    // 2. Execute logic
    // 3. Format output
    output::print_success(format, "Done", Some(data));
    Ok(())
}
```

#### Step 2.5: Implement Error Handling

```yaml
output:
  - cli/{tool}/src/error.rs
  - Typed errors with codes
  - Exit code mapping
```

---

### Phase 3: Testing (30-45 min)

**Lead**: Test writer or primary developer
**Duration**: 30-45 minutes

#### Step 3.1: Create CLI Integration Tests

```yaml
agent: Test Writer
input:
  - Command specifications
  - Expected outputs
output:
  - cli/{tool}/tests/cli_tests.rs
  - Tests for all commands
  - Tests for all output formats
```

**Test Pattern**:
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_command_json_output() {
    Command::cargo_bin("tool-name").unwrap()
        .arg("command")
        .arg("--format").arg("json")
        .assert()
        .success()
        .stdout(predicate::str::is_json());
}
```

#### Step 3.2: Test All Output Formats

For each command, test:
- [ ] Pretty output (human-readable)
- [ ] JSON output (valid JSON)
- [ ] Quiet output (empty on success)
- [ ] Error output (proper codes)

#### Step 3.3: Test Error Scenarios

```yaml
test_scenarios:
  - Invalid input
  - Missing files
  - Permission errors
  - Edge cases
```

#### Step 3.4: Run Test Suite

```bash
# Run all tests
cargo test -p $TOOL_MODULE

# Check output
cargo test -p $TOOL_MODULE -- --nocapture
```

**Gate**: All tests must pass before optimization

---

### Phase 4: Optimization & Polish (30-45 min)

**Lead**: Rust expert
**Duration**: 30-45 minutes

#### Step 4.1: Binary Size Optimization

```bash
# Build release
cargo build --release --bin $TOOL_NAME

# Check size
ls -lh target/release/$TOOL_NAME

# Analyze bloat
cargo bloat --release --bin $TOOL_NAME -n 20
```

**Optimizations**:
- Cargo.toml already configured for size
- Remove unused dependencies
- Use feature flags to disable unused features
- Consider `cargo-strip` if needed

**Target**: < 3MB stripped binary

#### Step 4.2: Startup Time Measurement

```bash
# Measure startup
hyperfine --warmup 5 "./target/release/$TOOL_NAME --version"

# Or simple timing
time ./target/release/$TOOL_NAME --version
```

**Target**: < 10ms cold start

**If too slow**:
- Profile with `cargo flamegraph`
- Check for heavy initialization
- Lazy-load expensive operations
- Reduce dependency count

#### Step 4.3: Help Text & Documentation

```yaml
polish:
  - Add examples to --help
  - Improve error messages
  - Add suggestions for common errors
  - Update README with examples
```

---

### Phase 5: Integration & Documentation (20-30 min)

**Lead**: Documenter
**Duration**: 20-30 minutes

#### Step 5.1: Update Project Documentation

```bash
# Update CLI tools README
cat >> cli/README.md << EOF

### $TOOL_NAME

**Description**: $DESCRIPTION
**Binary**: \`$TOOL_NAME\`
**Startup**: < 10ms
**Status**: Active

\`\`\`bash
# Example
$TOOL_NAME command --option value --format json
\`\`\`
EOF
```

#### Step 5.2: Create Usage Examples

```bash
cat > cli/$TOOL_MODULE/EXAMPLES.md << 'EOF'
# Usage Examples for {TOOL_NAME}

## Basic Usage

\`\`\`bash
# Example 1
{command}

# Example 2
{command}
\`\`\`

## Advanced Usage

\`\`\`bash
# Example 3
{command}
\`\`\`

## Agent Invocation (via Skills)

\`\`\`bash
# JSON output for parsing
{command} --format json
\`\`\`
EOF
```

#### Step 5.3: Update Project Tracking

```bash
# Update project state
echo "- [x] Created $TOOL_NAME CLI tool ($(date +%Y-%m-%d))" >> .claude/context/project-state.md

# Update CHANGELOG
echo "### Added
- **CLI Tool**: $TOOL_NAME - $DESCRIPTION" >> CHANGELOG.md
```

---

## Workflow Summary

```
Phase 1: Design & API Contract (45 min)
  ↓
Phase 2: Implementation (1-2 hours)
  ↓
Phase 3: Testing (30-45 min)
  ↓
Phase 4: Optimization & Polish (30-45 min)
  ↓
Phase 5: Integration & Documentation (20-30 min)
  ↓
Complete! (Total: 2-4 hours)
```

---

## Deliverables

### Required Files

- [ ] `cli/{tool}/Cargo.toml` - With size optimizations
- [ ] `cli/{tool}/src/main.rs` - CLI entry point
- [ ] `cli/{tool}/src/commands/*.rs` - Command implementations
- [ ] `cli/{tool}/src/output.rs` - Output formatting
- [ ] `cli/{tool}/src/error.rs` - Error types
- [ ] `cli/{tool}/tests/cli_tests.rs` - Integration tests
- [ ] `cli/{tool}/README.md` - Usage documentation

### Optional Files

- [ ] `cli/{tool}/EXAMPLES.md` - Usage examples
- [ ] `cli/{tool}/PERFORMANCE.md` - Performance benchmarks

---

## Quality Checklist

### Functionality

- [ ] All commands implemented
- [ ] All output formats working
- [ ] Error handling comprehensive
- [ ] Help text clear and useful

### Performance

- [ ] Binary size < 3MB
- [ ] Startup time < 10ms
- [ ] Execution time acceptable
- [ ] Memory usage < 10MB

### Agent-Friendliness

- [ ] JSON output valid and consistent
- [ ] Error codes documented
- [ ] Idempotent operations
- [ ] Non-interactive (no prompts)
- [ ] Actionable error messages

### Testing

- [ ] All commands tested
- [ ] All output formats tested
- [ ] Error scenarios tested
- [ ] JSON schema validated

### Documentation

- [ ] README with examples
- [ ] Help text comprehensive
- [ ] Error messages actionable
- [ ] Project docs updated

---

## Examples

### Example 1: RFD CLI (Simple CRUD)

**Phase 1**: Define create, list, show, status commands
**Phase 2**: Implement with file I/O and YAML parsing
**Phase 3**: Test JSON output for agent invocation
**Phase 4**: Optimize to < 10ms startup
**Phase 5**: Document in skills/rfd-manager/

**Total Time**: ~2.5 hours

### Example 2: Code Analyzer (Complex Logic)

**Phase 1**: Define check, report, fix commands
**Phase 2**: Implement AST parsing and analysis
**Phase 3**: Test accuracy and output formats
**Phase 4**: Optimize for large codebases
**Phase 5**: Create comprehensive examples

**Total Time**: ~4 hours

---

## Troubleshooting

### Binary Size Too Large

**Symptoms**: > 5MB binary

**Solutions**:
- Enable LTO in Cargo.toml
- Use `opt-level = "z"`
- Strip symbols
- Analyze with `cargo bloat`
- Remove unused dependencies

### Startup Time Too Slow

**Symptoms**: > 20ms cold start

**Solutions**:
- Profile with flamegraph
- Lazy-load heavy operations
- Reduce dependency initialization
- Consider static linking
- Check for regex compilation

### JSON Output Invalid

**Symptoms**: Can't parse JSON

**Solutions**:
- Test with `jq` or JSON validator
- Use `serde_json::to_string` (not manual formatting)
- Handle all error cases
- Validate schema in tests

---

## Automation

For faster CLI tool creation:

```bash
./.claude/scripts/scaffold-cli-tool.sh "$TOOL_NAME" "$DESCRIPTION"
```

This creates:
- Complete directory structure
- Cargo.toml with optimizations
- Scaffold code for all modules
- Basic tests

---

## Next Steps After Tool Creation

1. **Create Skill**: Build a Skill that uses this CLI tool
2. **Test Integration**: Invoke from Skills
3. **Measure Performance**: Validate < 10ms startup
4. **Gather Feedback**: Use in real workflows
5. **Iterate**: Improve based on usage

---

**Remember**: CLI tools should be simple and fast. No complex architecture, just solid execution!
