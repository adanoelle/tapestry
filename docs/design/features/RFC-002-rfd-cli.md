# RFC-002: RFD CLI Tool

## Summary

Build a Rust-based CLI tool for managing RFD (Request for Discussion) documents in Oxide Computer style. The tool enables agent-friendly document workflows through structured operations, JSON output, and idempotent commands - designed specifically for Claude Code Skills.

## Motivation

### Current State

Tapestry documentation currently uses ad-hoc markdown files and an RFC process without tooling support. Creating and managing structured technical documents requires manual file manipulation, making it error-prone and inconsistent.

### Problems to Solve

1. **No standardized documentation format** - RFCs, design docs, ADRs all use different structures
2. **Manual document management** - No tooling for creating, listing, or updating documents
3. **Agent-unfriendly workflows** - Claude must manually edit markdown files
4. **Inconsistent metadata** - No structured front matter or status tracking
5. **No searchability** - Can't easily find documents by status, author, or topic
6. **Missing template system** - Each document formatted differently

### Why RFD Format?

- **Proven at scale**: Oxide Computer uses RFDs successfully
- **Structured metadata**: YAML front matter with status, authors, dates
- **State machine**: Clear document lifecycle (draft → review → accepted → implemented)
- **Searchable**: Metadata enables filtering and discovery
- **AI-friendly**: Structured format is easier for agents to work with

### Why CLI Tool (not MCP)?

- **Fast startup**: < 10ms vs 100ms+ for MCP tools (critical for agent workflows)
- **Skills integration**: CLI tools are perfect for Skills to invoke
- **Token efficiency**: No protocol overhead, minimal context
- **Simple operations**: Document CRUD doesn't need MCP complexity
- **Validate Skills-first**: Test the Skills paradigm before heavy MCP investment

## Detailed Design

### Core Capabilities

```bash
# Create new RFD
rfd create --title "Feature Proposal" --author "Name <email@example.com>"

# List RFDs
rfd list --status draft --json

# Show RFD details
rfd show 003 --json

# Update status (idempotent)
rfd status 003 --set review --json

# Update metadata
rfd update 003 --field title --value "New Title" --json

# Validate RFD structure
rfd validate 003 --json
```

### Document Structure

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

### State Machine

```
draft ──> review ──> accepted ──> implemented
  │         │           │
  └─────────┴───────────┴───> rejected ──> archived
```

**Transitions**:
- `draft` can go to: `review`, `accepted`, `rejected`
- `review` can go to: `accepted`, `rejected`, `draft` (if needs changes)
- `accepted` can go to: `implemented`, `rejected` (if circumstances change)
- `rejected` can only go to: `archived`
- `implemented` can only go to: `archived`
- `archived` is terminal

### Agent-Friendly Design Principles

1. **Structured Operations** - Atomic commands instead of file editing
2. **JSON Output** - All commands support `--format json` for parsing
3. **Idempotency** - Commands succeed whether state changes or is already correct
4. **Actionable Errors** - Error messages include suggestions for fixes
5. **Non-Interactive** - No prompts; all data via flags
6. **Three Output Modes**:
   - `pretty` (default): Human-readable with colors
   - `json`: Structured data for agents
   - `quiet`: Errors only

### Template System

**Default Template** (`~/.config/rfd/templates/default.md.jinja`):

```jinja2
---
title: {{ title }}
authors: {{ authors | join(', ') }}
created: {{ created_date }}
state: {{ state }}
discussion: {{ discussion_url }}
---

# RFD {{ rfd_number }}: {{ title }}

## Summary
{{ summary }}

## Motivation
{{ motivation }}

{% for section in sections %}
## {{ section.title }}
{{ section.content }}
{% endfor %}
```

**Template Variables**:
- `rfd_number`: Zero-padded ID (e.g., "003")
- `title`, `authors`, `created_date`, `updated_date`
- `state`, `discussion_url`, `tags`
- `summary`, `motivation`, `sections`

**Template Locations** (priority order):
1. Project: `.rfd/templates/`
2. User: `~/.config/rfd/templates/`
3. Built-in: Embedded in binary

### File Organization

```
project/
├── .rfd/
│   ├── config.toml          # Project configuration
│   └── templates/           # Custom templates
│       └── default.md.jinja
└── rfds/
    ├── 0001-initial-proposal.md
    ├── 0002-architecture.md
    └── 0003-implementation.md
```

### Configuration

```.toml
# .rfd/config.toml
[rfd]
directory = "rfds"
template = "default"
id_format = "{:04d}"  # Zero-padded 4 digits

[metadata]
default_state = "draft"
required_fields = ["title", "authors", "summary"]

[output]
default_format = "pretty"  # pretty, json, quiet
color = "auto"  # auto, always, never
```

### JSON Output Examples

**List Response**:
```json
{
  "rfds": [
    {
      "id": "001",
      "title": "Initial Proposal",
      "state": "accepted",
      "authors": ["Alice <alice@example.com>"],
      "created": "2025-10-17",
      "updated": "2025-10-20",
      "path": "rfds/0001-initial-proposal.md"
    }
  ],
  "total": 1
}
```

**Error Response**:
```json
{
  "error": "INVALID_TRANSITION",
  "message": "Cannot transition from 'draft' to 'archived'",
  "details": {
    "current_state": "draft",
    "valid_next_states": ["review", "accepted", "rejected"],
    "suggestion": {
      "command": "rfd status 003 --set rejected",
      "description": "Move to rejected state instead"
    }
  }
}
```

## Technical Implementation

### Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }      # CLI framework
minijinja = "2"                                      # Templating
gray_matter = "0.2"                                  # Front matter parsing
serde = { version = "1", features = ["derive"] }    # Serialization
serde_yaml = "0.9"                                   # YAML parsing
chrono = { version = "0.4", features = ["serde"] }  # Date/time
pulldown-cmark = "0.11"                              # Markdown parsing
anyhow = "1"                                         # Error handling
colored = "2"                                        # Terminal colors
```

### Architecture

Simple, flat structure (no hexagonal architecture needed):

```
cli/rfd/
├── src/
│   ├── main.rs           # CLI entry point, clap setup
│   ├── commands/         # Command implementations
│   │   ├── create.rs
│   │   ├── list.rs
│   │   ├── show.rs
│   │   ├── status.rs
│   │   └── validate.rs
│   ├── document.rs       # RFD document model
│   ├── template.rs       # Template rendering
│   ├── config.rs         # Configuration management
│   └── output.rs         # Output formatting (pretty/json/quiet)
├── templates/            # Built-in templates
│   └── default.md.jinja
└── Cargo.toml
```

### Performance Targets

- **Startup time**: < 10ms (cold start)
- **Create RFD**: < 50ms
- **List RFDs**: < 100ms for 100 documents
- **Binary size**: < 3MB (stripped with LTO)
- **Memory usage**: < 10MB peak

## Alternative Approaches Considered

### Alternative 1: Manual Markdown Files

**Pros**: Simple, no tooling needed
**Cons**: Error-prone, inconsistent, agent-unfriendly
**Decision**: Rejected - scales poorly, doesn't solve the problem

### Alternative 2: MCP Tool Instead of CLI

**Pros**: Deeper integration potential
**Cons**: Slower startup (100ms+), more complex, higher token cost
**Decision**: Rejected - CLI is better fit for simple file operations

### Alternative 3: Python Script

**Pros**: Faster initial development (1-2 days vs 2-4 weeks)
**Cons**: 100ms startup vs 5ms, 25-100MB vs 3MB, dependency management
**Decision**: Rejected - Rust's performance characteristics better for agents

### Alternative 4: AsciiDoc Format (Oxide's actual format)

**Pros**: True Oxide compatibility
**Cons**: Markdown more familiar, can convert later via Pandoc
**Decision**: Use Markdown, add AsciiDoc export as future extension

## Implementation Plan

### Week 1-2: MVP

- [ ] CLI structure with clap (create, list, show, status)
- [ ] Template rendering with minijinja
- [ ] YAML front matter parsing
- [ ] File I/O and directory management
- [ ] JSON output mode
- [ ] Basic validation

### Week 3: Agent Features

- [ ] Idempotent operations
- [ ] Structured error responses
- [ ] Status transition validation
- [ ] Non-interactive mode
- [ ] Configuration file support

### Week 4: Polish

- [ ] Custom template loading
- [ ] Comprehensive testing
- [ ] Binary distribution setup
- [ ] Documentation and examples
- [ ] Create rfd-manager Skill

## Success Criteria

### Immediate (Launch)

- [x] RFD CLI compiles and runs
- [ ] Can create, list, show RFDs
- [ ] JSON output works
- [ ] Status transitions validated
- [ ] Idempotent operations

### Short-term (1 month)

- [ ] Dogfood on all Tapestry documentation
- [ ] Convert existing RFCs to RFD format
- [ ] Template system working
- [ ] rfd-manager Skill functional
- [ ] Binary distribution available

### Long-term (3 months)

- [ ] 20+ RFDs in Tapestry
- [ ] gh CLI integration (issue → RFD)
- [ ] External users adopting tool
- [ ] Skills limitations documented from real usage

## Open Questions

1. **Template extensibility**: How far should we go with custom templates?
2. **RFD numbering**: Auto-increment or allow gaps? How to handle deletions?
3. **Multi-document types**: Should we support ADRs, specs separately or one tool?
4. **gh CLI integration**: Build into rfd or separate tool?
5. **AsciiDoc export**: Priority for Oxide compatibility?

## Future Extensions

- **Search functionality**: Full-text search across RFDs
- **gh CLI integration**: `gh issue view 123 | rfd create --from-issue`
- **Multiple templates**: RFD, ADR, spec, feature templates
- **Git integration**: Auto-branch, auto-commit for RFDs
- **Export formats**: AsciiDoc, HTML, PDF via Pandoc
- **RFD linking**: Automatic cross-references between RFDs

---

**RFC Status**: PROPOSED
**Created**: 2025-10-17
**Author**: Ada
**Related**: VISION.md (hybrid architecture), RFC-001 (git-workflow tool)
