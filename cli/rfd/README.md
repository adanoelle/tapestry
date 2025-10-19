# RFD CLI Tool

> **Status**: 🚧 In Development
>
> A Rust-based CLI tool for creating and managing RFD (Request for Discussion) documents. Designed for agent-friendly operation with Claude Code Skills.

## Overview

The RFD CLI provides a structured, agent-friendly interface for managing technical documentation in the Oxide Computer style RFD format. Built with Rust for fast startup times (< 10ms) and zero-dependency distribution.

## Features

- ✅ Create new RFD documents from templates
- ✅ List and search existing RFDs
- ✅ Update metadata without manual file editing
- ✅ Validate RFD structure and conventions
- ✅ JSON output for agent consumption
- ✅ Idempotent operations for safe retries
- ✅ Actionable error messages

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

## Related Projects

- [Oxide RFD](https://rfd.shared.oxide.computer/): Original RFD format
- [Tapestry](../../): Parent project with MCP tools and Skills

## License

MIT OR Apache-2.0
