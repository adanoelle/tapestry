# Tapestry Technical Decisions

## Core Technology Stack

### Language: Rust

**Why Rust?**

- **Memory Safety**: No null pointer dereferences, no data races
- **Performance**: Zero-cost abstractions, no garbage collector
- **Ecosystem**: Strong async support with Tokio, excellent tooling
- **Type System**: Catches errors at compile time
- **MCP Support**: Official rmcp SDK available

**Trade-offs Accepted**:

- Steeper learning curve
- Longer initial development time
- Verbose error handling (mitigated by good patterns)

### Architecture: Hexagonal (Ports & Adapters)

**Why Hexagonal?**

- **Testability**: Pure domain logic without infrastructure
- **Flexibility**: Easy to swap implementations
- **Clear Boundaries**: Explicit separation of concerns
- **Proven Pattern**: Successfully used at Stripe, Uber
- **Future-Proof**: Easy to extract to microservices

**Why Not Clean Architecture?**

- Hexagonal is simpler and more focused
- Clean Architecture adds unnecessary layers for our needs
- Hexagonal better matches MCP's tool model

### Monolithic Deployment

**Why Monolithic First?**

- **Simplicity**: One binary to deploy and manage
- **Performance**: No network overhead between tools
- **Shared Infrastructure**: Reuse auth, logging, metrics
- **YAGNI**: We don't need microservices complexity yet
- **Proven Path**: Netflix, Uber, all started monolithic

**Migration Path**: When we need to split (>100 tools or >50 developers):

1. Domain boundaries already defined (hexagonal)
2. Tools already isolated
3. Extract high-traffic tools first
4. Gradual migration

## Infrastructure Decisions

### Async Runtime: Tokio

**Why Tokio?**

- Industry standard for Rust async
- Mature ecosystem
- Excellent performance
- Good debugging tools
- Required by rmcp

**Configuration**:

```toml
tokio = { version = "1.35", features = ["full"] }
```

### MCP Implementation: rmcp

**Why rmcp?**

- Official Rust SDK
- Actively maintained
- Good documentation
- Supports all MCP features
- Clean API design

**Version**: 0.3.2 (latest stable)

### Error Handling: thiserror + anyhow

**Strategy**:

- `thiserror` for library code (domain layer)
  - Type-safe error enums
  - Clear error semantics
- `anyhow` for application code
  - Context chain for debugging
  - Ergonomic error handling

**Pattern**:

```rust
// Domain layer (thiserror)
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Not found: {0}")]
    NotFound(String),
}

// Application layer (anyhow)
use anyhow::{Context, Result};
let config = load_config()
    .context("Failed to load configuration")?;
```

### Serialization: serde

**Why serde?**

- De facto standard in Rust
- Excellent performance
- Derive macros reduce boilerplate
- Required by MCP (JSON)
- Supports many formats

### Logging: tracing

**Why tracing over log?**

- Structured logging
- Async-aware
- Span-based context
- Better for distributed systems
- Easy integration with observability platforms

**Configuration**:

```rust
tracing_subscriber::fmt()
    .with_env_filter("tapestry=debug,rmcp=info")
    .init();
```

### Testing: Built-in + proptest

**Strategy**:

- Built-in for unit tests
- `proptest` for property-based testing
- `criterion` for benchmarks
- `tokio::test` for async tests

**Why proptest?**

- Catches edge cases humans miss
- Great for parsing and validation
- Generates test cases automatically

## Development Environment

### Package Manager: Nix

**Why Nix?**

- Reproducible builds
- Declarative environment
- No "works on my machine"
- Easy CI/CD integration
- Rollback capability

### Version Control: Git

**Workflow**:

- Trunk-based development (like Google)
- Feature flags for incomplete features
- Small, frequent commits
- Squash merge to main

### Documentation: Markdown + Rust Docs

**Strategy**:

- Markdown for high-level docs (RFCs, guides)
- Rust doc comments for API documentation
- Examples in doc tests
- Architecture diagrams in Mermaid

## Security Decisions

### Authentication: OAuth 2.0

**Why OAuth?**

- Industry standard
- Well-understood security model
- Good library support
- Required by MCP spec for remote servers
- Supports various flows

### Secrets Management: Environment Variables

**Current** (Development):

- Environment variables
- `.env` files (never committed)

**Future** (Production):

- HashiCorp Vault or similar
- Kubernetes secrets
- Cloud provider secret manager

### Rate Limiting: Token Bucket

**Why Token Bucket?**

- Simple to implement
- Fair burst handling
- Well-understood algorithm
- Per-tool configuration

## Performance Targets

### Latency

- P50: < 100ms
- P99: < 500ms
- Tool discovery: < 10ms

### Throughput

- 1000+ concurrent tool executions
- 10,000+ requests per second

### Resource Usage

- Memory per tool: < 10MB
- Startup time: < 1 second
- Binary size: < 50MB

## Observability

### Metrics: Prometheus

**Why Prometheus?**

- Industry standard
- Great Rust support
- Pull-based model
- Powerful query language
- Grafana integration

### Tracing: OpenTelemetry

**Why OpenTelemetry?**

- Vendor-neutral
- Comprehensive tracing
- Automatic instrumentation
- Correlation with logs

### Dashboards: Grafana

**Why Grafana?**

- Works well with Prometheus
- Flexible dashboards
- Alert management
- Open source

## Future Considerations

### Not Decided Yet

1. **Database**:
   - PostgreSQL likely (if needed)
   - SQLite for embedded use cases
2. **Message Queue**:

   - Not needed yet
   - Redis or NATS when needed

3. **Container Orchestration**:

   - Docker for packaging
   - Kubernetes when we need orchestration

4. **CI/CD Platform**:
   - GitHub Actions likely
   - Self-hosted runners for security

### Evaluation Criteria

When making future technical decisions:

1. **Simplicity First**: Does this add necessary complexity?
2. **Performance**: Will this meet our latency targets?
3. **Maintenance**: Can the team maintain this?
4. **Security**: Does this improve or compromise security?
5. **Cost**: Is the benefit worth the cost?
6. **Reversibility**: Can we change this decision later?

## Rejected Alternatives

### Language: Python

- **Why Considered**: Faster development, AI/ML ecosystem
- **Why Rejected**: Performance concerns, type safety

### Language: TypeScript

- **Why Considered**: Familiar to many, good ecosystem
- **Why Rejected**: Performance, deployment complexity

### Architecture: Microservices First

- **Why Considered**: Modern, scalable
- **Why Rejected**: Unnecessary complexity initially

### MCP: Custom Protocol

- **Why Considered**: Full control
- **Why Rejected**: Reinventing the wheel, compatibility

---

_Technical decisions are living choices. Propose changes via RFC when
assumptions change._
