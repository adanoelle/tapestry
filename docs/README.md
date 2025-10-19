# Tapestry Documentation

Welcome to the Tapestry documentation. This guide will help you understand, use,
and contribute to Tapestry's suite of AI-native development tools.

## Getting Started

### New to Tapestry?

- **[Getting Started Guide](GETTING_STARTED.md)** - Installation, setup, and
  your first steps
- **[Project Vision](VISION.md)** - Understanding what Tapestry is and why it
  exists
- **[Architecture Overview](ARCHITECTURE.md)** - How Tapestry's hybrid
  architecture works

### Using Tapestry

- **[RFD CLI Guide](../cli/rfd/README.md)** - Complete guide to the RFD CLI tool
- **[Claude Code Integration](CLAUDE_CODE_INTEGRATION.md)** - How we keep Claude
  Code current
- **[CI/CD Guide](CI_CD.md)** - Continuous integration, releases, and git hooks

## Contributing

### For Contributors

- **[Contributing Guide](design/meta/CONTRIBUTING.md)** - How to contribute to
  Tapestry
- **[Team Conventions](.claude/context/team-conventions.md)** - Coding standards
  and practices
- **[Roadmap](ROADMAP.md)** - What we're building next

### For Developers

- **[Architecture Deep-Dive](ARCHITECTURE.md)** - Hexagonal architecture and
  design patterns
- **[Tech Decisions](.claude/context/tech-decisions.md)** - Why we made certain
  choices
- **[RFC Process](design/meta/CONTRIBUTING.md#rfc-process)** - How to propose
  new features

## Tools Documentation

### Available Tools

- **[RFD CLI](../cli/rfd/README.md)** - Request for Discussion document manager
  - [Architecture](../cli/rfd/ARCHITECTURE.md)
  - [Contributing](../cli/rfd/CONTRIBUTING.md)
  - [Examples](../cli/rfd/examples/)

### Planned Tools

- **Git Context Tool (MCP)** -
  [RFC-001](design/features/RFC-001-git-context-tool.md)
- **Code Review Tool** - Coming soon
- **Test Generator** - Coming soon

## Reference

### Architecture

- **[Hybrid Architecture](ARCHITECTURE.md#hybrid-architecture)** - CLI + MCP +
  Skills design
- **[Tool Patterns](ARCHITECTURE.md#tool-patterns)** - How to build Tapestry
  tools
- **[Performance Targets](ARCHITECTURE.md#performance-targets)** - Benchmarks
  and goals

### Operations

- **[CI/CD](CI_CD.md)** - Build, test, and release workflows
- **[Security](ARCHITECTURE.md#security)** - Security practices and
  considerations
- **[Deployment](GETTING_STARTED.md#deployment)** - Installing and distributing
  tools

### Community

- **[Roadmap](ROADMAP.md)** - Current and future work
- **[Design RFCs](design/features/)** - Feature proposals and decisions
- **[Acknowledgments](ARCHITECTURE.md#acknowledgments)** - Inspiration and
  credits

## Quick Links

| I want to...                | Go to...                                                         |
| --------------------------- | ---------------------------------------------------------------- |
| Install Tapestry            | [Getting Started](GETTING_STARTED.md)                            |
| Use the RFD CLI             | [RFD CLI Guide](../cli/rfd/README.md)                            |
| Understand the architecture | [Architecture](ARCHITECTURE.md)                                  |
| Contribute code             | [Contributing Guide](design/meta/CONTRIBUTING.md)                |
| Propose a feature           | [RFC Process](design/meta/CONTRIBUTING.md#rfc-process)           |
| See what's planned          | [Roadmap](ROADMAP.md)                                            |
| Report a bug                | [GitHub Issues](https://github.com/yourusername/tapestry/issues) |

## AI-Assisted Development

Tapestry is built with AI assistance in mind. The `.claude/` directory contains:

- **[Instructions](.claude/instructions.md)** - Core principles for AI
  assistants
- **[Context](.claude/context/)** - Architecture, conventions, and decisions
- **[Templates](.claude/templates/)** - RFC and tool templates
- **[Commands](.claude/commands/)** - Custom AI commands

This structure helps Claude Code and other AI assistants understand and
contribute to the codebase effectively.

## Need Help?

- **Questions?** Open a
  [discussion](https://github.com/yourusername/tapestry/discussions)
- **Bug report?** Create an
  [issue](https://github.com/yourusername/tapestry/issues)
- **Want to contribute?** Read the
  [contributing guide](design/meta/CONTRIBUTING.md)

---

**Tip**: Use this page as your navigation hub. Bookmark it for quick access to
all Tapestry documentation.
