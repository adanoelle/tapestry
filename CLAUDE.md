# CLAUDE.md - Tapestry Quick Reference

> **Purpose**: This file provides Claude Code with immediate, actionable
> patterns and commands for working in the Tapestry codebase. For deep context
> and principles, see `.claude/`.

## 🎯 What is Tapestry?

Tapestry is a suite of AI-native development tools using a **hybrid architecture**:
lightweight CLI tools for Skills, deep-integration MCP tools for complex operations,
and Skills that orchestrate both.

**Quick Context**:

- **Language**: Rust for both CLI and MCP tools
- **Architecture**: Three layers (Skills → CLI + MCP)
- **Approach**: Skills-first (validate before heavy MCP investment)
- **Current Focus**: RFD CLI tool + rfd-manager Skill
- **Key Goal**: Right tool for each job in AI-assisted workflows

## 🚀 Quick Commands

```bash
# Build and run RFD CLI
cargo run --bin rfd -- --help

# Run workspace checks
cargo check --workspace
cargo test --workspace

# Create RFD (once implemented)
rfd create --title "Feature Name" --author "Name <email>"

# Check current sprint status
cat .claude/context/project-state.md

# See project vision
cat docs/VISION.md
```

## 📁 Directory Structure

```
tapestry/
├── cli/                   # Standalone CLI tools (for Skills)
│   └── rfd/              # RFD document manager (current focus)
│       ├── src/
│       │   └── main.rs   # CLI entry point
│       ├── Cargo.toml
│       └── README.md
├── mcp/                   # MCP protocol tools (deep integration)
│   └── git_workflow/     # Git workflow automation (paused)
│       ├── src/
│       │   ├── domain.rs # Pure logic
│       │   ├── port.rs   # Interface
│       │   └── adapter.rs # MCP impl
│       └── Cargo.toml
├── skills/                # Skills that orchestrate tools
│   └── rfd-manager/      # Skill for RFD workflow (planned)
│       └── SKILL.md
├── docs/
│   ├── design/
│   │   └── features/     # RFC documents
│   └── VISION.md         # Project vision
├── .claude/              # AI collaboration context
├── Cargo.toml            # Workspace configuration
└── CLAUDE.md             # This file
```

## 🏗️ Architecture Patterns

### Three-Layer Hybrid Architecture

**When to use each:**

| Need | Use CLI Tool | Use MCP Tool | Use Skill |
|------|--------------|--------------|-----------|
| Fast startup (< 10ms) | ✅ | ❌ | ✅ |
| Agent invokes directly | ✅ | ✅ | ✅ |
| Stateful/complex ops | ❌ | ✅ | - |
| Simple file CRUD | ✅ | ❌ | - |
| Orchestrate workflows | ❌ | ❌ | ✅ |

### CLI Tool Pattern (cli/*)

```rust
// src/main.rs - Simple, fast, agent-friendly
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "pretty")]
    format: String,  // pretty, json, quiet
}

#[derive(Subcommand)]
enum Commands {
    Create { /* args */ },
    List { /* args */ },
    Show { id: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Execute command
    // Output JSON when format == "json"
    Ok(())
}
```

### MCP Tool Pattern (mcp/*)

```rust
// Hexagonal architecture for complex tools
// 1. Domain (domain.rs) - PURE LOGIC
pub struct MyToolService {
    // No external dependencies!
}

// 2. Port (port.rs) - INTERFACE
#[async_trait]
pub trait MyToolPort {
    async fn execute(&self, input: Input) -> Result<Output>;
}

// 3. Adapter (adapter.rs) - MCP IMPLEMENTATION
#[rmcp::tool(name = "my-tool")]
impl Tool for MyToolAdapter {
    // MCP protocol implementation
}
```

**Remember**:
- CLI tools: Simple, fast, stateless
- MCP tools: Complex, stateful, deep integration
- Skills: Orchestrate both types

## ✅ Quick Checklist for New Code

### Before Writing

- [ ] Is there an RFD/RFC for this feature? Check `docs/design/features/`
- [ ] Have you read the conventions? See `.claude/context/team-conventions.md`
- [ ] Decided on tool type? CLI (fast, simple) vs MCP (complex, stateful)?
- [ ] Is your session context current? Update `.claude/sessions/current.md`

### While Writing

- [ ] **CLI tools**: JSON output mode, idempotent operations, actionable errors
- [ ] **MCP tools**: Domain logic has ZERO external dependencies
- [ ] Using `Result<T, E>` for all fallible operations (no `unwrap()`!)
- [ ] Errors are actionable (tell the user/agent how to fix)
- [ ] Following naming conventions (snake_case functions, PascalCase types)

### After Writing

- [ ] Tests written (unit for domain, integration for adapters)
- [ ] Documentation updated (in same PR!)
- [ ] Run: `cargo fmt && cargo clippy && cargo test`
- [ ] Updated CHANGELOG.md

## 🔧 Common Patterns

### Error Handling

```rust
// Domain errors (thiserror)
#[derive(Error, Debug)]
pub enum ToolError {
    #[error("Configuration missing: Set {var} environment variable")]
    ConfigMissing { var: String },
}

// Application errors (anyhow)
use anyhow::{Context, Result};
let config = load_config()
    .context("Failed to load configuration")?;
```

### Tool Creation

```rust
// Always follow this pattern for new tools
pub mod my_tool {
    pub mod domain;    // Core logic
    pub mod port;      // Interface
    pub mod adapter;   // MCP impl

    pub fn create_tool() -> MyToolAdapter {
        MyToolAdapter::new()
    }
}
```

### Testing

```rust
#[cfg(test)]
mod tests {
    // Test names describe behavior
    #[test]
    fn should_return_error_when_input_invalid() {
        // Arrange
        // Act
        // Assert
    }
}
```

## 📍 Navigation Guide

### Where to Find Things

| Looking for...       | Location                              | Command                                   |
| -------------------- | ------------------------------------- | ----------------------------------------- |
| Core principles      | `.claude/instructions.md`             | `cat .claude/instructions.md`             |
| Architecture details | `.claude/context/architecture.md`     | `cat .claude/context/architecture.md`     |
| Coding standards     | `.claude/context/team-conventions.md` | `cat .claude/context/team-conventions.md` |
| Current sprint       | `.claude/context/project-state.md`    | `cat .claude/context/project-state.md`    |
| Tech decisions       | `.claude/context/tech-decisions.md`   | `cat .claude/context/tech-decisions.md`   |
| RFC template         | `.claude/templates/rfc-template.md`   | Use `/write-rfc` command                  |
| Tool template        | `.claude/commands/create-mcp-tool.md` | Use `/create-mcp-tool` command            |

### When to Reference What

**Use THIS file (CLAUDE.md) for**:

- Quick reminders of patterns
- Common code snippets
- Directory navigation
- Quick commands

**Use `.claude/instructions.md` for**:

- Core principles that never change
- S-tier company practices
- Security requirements
- Overall philosophy

**Use `.claude/context/` for**:

- Current project state
- Architecture decisions
- Team conventions
- Technical choices

**Use `.claude/sessions/current.md` for**:

- Today's specific task
- Current blockers
- Work in progress

## 🎭 Working with Other CLAUDE.md Files

### Hierarchy of CLAUDE.md Files

```
CLAUDE.md (root - this file)         # Global patterns, navigation
├── src/tools/CLAUDE.md              # Tool-specific patterns
├── src/domain/CLAUDE.md             # Domain modeling patterns
├── src/infrastructure/CLAUDE.md     # Infrastructure patterns
└── tests/CLAUDE.md                  # Testing patterns
```

### Rules for CLAUDE.md Files

1. **Root CLAUDE.md** (this file): Quick reference, navigation, common patterns
2. **Module CLAUDE.md**: Module-specific patterns that override or extend root
3. **Always check both**: Module-specific first, then root for general patterns

### Creating Module-Specific CLAUDE.md

```markdown
# CLAUDE.md - [Module Name] Patterns

> **Context**: This file extends the root CLAUDE.md with patterns specific to
> [module]. **Parent**: [/CLAUDE.md](/CLAUDE.md)

## Module-Specific Patterns

[Patterns that are unique to this module]

## Overrides

[Any patterns from root that work differently here]
```

## 🚨 Critical Rules

1. **NEVER put secrets in code** - Use environment variables
2. **NEVER use `unwrap()` in production** - Always handle errors
3. **NEVER let domain depend on infrastructure** - Dependencies flow inward
4. **NEVER skip tests** - Test domain logic thoroughly
5. **NEVER merge without review** - Even AI-generated code needs human review

## 🔄 Development Workflow

```mermaid
graph LR
    A[Read Task] --> B[Check/Write RFC]
    B --> C[Update Session]
    C --> D[Write Code]
    D --> E[Write Tests]
    E --> F[Update Docs]
    F --> G[Review]
    G --> H[Merge]
```

## 📊 Quick Metrics

**Performance Targets**:

**CLI Tools**:
- Startup time: < 10ms (cold start)
- Execution: < 50ms for simple operations
- Binary size: < 3MB (stripped with LTO)
- Memory: < 10MB peak

**MCP Tools**:
- Tool execution: < 100ms P50, < 500ms P99
- Memory per tool: < 10MB
- Startup time: < 1 second

**Code Quality**:

- Test coverage: > 80% for domain/business logic
- Zero clippy warnings
- All public APIs documented
- Skills documented with examples

## 🤝 Working with Claude Code

### Best Practices

1. **Start with context**: Always ensure `.claude/sessions/current.md` is
   updated
2. **Use commands**: Leverage `/create-mcp-tool` and `/write-rfc` commands
3. **Reference patterns**: Point to specific pattern files when asking for
   implementation
4. **Verify understanding**: Ask Claude to explain the architecture before
   implementing

### Example Interactions

```
# Building a new tool
You: "Create a new tool for managing documentation"

Better: "Build a CLI tool for managing RFD documents. Should support
create, list, show, and status commands with JSON output. Follow the
pattern from cli/rfd and see docs/VISION.md for our hybrid approach."

# Understanding architecture
You: "Why do we have both CLI and MCP tools?"

Better: "Explain the three-layer architecture from docs/VISION.md.
When should I use CLI vs MCP? What are Skills for?"
```

## 🔗 Quick Links

### Internal

- [Project Vision](docs/VISION.md)
- [Architecture Decision Records](.claude/knowledge/decisions/)
- [Team Conventions](.claude/context/team-conventions.md)
- [Current Sprint](.claude/context/project-state.md)

### External

- [Anthropic Skills Documentation](https://docs.claude.com/en/docs/claude-code/skills)
- [MCP Specification](https://modelcontextprotocol.io/)
- [rmcp Documentation](https://docs.rs/rmcp)
- [clap Documentation](https://docs.rs/clap)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 📝 Remember

> "Start with the simplest solution and iterate" - Anthropic
>
> "Once code is written using our API, it should never need to change" - Stripe
>
> "Documentation lives with code, not separate from it" - Google

---

**Need more detail?** Check `.claude/` directory  
**Need to update patterns?** Submit an RFC  
**Found a bug in this guide?** Update it and submit a PR!
