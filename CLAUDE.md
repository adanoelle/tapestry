# CLAUDE.md - Tapestry Quick Reference

> **Purpose**: This file provides Claude Code with immediate, actionable
> patterns and commands for working in the Tapestry codebase. For deep context
> and principles, see `.claude/`.

## 🎯 What is Tapestry?

Tapestry is a monolithic suite of MCP (Model Context Protocol) tools for
AI-assisted development, built with Rust using hexagonal architecture.

**Quick Context**:

- **Language**: Rust (async-first with Tokio)
- **Architecture**: Hexagonal (Ports & Adapters)
- **Deployment**: Monolithic (for now)
- **MCP Library**: rmcp
- **Key Goal**: Build tools that enhance AI-assisted development workflows

## 🚀 Quick Commands

```bash
# Create a new MCP tool
/create-mcp-tool "tool-name" "description"

# Write an RFC for a feature
/write-rfc "Feature Name" "Problem to solve"

# Check current sprint status
cat .claude/context/project-state.md

# See today's focus
cat .claude/sessions/current.md
```

## 📁 Directory Structure

```
tapestry/
├── src/
│   ├── domain/            # Pure business logic (no dependencies!)
│   ├── application/       # Use cases and ports (interfaces)
│   ├── infrastructure/    # External adapters (MCP, DB, APIs)
│   ├── tools/             # Individual MCP tools
│   │   └── {tool}/
│   │       ├── domain.rs  # Core logic
│   │       ├── port.rs    # Interface
│   │       └── adapter.rs # MCP implementation
│   └── registry/          # Tool discovery and management
├── docs/
│   ├── design/            # RFCs and design documents
│   └── VISION.md          # Project vision
├── .claude/               # AI collaboration context (see below)
└── CLAUDE.md              # This file
```

## 🏗️ Architecture Pattern

**Every tool follows this structure:**

```rust
// 1. Domain (src/tools/my_tool/domain.rs) - PURE LOGIC
pub struct MyToolService {
    // No external dependencies!
}

impl MyToolService {
    pub fn execute(&self, input: Input) -> Result<Output> {
        // Pure business logic only
    }
}

// 2. Port (src/tools/my_tool/port.rs) - INTERFACE
#[async_trait]
pub trait MyToolPort {
    async fn execute(&self, input: Input) -> Result<Output>;
}

// 3. Adapter (src/tools/my_tool/adapter.rs) - MCP IMPLEMENTATION
#[rmcp::tool(name = "my-tool", description = "...")]
impl Tool for MyToolAdapter {
    // MCP protocol implementation
}
```

**Remember**: Dependencies flow inward → Infrastructure depends on Application
depends on Domain

## ✅ Quick Checklist for New Code

### Before Writing

- [ ] Is there an RFC for this feature? Check `docs/design/features/`
- [ ] Have you read the conventions? See `.claude/context/team-conventions.md`
- [ ] Is your session context current? Update `.claude/sessions/current.md`

### While Writing

- [ ] Domain logic has ZERO external dependencies
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

- Tool execution: < 100ms P50, < 500ms P99
- Memory per tool: < 10MB
- Startup time: < 1 second

**Code Quality**:

- Test coverage: > 80% for domain logic
- Zero clippy warnings
- All public APIs documented

## 🤝 Working with Claude Code

### Best Practices

1. **Start with context**: Always ensure `.claude/sessions/current.md` is
   updated
2. **Use commands**: Leverage `/create-mcp-tool` and `/write-rfc` commands
3. **Reference patterns**: Point to specific pattern files when asking for
   implementation
4. **Verify understanding**: Ask Claude to explain the architecture before
   implementing

### Example Interaction

```
You: "Create a new MCP tool for code analysis"

Better: "Using /create-mcp-tool, create a 'code-analyzer' tool that
analyzes Rust code for complexity metrics. Follow our hexagonal
architecture pattern from .claude/context/architecture.md"
```

## 🔗 Quick Links

### Internal

- [Project Vision](docs/VISION.md)
- [Architecture Decision Records](.claude/knowledge/decisions/)
- [Team Conventions](.claude/context/team-conventions.md)
- [Current Sprint](.claude/context/project-state.md)

### External

- [MCP Specification](https://modelcontextprotocol.io/)
- [rmcp Documentation](https://docs.rs/rmcp)
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
