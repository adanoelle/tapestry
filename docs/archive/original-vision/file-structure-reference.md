# Design Documentation File Structure

**Purpose:** Complete reference for where to place design documents in the
Development Provenance Platform repository.

## Root Level Documents

```
VISION.md                           # Project vision and goals
README.md                           # Repository overview and quick start
```

## Complete Documentation Structure

```
docs/
├── design/
│   ├── README.md                                    # Design documentation index
│   │
│   ├── core/                                        # Foundational system components
│   │   ├── 01-event-model.md                       # Unified event structure (foundation)
│   │   ├── 02-provenance-tracker.md                # Central orchestrator
│   │   ├── 03-session-management.md                # Development session concepts
│   │   └── 04-storage-architecture.md              # Data persistence and querying
│   │
│   ├── servers/                                     # MCP server specifications
│   │   ├── ai-interaction-logger/                  # Claude Code interaction capture
│   │   │   ├── design.md                           # Core server design
│   │   │   ├── mcp-interface.md                    # MCP protocol details
│   │   │   └── implementation-notes.md             # Development guidance
│   │   │
│   │   ├── file-system-monitor/                    # File change detection
│   │   │   ├── design.md
│   │   │   ├── mcp-interface.md
│   │   │   └── implementation-notes.md
│   │   │
│   │   ├── git-intelligence/                       # Git operations and analysis
│   │   │   ├── design.md
│   │   │   ├── mcp-interface.md
│   │   │   └── implementation-notes.md
│   │   │
│   │   ├── decision-graph/                         # Decision tracking and linking
│   │   │   ├── design.md
│   │   │   └── mcp-interface.md
│   │   │
│   │   ├── pattern-recognition/                    # Development pattern analysis
│   │   │   ├── design.md
│   │   │   └── mcp-interface.md
│   │   │
│   │   ├── project-memory/                         # Cross-session knowledge
│   │   │   ├── design.md
│   │   │   └── mcp-interface.md
│   │   │
│   │   ├── context-bridge/                         # Session continuity
│   │   │   ├── design.md
│   │   │   └── mcp-interface.md
│   │   │
│   │   ├── documentation-generator/                # Living docs generation
│   │   │   ├── design.md
│   │   │   └── mcp-interface.md
│   │   │
│   │   └── review-assistant/                       # Code review support
│   │       ├── design.md
│   │       └── mcp-interface.md
│   │
│   ├── features/                                    # User-facing capabilities
│   │   ├── claude-md-enhancement.md                # Documentation divergence detection
│   │   ├── decision-tracking.md                    # Architectural decision capture
│   │   ├── cross-session-memory.md                 # Context preservation
│   │   ├── pattern-learning.md                     # Development pattern recognition
│   │   └── team-collaboration.md                   # Multi-developer workflows
│   │
│   ├── implementation/                              # Cross-cutting technical guidance
│   │   ├── 01-mcp-patterns.md                      # Common MCP server patterns
│   │   ├── 02-data-flow.md                         # System-wide data movement
│   │   ├── 03-testing-strategy.md                  # Testing approach across components
│   │   └── 04-deployment.md                        # Installation and configuration
│   │
│   └── meta/                                        # Project management and process
│       ├── design-documentation-guide.md           # This document's companion
│       ├── architecture-decisions.md               # Key technical choices log
│       ├── research-agenda.md                      # Open questions and investigations
│       ├── roadmap.md                              # Development phases and milestones
│       └── contributing.md                         # How to contribute to the platform
```

## File Placement Rules

### Core Architecture (`docs/design/core/`)

**When to use:** Foundational system components that other parts depend on.
**Numbering:** Always numbered - these build on each other. **Examples:** Event
models, storage systems, session management.

### MCP Servers (`docs/design/servers/[server-name]/`)

**When to use:** Individual MCP server specifications. **Structure:** Each
server gets its own directory with consistent file names. **Files:**

- `design.md` - Core server architecture and purpose
- `mcp-interface.md` - MCP protocol specifics (tools, resources, prompts)
- `implementation-notes.md` - Development guidance and considerations

### Features (`docs/design/features/`)

**When to use:** User-facing capabilities that span multiple components.
**Numbering:** No numbers - these are independent feature specifications.
**Examples:** CLAUDE.md enhancement, decision tracking, collaboration features.

### Implementation Guides (`docs/design/implementation/`)

**When to use:** Cross-cutting technical guidance for developers. **Numbering:**
Numbered - these represent a learning/implementation sequence. **Examples:** MCP
patterns, testing strategies, deployment guides.

### Meta Documentation (`docs/design/meta/`)

**When to use:** Process, project management, and self-referential
documentation. **Numbering:** No numbers - these are reference materials.
**Examples:** This guide, architecture decisions, research agendas.

## Quick Reference for Document Creation

### "I'm designing a new MCP server"

**Location:** `docs/design/servers/[server-name]/design.md` **Template:** MCP
Server Template **Also create:** `mcp-interface.md` for protocol details

### "I'm specifying a user-facing feature"

**Location:** `docs/design/features/[feature-name].md` **Template:** Feature
Specification Template **Consider:** Does this need its own subdirectory if
complex?

### "I'm documenting a core system component"

**Location:** `docs/design/core/[##-component-name].md` **Template:** Core
Architecture Template **Numbering:** Yes - determine dependency order

### "I'm writing implementation guidance"

**Location:** `docs/design/implementation/[##-guide-name].md` **Template:** Core
Architecture Template (adapted for guidance) **Numbering:** Yes - logical
learning sequence

### "I'm documenting a decision or process"

**Location:** `docs/design/meta/[document-name].md` **Template:** Varies based
on content type **Numbering:** No - reference material

## Maintenance Guidelines

### When Adding New Documents

1. Create the file in the appropriate location
2. Use the correct template from the documentation guide
3. Update `docs/design/README.md` index
4. Link from related documents where appropriate

### Directory Creation

- Create server directories as needed:
  `mkdir -p docs/design/servers/new-server-name`
- Create feature subdirectories if the feature is complex enough to warrant
  multiple files
- Don't create empty directories - add them when you have content

### File Naming Conventions

- Use `kebab-case-naming.md` for all files
- Number only when sequence matters: `01-foundational-concept.md`
- Keep names descriptive but concise
- Avoid abbreviations that might be unclear

### Cross-References

- Link related documents in headers:
  `**Related:** [Event Model](../core/01-event-model.md)`
- Update the design index when adding new documents
- Use relative paths for internal links
- Check links when reorganizing files

## Integration with Development Workflow

### For Human Developers

- Browse `docs/design/README.md` to understand system architecture
- Start with core concepts, then dive into specific servers or features
- Use meta documentation to understand contribution process

### For Claude Code

- Reference the design documentation guide for template usage
- Follow file placement rules when creating new design documents
- Maintain consistency with existing document structure
- Update indexes and cross-references when adding new content

### For Provenance Tracking

- This structure becomes part of your development provenance
- Document creation follows traceable patterns
- File organization reflects architectural decisions
- Changes to structure indicate evolving system understanding

---

_This structure will evolve as the platform grows. Maintain this reference when
making organizational changes._
