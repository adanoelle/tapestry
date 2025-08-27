# Development Context for Claude Code

This document provides essential context for AI assistants working on the Development Provenance Platform.

## Project Overview

We're building a **Development Provenance Platform** - a collection of Rust-based MCP servers that capture the complete context of human-AI collaboration during software development. The goal is to transform AI-assisted coding from a black box into a transparent, learnable process that builds institutional knowledge.

**Key Concept:** Instead of losing the "why" behind development decisions, we capture reasoning, alternatives considered, and outcomes to create persistent development knowledge that survives team changes.

## Architecture Philosophy

- **MCP-native design**: Built specifically for the Claude ecosystem using Model Context Protocol
- **Event sourcing approach**: All development activities become traceable events
- **Composable intelligence**: Multiple specialized MCP servers work together
- **Human-AI collaboration focus**: Capture both human reasoning and AI suggestions
- **Institutional memory**: Build knowledge that persists across team changes

## Current Development Phase

**Phase 1: Foundation Building**
- Defining core event models and data structures
- Implementing the first MCP server (AI Interaction Logger)
- Establishing development patterns for the monorepo
- Creating basic provenance data collection

## Documentation Structure

Our design documentation follows a strict organization pattern:

### Essential Documents to Reference
- **`docs/VISION.md`** - Complete project vision and long-term goals
- **`docs/design/README.md`** - Design documentation index and navigation
- **`docs/design/meta/design-documentation-guide.md`** - Templates and standards for creating new design docs
- **`docs/design/meta/file-structure-reference.md`** - Where to place new documentation

### When Creating New Design Documents
1. **Check the file structure reference** for correct placement
2. **Use appropriate templates** from the documentation guide
3. **Follow naming conventions** (kebab-case, numbering only when sequence matters)
4. **Update the design index** when adding new documents

### Documentation Categories
- **`docs/design/core/`** - Foundational components (numbered sequence)
- **`docs/design/servers/`** - Individual MCP server specs (one directory per server)
- **`docs/design/features/`** - User-facing capabilities (no numbering)
- **`docs/design/implementation/`** - Cross-cutting technical guidance (numbered sequence)
- **`docs/design/meta/`** - Process and project management docs

## Technology Stack

**Language:** Rust (chosen for performance, memory safety, and ecosystem compatibility)
**Protocol:** Model Context Protocol (MCP) for Claude integration
**Architecture:** Event-driven, microservices-style MCP servers
**Storage:** TBD - likely SQLite for local development, PostgreSQL for production

## Development Principles

### Code Quality Standards
- **Explicit error handling**: Use `Result<T, E>` types, avoid `unwrap()` in production code
- **Async throughout**: All I/O operations should be async for performance
- **Type safety**: Leverage Rust's type system to prevent runtime errors
- **Structured logging**: Use consistent logging for debugging and observability

### MCP Server Design Patterns
- **Single responsibility**: Each server handles one aspect of provenance
- **Event publishing**: Servers emit events that others can consume
- **Stateless when possible**: Minimize server-side state management
- **Graceful degradation**: Continue working even if some data sources are unavailable

### Testing Approach
- **Unit tests**: Core business logic and data transformations
- **Integration tests**: MCP protocol compliance and server interactions
- **End-to-end tests**: Full workflow validation with real Claude Code usage

## Workspace Structure

```
tapestry/                           # Root of monorepo
├── Cargo.toml                      # Workspace configuration
├── VISION.md                       # Project vision
├── CLAUDE.md                       # This file
├── docs/                           # All documentation
│   └── design/                     # Design specifications
├── crates/                         # Rust crates
│   ├── provenance-core/            # Shared types and utilities
│   ├── ai-interaction-logger/      # First MCP server
│   └── [other-servers]/            # Additional MCP servers
└── examples/                       # Usage examples and demos
```

## Current Priorities

### Immediate Next Steps (This Session)
1. **Set up Rust workspace**: Create `Cargo.toml` and initial crate structure
2. **Define core event model**: Create `docs/design/core/01-event-model.md` and implement in `provenance-core`
3. **Start AI Interaction Logger**: Basic MCP server scaffold with event capture

### Short-term Goals (Next Few Sessions)
- Complete AI Interaction Logger with full MCP interface
- Add file system monitoring capabilities
- Implement basic session boundary detection
- Create initial data storage and querying

## Key Design Decisions

### Event Sourcing for Provenance
**Decision:** Use event sourcing as the foundation for all provenance tracking
**Rationale:** Provides complete audit trail, enables time-travel debugging, and supports flexible querying
**Alternatives Considered:** CRUD-based approach, document storage
**Status:** Committed

### Rust for MCP Servers
**Decision:** Implement all MCP servers in Rust
**Rationale:** Performance for high-volume event processing, memory safety, strong type system
**Alternatives Considered:** TypeScript (easier development), Python (rapid prototyping)
**Status:** Committed

### Separate MCP Servers vs. Monolithic
**Decision:** Build multiple specialized MCP servers that compose together
**Rationale:** Better separation of concerns, independent scaling, clearer interfaces
**Alternatives Considered:** Single large MCP server with multiple tools
**Status:** Committed

## Integration Context

### MCP Protocol Usage
- **Servers as tools**: Each MCP server exposes tools for specific provenance capabilities
- **Event-driven coordination**: Servers communicate through shared event streams
- **Claude Code integration**: Designed to work seamlessly with Claude Code workflows

### Development Workflow Integration
- **Git hooks**: Capture commit-level provenance automatically
- **File watching**: Monitor development workspace for changes
- **Session tracking**: Understand development session boundaries and context

## Common Patterns and Conventions

### Error Handling
```rust
// Prefer Result types with descriptive errors
pub enum ProvenanceError {
    StorageError(String),
    SerializationError(String),
    NetworkError(String),
}

// Use ? operator for error propagation
pub fn process_event(event: Event) -> Result<(), ProvenanceError> {
    let stored = store_event(event)?;
    publish_event(stored)?;
    Ok(())
}
```

### Async Patterns
```rust
// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server startup
}

// Prefer async/await over callbacks
async fn handle_request(req: Request) -> Response {
    let result = process_async(req).await;
    create_response(result)
}
```

### MCP Server Structure
```rust
// Standard MCP server trait implementation
impl McpServer for ProvenanceServer {
    fn tools(&self) -> Vec<Tool> { ... }
    async fn call_tool(&self, name: &str, args: Value) -> Result<ToolResult> { ... }
}
```

## Research Questions

We're actively investigating these questions as we build:
- **What granularity of events provides the most value?** Too fine-grained creates noise, too coarse loses important context
- **How do we balance automation vs. human input?** Some decisions require human context that AI can't infer
- **What makes provenance data actionable?** Raw events aren't useful without good querying and presentation

## Success Metrics

We'll know we're succeeding when:
- **Development teams maintain context** across sessions and team member changes
- **AI assistance improves** through understanding of successful patterns
- **Architectural decisions are traceable** with full reasoning and alternatives
- **Code reviews leverage historical context** for better decision making

---

## Instructions for AI Assistants

When working on this project:

1. **Always reference the design docs** before making architectural decisions
2. **Follow the established patterns** for error handling, async usage, and MCP integration
3. **Create design documentation** for new components using the templates in `docs/design/meta/`
4. **Update this CLAUDE.md file** as the project evolves and new patterns emerge
5. **Consider provenance implications** - how will the code you're writing be tracked and understood?

The ultimate goal is building a platform that makes itself more effective by learning from its own development process.
