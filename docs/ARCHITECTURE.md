# Tapestry Architecture

This document provides a comprehensive overview of Tapestry's architecture,
design decisions, and technical implementation.

## Table of Contents

- [Overview](#overview)
- [Hybrid Architecture](#hybrid-architecture)
- [Tool Patterns](#tool-patterns)
- [Performance Targets](#performance-targets)
- [Security](#security)
- [Technology Stack](#technology-stack)
- [Design Principles](#design-principles)

## Overview

Tapestry is built on a **hybrid architecture** that combines the strengths of
lightweight CLI tools and deep-integration MCP tools, orchestrated by Claude
Code Skills.

### Core Philosophy

1. **Right Tool for the Job**: CLI for speed, MCP for depth, Skills for
   orchestration
2. **AI-First Design**: All tools are optimized for both human and AI use
3. **Production Quality**: S-tier engineering practices from day one
4. **Progressive Enhancement**: Start simple (CLI), add complexity as needed
   (MCP)

## Hybrid Architecture

### Three-Layer Design

```
┌─────────────────────────────────────────┐
│        Skills Layer (Claude Code)        │
│    Orchestration & Multi-step Workflows │
└─────────────────┬───────────────────────┘
                  │
         ┌────────┴────────┐
         │                 │
         ▼                 ▼
┌──────────────┐   ┌──────────────┐
│  CLI Tools   │   │  MCP Tools   │
│  Fast & Light│   │  Deep & Rich │
└──────────────┘   └──────────────┘
```

### CLI Tools (`cli/*`)

**Purpose**: Fast, stateless operations for simple workflows

**Characteristics**:

- ⚡ **Fast Startup**: < 10ms cold start
- 📦 **Small Binaries**: < 3MB (stripped with LTO)
- 💾 **Low Memory**: < 10MB peak usage
- 🔄 **Stateless**: No persistent connections
- 🤖 **Agent-Friendly**: JSON output, idempotent operations
- 📁 **File-Centric**: Direct filesystem operations

**When to Use**:

- Simple CRUD operations
- File generation and validation
- Quick lookups and queries
- Batch processing
- Scripts and automation

**Example**: RFD CLI - manages RFD documents through simple file operations

### MCP Tools (`mcp/*`)

**Purpose**: Complex, stateful operations requiring deep integration

**Characteristics**:

- 🔌 **Deep Integration**: Direct IDE/editor connections
- 💾 **Stateful**: Maintain context across operations
- 🔄 **Complex Operations**: Multi-step workflows with rollback
- 📡 **Real-time**: Streaming updates and notifications
- 🌐 **Network-Aware**: Git, APIs, databases
- ⏱️ **Longer-Running**: Designed for persistent sessions

**When to Use**:

- Git workflows with conflict resolution
- Multi-file refactoring
- Database operations
- API integrations
- Real-time collaboration

**Example**: Git Context Tool (planned) - provides rich git history and context

### Skills (`.claude/skills/`)

**Purpose**: Orchestrate CLI and MCP tools for complex workflows

**Characteristics**:

- 🎯 **Workflow Orchestration**: Combine multiple tools
- 🧠 **Context Management**: Maintain state across tool calls
- 🔀 **Decision Making**: Choose appropriate tools for tasks
- 📋 **Task Decomposition**: Break complex tasks into steps
- 🔁 **Error Handling**: Retry and fallback logic

**When to Use**:

- Multi-tool workflows (create RFD → generate docs → open PR)
- Complex decision trees
- Context-dependent operations
- Learning and adaptation

**Example**: rfd-manager (planned) - manages entire RFD lifecycle

### Decision Matrix

| Need                  | CLI | MCP | Skill |
| --------------------- | --- | --- | ----- |
| Fast startup (< 10ms) | ✅  | ❌  | ✅    |
| File CRUD             | ✅  | ❌  | -     |
| Git operations        | ❌  | ✅  | -     |
| Multi-step workflows  | ❌  | ❌  | ✅    |
| Stateful operations   | ❌  | ✅  | ✅    |
| Batch processing      | ✅  | ❌  | -     |
| Real-time updates     | ❌  | ✅  | -     |

## Tool Patterns

### Hexagonal Architecture (Ports & Adapters)

All tools follow hexagonal architecture for testability and maintainability:

```
┌───────────────────────────────────────┐
│         Adapter Layer                 │
│  (CLI/MCP/HTTP - Infrastructure)      │
└─────────────┬─────────────────────────┘
              │
┌─────────────▼─────────────────────────┐
│         Port Layer                    │
│  (Trait definitions - Interface)      │
└─────────────┬─────────────────────────┘
              │
┌─────────────▼─────────────────────────┐
│         Domain Layer                  │
│  (Pure business logic - Core)         │
└───────────────────────────────────────┘
```

### Layer Responsibilities

#### Domain Layer (`domain.rs`)

**Pure business logic with zero external dependencies**

```rust
// ✅ Good: Pure domain logic
pub struct RfdService {
    // No external dependencies!
}

impl RfdService {
    pub fn create_rfd(&self, title: String, author: String) -> Result<Rfd> {
        // Validation and business rules
        if title.is_empty() {
            return Err(RfdError::InvalidTitle);
        }

        Ok(Rfd {
            number: self.next_number(),
            title,
            author,
            status: RfdStatus::Ideation,
        })
    }
}

// ❌ Bad: Domain depending on infrastructure
pub struct RfdService {
    fs: FileSystem,  // Infrastructure leak!
}
```

**Key Rules**:

- No I/O operations (filesystem, network, database)
- No external crates except core utilities (serde, thiserror)
- 100% unit testable
- No timestamps - use dependency injection
- All errors are domain errors

#### Port Layer (`port.rs`)

**Interface definitions (traits) for adapters**

```rust
#[async_trait]
pub trait RfdRepository {
    async fn save(&self, rfd: &Rfd) -> Result<()>;
    async fn load(&self, number: u32) -> Result<Rfd>;
    async fn list(&self) -> Result<Vec<RfdMetadata>>;
}

#[async_trait]
pub trait TemplateEngine {
    async fn render(&self, template: &str, context: &Context) -> Result<String>;
}
```

**Key Rules**:

- Define what, not how
- Use async when appropriate
- Return domain types
- No implementation details

#### Adapter Layer (`adapter.rs` / `cli.rs` / `mcp.rs`)

**Infrastructure implementation of ports**

```rust
pub struct FileSystemRfdRepository {
    root_dir: PathBuf,
}

#[async_trait]
impl RfdRepository for FileSystemRfdRepository {
    async fn save(&self, rfd: &Rfd) -> Result<()> {
        let path = self.root_dir.join(format!("{:04}.md", rfd.number));
        tokio::fs::write(path, rfd.to_markdown()).await?;
        Ok(())
    }
}
```

**Key Rules**:

- Implement port traits
- Handle all I/O
- Convert between domain and external types
- Error handling and retries

### Dependency Flow

```
Infrastructure → Application → Domain
(Depends on)      (Uses)        (Pure)
```

- **Dependencies flow inward**: Infrastructure depends on Application depends on
  Domain
- **Domain has ZERO outward dependencies**: Can't import infrastructure code
- **Testability**: Domain is 100% unit testable, adapters use integration tests

## Performance Targets

### CLI Tools

| Metric                     | Target | RFD CLI (Actual) |
| -------------------------- | ------ | ---------------- |
| **Startup time**           | < 10ms | ~1ms ✅          |
| **Binary size (stripped)** | < 3MB  | 2.4MB ✅         |
| **Memory (peak)**          | < 10MB | ~5MB ✅          |
| **Simple operation**       | < 50ms | < 20ms ✅        |
| **Build time (release)**   | < 60s  | ~30s ✅          |

### MCP Tools (Targets)

| Metric                   | Target  |
| ------------------------ | ------- |
| **Tool execution (P50)** | < 100ms |
| **Tool execution (P99)** | < 500ms |
| **Memory per tool**      | < 10MB  |
| **Startup time**         | < 1s    |
| **Concurrent tools**     | 1000+   |

### Optimization Techniques

#### Binary Size

```toml
[profile.release]
# Link-time optimization
lto = true

# Single codegen unit for max optimization
codegen-units = 1

# Optimize for size
opt-level = "z"

# Strip debug symbols
strip = true

# Smaller panic handling
panic = "abort"
```

**Results**:

- Before: 8.2MB
- After: 2.4MB (70% reduction)

#### Startup Time

- **Lazy loading**: Only load what's needed
- **Minimal dependencies**: Avoid heavy crates
- **Static linking**: No dynamic library loading
- **Fast argument parsing**: clap with minimal features

#### Memory Usage

- **Stream processing**: Don't load entire files
- **Arena allocation**: When appropriate
- **String interning**: For repeated strings
- **Limit buffers**: Cap read sizes

## Security

### Principles

1. **Defense in Depth**: Multiple layers of protection
2. **Least Privilege**: Minimal permissions required
3. **Fail Secure**: Errors should deny access, not grant it
4. **Audit Everything**: Log all security-relevant operations

### Authentication & Authorization

#### MCP Tools (Future)

- **OAuth 2.0**: Standard authentication flow
- **JWT Tokens**: Short-lived (15 min) with refresh
- **Scope-Based**: Fine-grained permissions
- **Revocable**: Tokens can be revoked immediately

#### CLI Tools

- **File Permissions**: Respect OS permissions
- **No Credentials**: Never store secrets
- **Environment Variables**: For configuration only
- **Safe Defaults**: Deny by default

### Input Validation

```rust
// ✅ Good: Validate all inputs
pub fn create_rfd(title: &str) -> Result<Rfd> {
    if title.is_empty() {
        return Err(RfdError::InvalidTitle);
    }

    if title.len() > 200 {
        return Err(RfdError::TitleTooLong);
    }

    // Sanitize special characters
    let safe_title = sanitize_filename(title);

    Ok(Rfd { title: safe_title, /* ... */ })
}

// ❌ Bad: Trust user input
pub fn create_rfd(title: &str) -> Result<Rfd> {
    Ok(Rfd { title: title.to_string(), /* ... */ })
}
```

### Dependency Security

- **cargo-audit**: Run on every CI build
- **Minimal dependencies**: Fewer attack vectors
- **Pin versions**: Reproducible builds
- **Review updates**: Don't blindly update

### File System Safety

- **Path Sanitization**: Prevent directory traversal
- **Atomic Writes**: Temp file + rename
- **Permission Checks**: Verify before write
- **No Symlink Following**: Avoid symlink attacks

```rust
// ✅ Good: Safe file writing
pub async fn write_rfd(rfd: &Rfd, dir: &Path) -> Result<()> {
    // Validate path
    let path = dir.join(format!("{:04}.md", rfd.number));
    if !path.starts_with(dir) {
        return Err(Error::PathTraversal);
    }

    // Atomic write
    let temp = tempfile::NamedTempFile::new_in(dir)?;
    temp.write_all(rfd.to_markdown().as_bytes())?;
    temp.persist(&path)?;

    Ok(())
}
```

## Technology Stack

### Core

- **Language**: Rust (stable channel)
- **Async Runtime**: Tokio (for MCP tools)
- **Serialization**: serde (JSON, YAML)
- **Error Handling**: thiserror + anyhow

### CLI Tools

- **Argument Parsing**: clap (derive API)
- **Templating**: minijinja (Jinja2-compatible)
- **Markdown**: pulldown-cmark
- **Colors**: colored

### MCP Tools (Future)

- **MCP SDK**: rmcp (when stable)
- **Git Operations**: git2
- **Database**: sqlx (if needed)

### Development

- **Build System**: Cargo
- **Testing**: Built-in + assert_cmd + predicates
- **Benchmarking**: criterion
- **Mocking**: mockall
- **Property Testing**: proptest

### Infrastructure

- **CI/CD**: GitHub Actions
- **Package Management**: Nix flakes
- **Binary Distribution**: GitHub Releases
- **Code Coverage**: cargo-llvm-cov + Codecov
- **Security Scanning**: cargo-audit

## Design Principles

### From Stripe

> "Once code is written using our API, it should never need to change"

- **Backward Compatibility**: Never break existing integrations
- **Versioning**: Explicit versions, long deprecation cycles
- **Careful Evolution**: Add, don't change

### From Google

> "Documentation lives with code, not separate from it"

- **Inline Docs**: Rustdoc for all public APIs
- **Examples**: Every feature has an example
- **Architecture Docs**: In-repo, version controlled

### From Anthropic

> "Start with the simplest solution and iterate"

- **CLI First**: Validate before MCP investment
- **Minimal Viable**: Ship early, get feedback
- **User-Driven**: Let real usage guide development

### From Oxide

> "RFDs ensure thoughtful, collaborative decisions"

- **RFC Process**: Design before implementation
- **Written Artifacts**: Decisions are documented
- **Inclusive**: Everyone can participate

## Testing Strategy

### Pyramid Approach

```
        ┌─────┐
       ↗│ E2E │←  10% - Full integration, slow
      ↗ └─────┘
     ↗  ┌─────────┐
    ↗   │  Integ  │←  20% - Adapters, moderate
   ↗    └─────────┘
  ↗     ┌─────────────┐
 ↗      │    Unit     │←  70% - Domain logic, fast
↗       └─────────────┘
```

### Unit Tests (70%)

- **Focus**: Domain logic
- **Speed**: < 100ms total
- **Coverage**: > 80% of domain code
- **Isolation**: No I/O, pure functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_reject_empty_title() {
        let service = RfdService::new();
        let result = service.create_rfd("", "Author <a@example.com>");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RfdError::InvalidTitle);
    }
}
```

### Integration Tests (20%)

- **Focus**: Adapters and I/O
- **Speed**: < 5s total
- **Coverage**: All critical paths
- **Real Dependencies**: Temp files, test databases

```rust
#[tokio::test]
async fn should_persist_rfd_to_filesystem() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo = FileSystemRfdRepository::new(temp_dir.path());

    let rfd = Rfd { /* ... */ };
    repo.save(&rfd).await.unwrap();

    let loaded = repo.load(rfd.number).await.unwrap();
    assert_eq!(loaded, rfd);
}
```

### End-to-End Tests (10%)

- **Focus**: CLI commands and workflows
- **Speed**: < 30s total
- **Coverage**: Happy paths and critical errors
- **Real Binaries**: Test actual compiled output

```rust
#[test]
fn cli_creates_valid_rfd() {
    Command::cargo_bin("rfd")
        .unwrap()
        .args(&["create", "--title", "Test", "--author", "Me <me@example.com>"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Created RFD 0001"));
}
```

## Acknowledgments

Tapestry's architecture is inspired by:

- **[Anthropic](https://anthropic.com)** - MCP protocol and AI-first development
- **[Stripe](https://stripe.com)** - API design and backward compatibility
- **[Google](https://google.com)** - Engineering practices and documentation
- **[Oxide Computer](https://oxide.computer)** - RFD process and decision-making
- **[The Rust Community](https://www.rust-lang.org)** - Excellent tooling and
  practices

---

**See Also**:

- [Getting Started](GETTING_STARTED.md) - Installation and setup
- [Contributing](design/meta/CONTRIBUTING.md) - How to contribute
- [Roadmap](ROADMAP.md) - Future plans
