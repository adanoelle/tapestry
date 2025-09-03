# Claude Code Instructions for Tapestry

## Project Overview

Tapestry is a monolithic collection of developer and Claude Code-centric MCP
tools built with Rust. We're building a comprehensive suite of tools that
enhance AI-assisted development workflows while maintaining S-tier engineering
standards.

## Core Principles (From S-Tier Companies)

### Stripe's Standards

- Treat every API as a product - backward compatibility is sacred
- Once code is written using our API, it should never need to change
- Measure everything but use metrics to improve systems, not judge people
- Help others succeed - your success depends on team success

### Anthropic's Philosophy

- Take an empirical approach - impact matters more than sophistication
- Start with the simplest solution and iterate
- Don't build a spaceship when a bicycle suffices
- High trust, low ego - assume good intentions

### Google's Engineering Excellence

- Documentation lives with code, not separate from it
- Every document should have a singular purpose
- A function should do one thing and do it well
- Trunk-based development with continuous integration

### Netflix's Approach

- Highly aligned, loosely coupled teams
- Farm for dissent - actively seek disagreement
- Context, not control - provide information, let people decide

## Tapestry Architectural Principles

### Hexagonal Architecture

- Domain logic knows nothing about infrastructure
- Dependencies flow inward only
- Ports define interfaces, adapters implement them
- Each MCP tool follows this pattern consistently

### Rust Patterns

```rust
// Always use Result for fallible operations
pub type TapestryResult<T> = Result<T, TapestryError>;

// Never panic in production
// Use async-first with Tokio
// Zero-copy operations where possible
// Builder pattern for complex configurations
```

### MCP Tool Standards

- Each tool is independent but follows shared patterns
- Tools communicate through well-defined interfaces
- Use rmcp crate for MCP protocol implementation
- Tools are discoverable and self-documenting

## Code Standards

### Naming Conventions

- Functions: `snake_case`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Module files: lowercase with underscores
- MCP tools: kebab-case (e.g., `auth-handler`, `code-reviewer`)

### Error Handling

```rust
// Errors must be actionable
#[error("Configuration required: Set {var} environment variable. See: {docs_url}")]
ConfigMissing { var: String, docs_url: String }

// Guide the developer/agent to the solution
#[error("Rate limit exceeded. Retry after {retry_after} seconds")]
RateLimit { retry_after: u64 }
```

### Documentation Requirements

- Every public API must have documentation with examples
- Document WHY, not just WHAT
- Include failure modes and recovery strategies
- Reference relevant RFCs for design decisions

## Development Workflow

### Before Writing Code

1. Check if an RFC exists in `docs/design/`
2. Understand acceptance criteria from the RFC
3. Review related architecture decisions
4. Check `CLAUDE.md` for project-specific patterns

### While Writing Code

1. Write tests first when possible (TDD)
2. Keep functions small and focused (< 50 lines ideal)
3. Comment complex logic with WHY, not WHAT
4. Use descriptive variable names over comments

### After Writing Code

1. Update documentation in the same PR
2. Ensure all tests pass (`cargo test`)
3. Run lints (`cargo clippy`)
4. Check formatting (`cargo fmt`)
5. Update CLAUDE.md if new patterns discovered

## Security Non-Negotiables

### Authentication & Authorization

- OAuth 2.0 for all external authentication
- JWT tokens with short expiration (15 minutes)
- Refresh tokens stored securely
- Principle of least privilege always

### Data Protection

- Never log sensitive data
- Encrypt data at rest and in transit
- Validate all external input
- Sanitize all output

### MCP-Specific Security

- Validate tool permissions before execution
- Rate limit all tool invocations
- Audit log all tool usage
- Sandbox untrusted tool execution

## Testing Philosophy

### Test Pyramid (Google's 70/20/10)

- 70% Unit tests (domain logic)
- 20% Integration tests (adapters)
- 10% End-to-end tests (full MCP flow)

### Test Standards

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test names describe behavior
    #[test]
    fn should_return_error_when_token_expired() {
        // Arrange
        // Act
        // Assert
    }
}
```

## Performance Guidelines

### Benchmarks

- Benchmark critical paths
- Target < 100ms P50 latency for tool execution
- < 500ms P99 latency
- Monitor memory usage per tool

### Optimization Rules

1. Measure first, optimize second
2. Optimize for clarity unless performance critical
3. Document why optimization was needed
4. Include benchmarks proving improvement

## AI Collaboration Rules

### When You're Uncertain

- Ask for clarification rather than assume
- Provide multiple options with trade-offs
- Reference similar patterns in the codebase
- Suggest simpler alternatives

### Code Generation

- Follow existing patterns in the codebase
- Maintain consistent style with surrounding code
- Generate comprehensive tests with the code
- Include error handling for all edge cases

### Review and Refactoring

- Explain the reasoning behind suggestions
- Prioritize security and correctness over cleverness
- Flag potential performance issues
- Suggest documentation improvements

## Tapestry-Specific Guidelines

### MCP Tool Creation

1. Each tool gets its own module under `src/tools/`
2. Follow the template in `.claude/templates/mcp-tool-template/`
3. Register in the tool registry (`src/registry/`)
4. Add integration tests in `tests/tools/`

### Documentation Structure

- RFCs go in `docs/design/features/`
- Implementation details in `docs/design/implementation/`
- Keep `docs/VISION.md` updated with project direction
- Update `CLAUDE.md` with new patterns

### Versioning

- Follow SemVer strictly
- Breaking changes require major version bump
- Document all changes in CHANGELOG.md
- Tag releases with comprehensive notes

## References

### Internal Documents

- `/docs/VISION.md` - Project vision and goals
- `/CLAUDE.md` - Quick reference for patterns
- `/docs/design/` - All RFCs and design docs

### External Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Stripe API Design](https://stripe.com/blog/api-design)

---

Remember: These principles come from companies that have scaled from tens to
thousands of engineers. They work. Trust the process.
