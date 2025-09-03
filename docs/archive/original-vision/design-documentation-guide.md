# Design Documentation Organization Guide

**Purpose:** This guide explains how to organize, name, and structure design
documents in this repository. It serves as a reference for both human
contributors and AI assistants (especially Claude Code) when creating new design
documentation.

## Directory Structure Philosophy

### Hierarchical Organization

We organize docs by **logical relationships** rather than chronological creation
order:

- **Core**: Foundational concepts that everything else builds upon
- **Servers**: Individual MCP server specifications
- **Features**: User-facing capabilities that span multiple servers
- **Implementation**: Practical guides for development
- **Meta**: Documentation about the project itself

### When to Use Numbered Prefixes

**USE numbers when documents have dependencies:**

```
01-foundation-concept.md    # Must understand this first
02-builds-on-foundation.md  # Requires knowledge from 01
03-advanced-usage.md        # Requires knowledge from 01 and 02
```

**DON'T use numbers for independent documents:**

```
authentication-server.md    # Standalone server design
payment-server.md          # Independent of authentication
notification-server.md     # Independent of both above
```

#### Numbering Guidelines

- **Start from 01** (not 00 or 1)
- **Use two digits** for future expansion (01, 02, ..., 10, 11)
- **Leave gaps for insertion**: Use 01, 03, 05 if you might add docs between
  them later
- **Renumber when necessary**: It's okay to renumber if dependencies change
  significantly

## Directory-Specific Rules

### `/core/` - Numbered Sequential Architecture

```
docs/design/core/
├── 01-event-model.md           # Everything builds on events
├── 02-provenance-tracker.md    # Central orchestrator
├── 03-session-management.md    # Uses events and tracker
├── 04-storage-architecture.md  # Uses all above concepts
```

**Rationale:** Core architecture has clear dependencies - you can't understand
sessions without understanding events.

### `/servers/` - Grouped by Server, No Numbers Between Servers

```
docs/design/servers/
├── ai-interaction-logger/
│   ├── 01-requirements.md      # Numbers within server for development phases
│   ├── 02-design.md
│   └── 03-mcp-interface.md
├── file-system-monitor/
│   └── design.md               # Single doc if no internal sequence needed
├── decision-graph/
│   ├── 01-data-model.md
│   └── 02-algorithms.md
```

**Rationale:** Each server can be developed independently, but within a server,
there may be a logical development sequence.

### `/features/` - No Numbers, Independent Features

```
docs/design/features/
├── claude-md-enhancement.md
├── decision-tracking.md
├── cross-session-memory.md
├── team-collaboration.md
```

**Rationale:** Features are user-facing capabilities that can be understood and
developed independently.

### `/implementation/` - Numbered Development Process

```
docs/design/implementation/
├── 01-mcp-patterns.md       # Start here for any implementation
├── 02-data-flow.md          # Then understand system data flow
├── 03-testing-strategy.md   # Then learn testing approaches
├── 04-deployment.md         # Finally, deployment
```

**Rationale:** There's a natural progression from patterns → architecture →
testing → deployment.

### `/meta/` - No Numbers, Reference Material

```
docs/design/meta/
├── architecture-decisions.md
├── research-agenda.md
├── roadmap.md
├── contributing.md
└── this-guide.md
```

**Rationale:** Meta documents are reference material accessed as needed, not in
sequence.

## Document Templates and Structure

### Status Indicators

Every design document should start with:

```markdown
**Status:** [🚧 Draft | 📝 In Progress | ✅ Complete | 🔄 Under Review | 📚
Reference]  
**Last Updated:** [YYYY-MM-DD]  
**Related:** [Links to related documents]
```

### Template Selection Guide

#### Core Architecture Template

Use for: Foundational system concepts, data models, central orchestrators

```markdown
# [Component Name]

## Overview

## Requirements

## Design

## Implementation Notes

## Integration Points

## Testing Strategy
```

#### MCP Server Template

Use for: Individual MCP server specifications

```markdown
# [Server Name] MCP Server

## Purpose

## MCP Interface (Tools/Resources/Prompts)

## Data Model

## Implementation Plan

## Usage Examples
```

#### Feature Template

Use for: User-facing capabilities, cross-cutting concerns

```markdown
# [Feature Name]

## Problem Statement

## Solution Approach

## User Experience

## Technical Design

## Success Metrics

## Future Extensions
```

#### Implementation Guide Template

Use for: Development processes, patterns, deployment guides

```markdown
# [Guide Name]

## Overview

## Prerequisites

## Step-by-Step Process

## Examples

## Common Pitfalls

## References
```

## Naming Conventions

### File Names

- **Use kebab-case**: `claude-md-enhancement.md` not `ClaudeMdEnhancement.md`
- **Be descriptive**: `decision-tracking.md` not `decisions.md`
- **Avoid abbreviations**: `authentication-server.md` not `auth-server.md`
  (unless the abbreviation is very standard)

### Directory Names

- **Use singular nouns**: `server/` not `servers/` (following Unix convention)
- **Be consistent**: If you use `server/`, don't mix with `mcp-servers/`

### Section Headers

- **Use sentence case**: `## Data model` not `## Data Model`
- **Be consistent**: Pick a style for the entire document
- **Make them scannable**: `## Storage architecture decisions` not
  `## Decisions about how we store stuff`

## AI Assistant Instructions

### For Claude Code When Creating New Docs

1. **Determine the correct directory** using the rules above
2. **Check if numbering is needed** based on dependencies
3. **Select the appropriate template** based on document type
4. **Use consistent naming conventions**
5. **Add the document to the index** (`docs/design/README.md`)
6. **Cross-reference related documents**

### Example Prompts for Scaffolding

**Creating a new MCP server design:**

```
"Create a design document for the [Server Name] MCP server following our documentation standards. Put it in docs/design/servers/[server-name]/, use the MCP Server Template, and update the design index."
```

**Creating a new feature specification:**

```
"Create a feature design document for [Feature Name] following our documentation organization guide. This should be an independent feature document in docs/design/features/."
```

### Document Relationships

When creating new documents, always consider:

- **Dependencies**: What documents should be read first?
- **Related concepts**: What other documents should be cross-referenced?
- **Integration points**: How does this connect to the overall system?
- **Index updates**: Does the main index need updating?

## Quality Guidelines

### Documentation Quality Checklist

- [ ] Status and metadata are current
- [ ] Template structure is followed consistently
- [ ] Cross-references to related documents are included
- [ ] Examples are concrete and realistic
- [ ] Technical details are sufficient for implementation
- [ ] Success criteria are clearly defined

### Maintenance Guidelines

- **Update status** as documents evolve from draft to complete
- **Keep cross-references current** when documents are moved or renamed
- **Archive obsolete documents** rather than deleting (move to `docs/archive/`)
- **Update the main index** when adding new documents

## Examples of Good Organization

### Well-Organized Server Documentation

```
docs/design/servers/ai-interaction-logger/
├── 01-requirements.md          # What this server needs to do
├── 02-design.md               # Technical architecture
├── 03-mcp-interface.md        # MCP tools/resources specification
└── implementation-notes.md    # Development considerations (no number - reference material)
```

### Well-Organized Feature Documentation

```
docs/design/features/
├── claude-md-enhancement.md   # Independent feature - no dependencies
├── decision-tracking.md       # Can be understood without claude-md-enhancement
├── pattern-learning.md        # Builds on decision-tracking but still standalone
```

## Anti-Patterns to Avoid

### Don't Do This

```
❌ docs/design/001-everything-you-need-to-know.md
❌ docs/servers-and-stuff/
❌ docs/design/random-thoughts.md
❌ docs/design/TODO.md
```

### Do This Instead

```
✅ docs/design/core/01-event-model.md
✅ docs/design/servers/file-system-monitor/
✅ docs/design/features/pattern-learning.md
✅ docs/design/meta/research-agenda.md
```

---

_This guide should be referenced when creating any new design documentation. It
ensures consistency and helps both humans and AI assistants contribute
effectively to the project's documentation._
