# RFD CLI Tool

> **Status**: ✅ Production Ready
>
> A Rust-based CLI tool for creating and managing RFD (Request for Discussion)
> documents. Designed for agent-friendly operation with Claude Code Skills.

## Overview

The RFD CLI provides a structured, agent-friendly interface for managing
technical documentation in the Oxide Computer style RFD format. Built with Rust
for fast startup times (< 10ms) and zero-dependency distribution.

## Features

- Create new RFD documents from templates
- List and filter existing RFDs
- **Search RFDs by content** with field-specific queries and filters
- Update metadata without manual file editing
- Validate RFD structure and conventions
- JSON output for agent consumption
- Idempotent operations for safe retries
- Actionable error messages

## Installation

### From Source

```bash
cargo install --path .
```

### Binary Release (Coming Soon)

```bash
# Download latest release
curl -L https://github.com/org/tapestry/releases/latest/download/rfd -o ~/.local/bin/rfd
chmod +x ~/.local/bin/rfd
```

## Usage

### Create a New RFD

```bash
rfd create --title "Feature Proposal" --author "Name <email@example.com>"
```

### List RFDs

```bash
rfd list
rfd list --status draft --json
```

### Search RFDs

```bash
# Basic text search
rfd search "authentication"

# Multiple terms (AND logic - all must match)
rfd search "oauth api"

# Search specific fields
rfd search "security" --in title
rfd search "database" --in content
rfd search "api" --in tags

# Case-sensitive search
rfd search "OAuth" --case-sensitive

# Combine search with filters
rfd search "api" --status accepted
rfd search "security" --author alice --limit 5

# JSON output for agents
rfd search "authentication" --format json
```

### Update Status

```bash
rfd status 003 --set review
```

### Validate

```bash
rfd validate 003 --json
```

## Output Modes

- **Pretty** (default): Human-readable with colors
- **JSON** (`--format json`): Structured output for agents
- **Quiet** (`--format quiet`): Errors only

## RFD Document Structure

RFDs use YAML front matter with standard sections:

```yaml
---
title: "Document Title"
authors: ["Author Name <email@example.com>"]
state: draft  # draft, review, accepted, rejected, implemented, archived
discussion: "https://github.com/org/repo/issues/123"
created: 2025-10-17
updated: 2025-10-17
tags: ["tag1", "tag2"]
---

# Summary
Brief overview of the proposal

# Motivation
Why this RFD exists

# Proposal
Detailed technical proposal

# Implementation
How to build it

# Alternatives
Other approaches considered

# Open Questions
Unresolved issues
```

## Performance Metrics

Measured on Linux 6.15 with Rust 1.83 (2025-10-19):

| Metric                | Target | Actual | Status |
| --------------------- | ------ | ------ | ------ |
| Binary size (release) | < 3MB  | 2.4MB  | ✅     |
| Startup time (cold)   | < 10ms | 1ms    | ✅     |
| Test suite execution  | -      | ~10ms  | -      |
| Memory usage (peak)   | < 10MB | ~5MB   | ✅     |

**Optimization techniques**:

- Link-time optimization (LTO)
- Single code generation unit
- Size-optimized compilation (`opt-level = "z"`)
- Stripped debug symbols
- Panic = abort strategy

See `Cargo.toml` `[profile.release]` section for details.

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

### Run

```bash
cargo run -- --help
```

## Architecture

The RFD CLI follows a layered architecture:

- **CLI Layer** (`main.rs`): Argument parsing with clap
- **Application Layer**: Business logic and orchestration
- **Domain Layer**: Core RFD types and validation
- **Infrastructure Layer**: File I/O, templates, git integration

## Design Principles

1. **Agent-First**: JSON output, idempotent operations, structured errors
2. **Fast**: < 10ms startup time, optimized for quick invocations
3. **Simple**: Single binary, no dependencies, clear commands
4. **Extensible**: Template system, configurable, plugin-friendly

## Documentation

- **[README.md](README.md)** (this file) - User guide and quick start
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Codebase structure and design
  patterns
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guide for developers
- **[examples/](examples/)** - Practical usage examples and scripts

## Examples

See the [examples/](examples/) directory for practical demonstrations:

- **[basic_workflow.sh](examples/basic_workflow.sh)** - Fundamental operations
  walkthrough
- **[bulk_operations.sh](examples/bulk_operations.sh)** - Automation and batch
  processing
- **[filtering_and_search.sh](examples/filtering_and_search.sh)** - Advanced
  querying with jq
- **[custom_templates.sh](examples/custom_templates.sh)** - Template
  customization
- **[agent_integration.sh](examples/agent_integration.sh)** - CI/CD and
  automation patterns

Each example is an executable bash script with detailed comments. See
[examples/README.md](examples/README.md) for details.

## Getting Help

- **Documentation**: Start with [ARCHITECTURE.md](ARCHITECTURE.md) for codebase
  overview
- **Examples**: Check [examples/](examples/) for practical usage patterns
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md) for development guide
- **Issues**: Found a bug?
  [Open an issue](https://github.com/yourusername/tapestry/issues)
- **Discussions**: Questions?
  [Start a discussion](https://github.com/yourusername/tapestry/discussions)

## Related Projects

- [Oxide RFD](https://rfd.shared.oxide.computer/) - Original RFD format and
  inspiration
- [Tapestry](../../) - Parent project with MCP tools and Skills
- [Model Context Protocol](https://modelcontextprotocol.io/) - AI-native
  protocol

## License

MIT OR Apache-2.0
