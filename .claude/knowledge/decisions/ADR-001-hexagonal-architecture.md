# ADR-001: Use Hexagonal Architecture for Tapestry

## Status

**Accepted** (2024-01-15)

## Context

We are building Tapestry, a monolithic suite of MCP (Model Context Protocol)
tools for AI-assisted development. We need an architectural pattern that:

1. **Supports AI-Assisted Development**: Claude Code and other AI assistants
   need to understand and work with our codebase effectively
2. **Enables Independent Testing**: We need to test business logic without
   external dependencies
3. **Allows Future Scaling**: While starting monolithic, we may need to extract
   microservices later
4. **Maintains Clear Boundaries**: Different concerns should be clearly
   separated
5. **Supports Multiple Tools**: Each MCP tool should be self-contained yet share
   infrastructure

We researched practices from S-tier engineering organizations:

- **Stripe**: Uses service-oriented architecture with clear API boundaries
- **Google**: Employs strict layering and interface definitions
- **Uber**: Evolved from monolith to microservices via Domain-Oriented
  Microservice Architecture (DOMA)
- **Netflix**: Pioneered microservices with clear service boundaries
- **Anthropic**: Emphasizes simplicity and clear separation of concerns

## Decision

We will use **Hexagonal Architecture** (also known as Ports and Adapters) as our
primary architectural pattern.

### Structure

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
```

### Key Principles

1. **Dependencies Point Inward**: Outer layers depend on inner layers, never the
   reverse
2. **Domain Independence**: The domain layer has zero external dependencies
3. **Ports Define Contracts**: Interfaces (traits in Rust) define how layers
   communicate
4. **Adapters Implement Details**: Technical details live in adapters
5. **Use Cases Orchestrate**: Application layer coordinates between domain and
   infrastructure

## Consequences

### Positive

✅ **Testability**: Domain logic can be tested without any infrastructure,
databases, or external services. This enables fast unit tests and TDD.

✅ **Flexibility**: We can swap implementations easily. Switch from PostgreSQL
to MongoDB? Just write a new adapter. Change from REST to GraphQL? New adapter,
same domain logic.

✅ **AI-Friendly**: Clear boundaries and explicit interfaces make it easier for
Claude Code and other AI assistants to understand the codebase structure and
generate appropriate code.

✅ **Future-Proof**: When we need to extract microservices, each hexagon can
become a service. The ports become API contracts.

✅ **Clear Ownership**: Each layer has clear responsibilities. Domain experts
work on domain logic, infrastructure experts on adapters.

✅ **Parallel Development**: Teams can work on different layers simultaneously
with minimal coordination.

✅ **Technology Agnostic**: The core business logic doesn't care if we use Rust,
Python, or any other language for adapters.

### Negative

❌ **Initial Complexity**: More boilerplate code initially. Every feature needs
domain models, ports, use cases, and adapters.

❌ **Learning Curve**: Team members need to understand the pattern. New
developers might find it overwhelming initially.

❌ **Indirection**: More layers mean more files to navigate. Simple operations
might feel over-engineered.

❌ **Performance Overhead**: Additional abstraction layers could impact
performance (though Rust's zero-cost abstractions minimize this).

### Neutral

➖ **More Files**: Each tool will have multiple files (domain.rs, port.rs,
adapter.rs) instead of a single file.

➖ **Explicit Contracts**: All interactions between layers must be explicitly
defined through interfaces.

➖ **Naming Conventions**: Need consistent naming patterns across all layers.

## Alternatives Considered

### Alternative 1: Clean Architecture

Clean Architecture adds more layers (Entities, Use Cases, Interface Adapters,
Frameworks) with similar separation principles.

**Why not chosen**:

- More complex than necessary for our needs
- Hexagonal is simpler and more focused
- Clean Architecture's additional layers don't provide value for our use case
- Hexagonal better matches MCP's tool-based model

### Alternative 2: Traditional Layered Architecture

Simple three-layer architecture: Presentation → Business → Data.

**Why not chosen**:

- Tends to leak database concerns into business logic
- Harder to test business logic in isolation
- Less flexible for adding new adapters
- Common in enterprise but leads to coupling

### Alternative 3: Monolithic Ball of Mud

No explicit architecture, organic growth.

**Why not chosen**:

- Becomes unmaintainable quickly
- Impossible to test properly
- AI assistants struggle with unclear boundaries
- No path to microservices
- Against S-tier company practices

### Alternative 4: Microservices from Start

Each MCP tool as a separate service.

**Why not chosen**:

- Premature optimization
- Operational complexity
- Network overhead between tools
- Harder to maintain consistency
- Goes against "start simple" principle from Anthropic

## Implementation Notes

### For Rust

```rust
// Domain Layer - Pure business logic
pub struct ToolDomain {
    // No external dependencies
}

// Port - Interface definition
#[async_trait]
pub trait ToolPort {
    async fn execute(&self, input: Input) -> Result<Output>;
}

// Application Layer - Use case
pub struct ToolUseCase<P: ToolPort> {
    port: P,
}

// Infrastructure Layer - Adapter
pub struct McpAdapter {
    use_case: ToolUseCase<ConcreteAdapter>,
}
```

### Directory Structure

```
src/
├── domain/           # Core business logic
├── application/      # Use cases and ports
├── infrastructure/   # Adapters and external concerns
└── tools/           # Individual MCP tools
    └── example_tool/
        ├── domain.rs
        ├── port.rs
        └── adapter.rs
```

## Migration Strategy

If we need to migrate to microservices later:

1. **Identify Boundaries**: Each tool is already a natural boundary
2. **Extract Domain**: Move domain + application layers to shared library
3. **Create Service**: Wrap hexagon in service scaffolding
4. **Convert Ports to APIs**: Ports become REST/gRPC endpoints
5. **Deploy Independently**: Each hexagon becomes a deployable service

## Validation

We will validate this decision by:

1. **Ease of Testing**: Can we write unit tests without mocking databases?
2. **AI Comprehension**: Can Claude Code understand and generate code following
   this pattern?
3. **Development Speed**: After initial setup, is development faster?
4. **Tool Independence**: Can we develop tools in parallel without conflicts?
5. **Performance**: Do the abstractions impact performance significantly?

## References

- [Alistair Cockburn's Original Hexagonal Architecture Article](https://alistair.cockburn.us/hexagonal-architecture/)
- [Ports and Adapters Pattern](https://herbertograca.com/2017/11/16/explicit-architecture-01-ddd-hexagonal-onion-clean-cqrs-how-i-put-it-all-together/)
- [Netflix's Architecture Evolution](https://netflixtechblog.com/tagged/architecture)
- [Uber's Domain-Oriented Microservice Architecture](https://eng.uber.com/microservice-architecture/)
- [Google's API Design Guide](https://cloud.google.com/apis/design)
- [Stripe's API Design](https://stripe.com/blog/api-design)

## Review History

- **2024-01-15**: Initial draft created based on S-tier company research
- **2024-01-15**: Accepted for Tapestry project

## Notes

This decision aligns with Anthropic's principle: "Start with the simplest
solution and iterate." Hexagonal architecture is simple enough to start with but
sophisticated enough to scale. It's a bicycle that can become a spaceship if
needed.
