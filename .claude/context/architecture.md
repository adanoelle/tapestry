# Tapestry Architecture

## Overview

Tapestry is a monolithic suite of MCP (Model Context Protocol) tools designed to
enhance AI-assisted development workflows. Each tool within Tapestry follows
hexagonal architecture while the overall system maintains monolithic simplicity
for easier deployment and management.

## Architectural Pattern: Hexagonal Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Infrastructure Layer                 │
│  ┌──────────────────────────────────────────────────┐   │
│  │                 Application Layer                │   │
│  │  ┌──────────────────────────────────────────┐    │   │
│  │  │           Domain Layer (Core)            │    │   │
│  │  │                                          │    │   │
│  │  │    • Business Logic                      │    │   │
│  │  │    • Domain Entities                     │    │   │
│  │  │    • Business Rules                      │    │   │
│  │  └──────────────────────────────────────────┘    │   │
│  │                                                  │   │
│  │    • Use Cases                                   │   │
│  │    • Port Definitions (Interfaces)               │   │
│  │    • Application Services                        │   │
│  └──────────────────────────────────────────────────┘   │
│                                                         │
│  • MCP Protocol Adapter (rmcp)                          │
│  • Database Adapters                                    │
│  • External API Adapters                                │
│  • CLI Adapter                                          │
└─────────────────────────────────────────────────────────┘

Dependencies flow: Infrastructure → Application → Domain
```

## Core Components

### 1. Tool Registry

- Central registry for all MCP tools
- Handles tool discovery and metadata
- Manages tool lifecycle
- Provides tool introspection

### 2. MCP Adapter

- Built on `rmcp` crate
- Handles protocol communication
- Manages tool registration with MCP
- Provides error handling and retry logic

### 3. Individual Tools

Each tool is a self-contained module with:

- Domain logic specific to its function
- Own set of ports and adapters
- Independent testing
- Tool-specific configuration

### 4. Shared Infrastructure

- Common authentication/authorization
- Shared rate limiting
- Centralized logging and metrics
- Configuration management

## Design Decisions

### Why Monolithic?

1. **Simplified Deployment**: One binary to deploy and manage
2. **Shared Infrastructure**: Reuse authentication, logging, metrics
3. **Easier Testing**: Test full workflows locally
4. **Performance**: No network overhead between tools
5. **Follows YAGNI**: Don't need microservices complexity yet

### Why Hexagonal Architecture?

1. **Testability**: Domain logic tested without infrastructure
2. **Flexibility**: Easy to swap adapters (e.g., different databases)
3. **Clear Boundaries**: Explicit separation of concerns
4. **Parallel Development**: Teams can work on different layers
5. **Future-Proof**: Can extract to microservices if needed

### Technology Choices

| Component      | Technology          | Rationale                               |
| -------------- | ------------------- | --------------------------------------- |
| Language       | Rust                | Performance, safety, good async support |
| Async Runtime  | Tokio               | Industry standard, mature ecosystem     |
| MCP Library    | rmcp                | Official Rust SDK for MCP               |
| Error Handling | thiserror + anyhow  | Type-safe errors with good ergonomics   |
| Serialization  | serde + serde_json  | De facto standard, MCP uses JSON        |
| Testing        | built-in + proptest | Property-based testing for robustness   |
| Logging        | tracing             | Structured, async-aware logging         |
| Metrics        | prometheus          | Industry standard, good ecosystem       |

## Tool Architecture Pattern

Each MCP tool follows this structure:

```rust
// Domain Layer
pub struct ToolDomain {
    // Core business logic
}

// Port (Interface)
#[async_trait]
pub trait ToolPort {
    async fn execute(&self, input: Input) -> Result<Output>;
}

// Application Service
pub struct ToolUseCase<P: ToolPort> {
    port: P,
}

// MCP Adapter
pub struct ToolMcpAdapter {
    use_case: ToolUseCase<ConcreteAdapter>,
}

impl McpTool for ToolMcpAdapter {
    // MCP protocol implementation
}
```

## Scalability Path

Current: **Monolith** (0-100 tools, 1-10 developers)

- Single repository
- Single deployment
- Shared infrastructure

Future Option 1: **Modular Monolith** (100-500 tools, 10-50 developers)

- Separate modules with clear interfaces
- Still single deployment
- Can run tools independently for testing

Future Option 2: **Microservices** (500+ tools, 50+ developers)

- Extract high-traffic tools to separate services
- Use current ports as service boundaries
- Gradual migration tool by tool

## Security Architecture

### Authentication Flow

```
Client → MCP Adapter → Auth Middleware → Tool
                ↓
            OAuth Provider
```

### Authorization Model

- Role-Based Access Control (RBAC)
- Per-tool permissions
- Audit logging for all tool invocations

## Performance Targets

| Metric              | Target  | Current |
| ------------------- | ------- | ------- |
| Tool Discovery      | < 10ms  | TBD     |
| Tool Invocation P50 | < 100ms | TBD     |
| Tool Invocation P99 | < 500ms | TBD     |
| Concurrent Tools    | 1000+   | TBD     |
| Memory per Tool     | < 10MB  | TBD     |

## Open Architectural Questions

1. **State Management**: Should tools share state or be completely isolated?
2. **Caching Strategy**: Where and what to cache?
3. **Rate Limiting**: Per-user, per-tool, or both?
4. **Versioning**: How to handle multiple versions of the same tool?
5. **Hot Reload**: Can we reload tools without restarting?

## References

- [ADR-001: Choosing Hexagonal Architecture](../knowledge/decisions/ADR-001-hexagonal-architecture.md)
- [ADR-002: Monolith First Approach](../knowledge/decisions/ADR-002-monolith-first.md)
- [RFC-001: Tapestry Vision](../../docs/VISION.md)
