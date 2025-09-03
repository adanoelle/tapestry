# Contributing to Tapestry

Welcome! Tapestry is building practical MCP tools that enhance AI-assisted
development workflows.

## Quick Start

1. **Check the RFC process** - All features start with an RFC in
   `docs/design/features/`
2. **Follow conventions** - See `.claude/context/team-conventions.md`
3. **Test thoroughly** - 70% unit, 20% integration, 10% e2e
4. **Document everything** - Code should be self-documenting with good error
   messages

## Development Process

### Starting a New Tool

1. Write an RFC using the template in `.claude/templates/rfc-template.md`
2. Get it reviewed (self-review is fine initially)
3. Implement following hexagonal architecture
4. Add comprehensive tests
5. Update documentation

### Code Standards

We follow S-tier engineering practices from leading companies:

- **Stripe**: APIs are products - backward compatibility matters
- **Google**: Documentation lives with code
- **Anthropic**: Start simple, iterate based on usage

See `.claude/instructions.md` for detailed principles.

### Pull Request Process

1. **Branch naming**: `feat/tool-name`, `fix/issue`, `docs/topic`
2. **Commit messages**: Follow conventional commits format
3. **Tests**: All tests must pass
4. **Linting**: `cargo fmt` and `cargo clippy` clean
5. **Documentation**: Update relevant docs in same PR

### Architecture Guidelines

Every tool follows hexagonal architecture:

```
src/tools/{tool_name}/
├── domain.rs    # Pure business logic
├── port.rs      # Interface definitions
└── adapter.rs   # MCP implementation
```

Dependencies flow inward: Infrastructure → Application → Domain

### Testing Requirements

- Domain logic: Comprehensive unit tests
- Adapters: Integration tests with real dependencies
- End-to-end: Test full MCP protocol flow
- Performance: Benchmark critical paths

### Documentation Standards

- Every public API needs rustdoc with examples
- RFCs for all new tools/features
- Update CHANGELOG.md for all changes
- Keep `.claude/` context files current

## Getting Help

- Check existing RFCs in `docs/design/features/`
- Review `.claude/` directory for context
- Open an issue for questions

## License

By contributing, you agree that your contributions will be licensed under the
project's license.

---

Ready to build the future of AI-assisted development? Start with an RFC!
