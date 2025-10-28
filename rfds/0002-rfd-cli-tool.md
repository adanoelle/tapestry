---
title: RFD CLI Tool
authors:
  - adanoelle <ada@tapestrylabs.org>
state: implemented
created: 2025-10-17T00:00:00Z
updated: 2025-10-27T19:46:41.661327800Z
tags:
  - cli
  - documentation
  - rfd
  - agent-friendly
  - skills
---

# RFD 0002: RFD CLI Tool

## Summary

Build a Rust-based CLI tool for managing RFD (Request for Discussion) documents
in Oxide Computer style. The tool enables agent-friendly document workflows
through structured operations, JSON output, and idempotent commands - designed
specifically for Claude Code Skills.

## Motivation

### Current State

Tapestry documentation currently uses ad-hoc markdown files and an RFC process
without tooling support. Creating and managing structured technical documents
requires manual file manipulation, making it error-prone and inconsistent.

### Problems to Solve

1. **No standardized documentation format** - RFCs, design docs, ADRs all use
   different structures
2. **Manual document management** - No tooling for creating, listing, or
   updating documents
3. **Agent-unfriendly workflows** - Claude must manually edit markdown files
4. **Inconsistent metadata** - No structured front matter or status tracking
5. **No searchability** - Can't easily find documents by status, author, or
   topic
6. **Missing template system** - Each document formatted differently

### Why RFD Format?

- **Proven at scale**: Oxide Computer uses RFDs successfully
- **Structured metadata**: YAML front matter with status, authors, dates
- **State machine**: Clear document lifecycle (draft → review → accepted →
  implemented)
- **Searchable**: Metadata enables filtering and discovery
- **AI-friendly**: Structured format is easier for agents to work with

### Why CLI Tool (not MCP)?

- **Fast startup**: < 10ms vs 100ms+ for MCP tools (critical for agent
  workflows)
- **Skills integration**: CLI tools are perfect for Skills to invoke
- **Token efficiency**: No protocol overhead, minimal context
- **Simple operations**: Document CRUD doesn't need MCP complexity
- **Validate Skills-first**: Test the Skills paradigm before heavy MCP
  investment

## Detailed Design

### Core Capabilities

```bash
# Create new RFD
rfd create --title "Feature Proposal" --author "Name <email@example.com>"

# List RFDs
rfd list --status draft --format json

# Show RFD details
rfd show 003 --format json

# Update status (idempotent)
rfd status 003 --set review --format json

# Update metadata
rfd update 003 --field title --value "New Title" --format json

# Validate RFD structure
rfd validate 003 --format json
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
3. **Idempotency** - Commands succeed whether state changes or is already
   correct
4. **Actionable Errors** - Error messages include suggestions for fixes
5. **Non-Interactive** - No prompts; all data via flags
6. **Three Output Modes**:
   - `pretty` (default): Human-readable with colors
   - `json`: Structured data for agents
   - `quiet`: Errors only

## Implementation

**Status**: ✅ IMPLEMENTED (as of 2025-10-27)

The RFD CLI has been successfully implemented with all planned features:

### Phase 1: MVP ✅

- [x] CLI structure with clap (create, list, show, status, update, validate, search)
- [x] Template rendering with minijinja
- [x] YAML front matter parsing
- [x] File I/O and directory management
- [x] JSON output mode
- [x] Basic validation

### Phase 2: Agent Features ✅

- [x] Idempotent operations
- [x] Structured error responses
- [x] Status transition validation
- [x] Non-interactive mode
- [x] Configuration file support
- [x] Verbose mode for diagnostics

### Phase 3: Polish ✅

- [x] Comprehensive testing (61 tests passing)
- [x] Documentation and examples
- [x] Search functionality with multiple scopes (content, metadata, tags)
- [x] Multiple filter options (status, author, limit)

### Current Capabilities

The tool now supports:

- **Create**: New RFDs with templates and default author from config
- **List**: Filter by status, author, limit with JSON output
- **Show**: Display RFD details
- **Status**: Update state with validation
- **Update**: Modify metadata fields
- **Validate**: Check RFD structure
- **Search**: Full-text search across content, metadata, or tags

## Alternative Approaches

### Alternative 1: Manual Markdown Files

**Pros**: Simple, no tooling needed **Cons**: Error-prone, inconsistent,
agent-unfriendly **Decision**: Rejected - scales poorly, doesn't solve the
problem

### Alternative 2: MCP Tool Instead of CLI

**Pros**: Deeper integration potential **Cons**: Slower startup (100ms+), more
complex, higher token cost **Decision**: Rejected - CLI is better fit for simple
file operations

### Alternative 3: Python Script

**Pros**: Faster initial development (1-2 days vs 2-4 weeks) **Cons**: 100ms
startup vs 5ms, 25-100MB vs 3MB, dependency management **Decision**: Rejected -
Rust's performance characteristics better for agents

### Alternative 4: AsciiDoc Format (Oxide's actual format)

**Pros**: True Oxide compatibility **Cons**: Markdown more familiar, can convert
later via Pandoc **Decision**: Use Markdown, add AsciiDoc export as future
extension

## Success Criteria

### Immediate (Launch) ✅

- [x] RFD CLI compiles and runs
- [x] Can create, list, show RFDs
- [x] JSON output works
- [x] Status transitions validated
- [x] Idempotent operations

### Short-term (1 month) 🎯

- [x] Dogfood on Tapestry documentation (started 2025-10-27)
- [x] Convert existing RFCs to RFD format (RFD-001, RFD-002, RFD-003)
- [ ] Template system working (using default template)
- [ ] rfd-manager Skill functional (planned)
- [ ] Binary distribution available

### Long-term (3 months)

- [ ] 20+ RFDs in Tapestry
- [ ] GitHub integration complete (RFD-003)
- [ ] External users adopting tool
- [ ] Skills limitations documented from real usage

## Open Questions

1. **Template extensibility**: How far should we go with custom templates?

   - **Current**: Single default template, works well for now

2. **RFD numbering**: Auto-increment or allow gaps? How to handle deletions?

   - **Current**: Auto-increment, no deletion support needed yet

3. **Multi-document types**: Should we support ADRs, specs separately or one
   tool?

   - **Current**: Focus on RFDs, can extend later if needed

4. **gh CLI integration**: Build into rfd or separate tool?
   - **Decision**: Separate GitHub integration (see RFD-003)

## Future Extensions

- **GitHub Integration** - See RFD-003 for GitHub issue integration
- **Multiple templates**: RFD, ADR, spec, feature templates
- **Git integration**: Auto-branch, auto-commit for RFDs
- **Export formats**: AsciiDoc, HTML, PDF via Pandoc
- **RFD linking**: Automatic cross-references between RFDs
- **rfd-manager Skill**: Skill that orchestrates RFD workflows

## References

- [VISION.md](../docs/VISION.md) - Hybrid architecture approach
- [RFD-001](./0001-git-workflow-automation-tool.md) - Git workflow tool
- [RFD-003](./0003-github-integration.md) - GitHub integration
- [Oxide RFDs](https://rfd.shared.oxide.computer/) - Original inspiration

---

**Authors**: adanoelle <ada@tapestrylabs.org> **Created**: 2025-10-17
**Updated**: 2025-10-27 **State**: implemented
