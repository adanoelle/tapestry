# Tapestry 🧵

**Practical MCP tools for supercharging AI-assisted development**

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![MCP](https://img.shields.io/badge/MCP-Protocol-blue?style=for-the-badge)](https://modelcontextprotocol.io/)

Tapestry is a collection of developer-centric MCP (Model Context Protocol) tools
that enhance AI-assisted development workflows. Built with Rust using hexagonal
architecture and S-tier engineering practices.

## 🎯 Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/tapestry.git
cd tapestry

# Build with cargo (coming soon)
cargo build --release

# Run tests
cargo test
```

## 🛠️ Tools

### Available Tools

_Coming soon - first tool in development_

### Planned Tools

- **Git Context Tool** - Rich git context for AI assistants
  ([RFC-001](docs/design/features/RFC-001-git-context-tool.md))
- **Code Review Tool** - Automated code review with AI insights
- **Test Generator** - Intelligent test generation based on code patterns
- **Documentation Generator** - Living documentation from code and comments

## 🏗️ Architecture

Each tool follows hexagonal architecture for maximum flexibility and
testability:

```
src/tools/{tool_name}/
├── domain.rs    # Pure business logic (no dependencies)
├── port.rs      # Interface definitions (traits)
└── adapter.rs   # MCP implementation (infrastructure)
```

**Key principles:**

- Dependencies flow inward (Infrastructure → Application → Domain)
- Domain logic has zero external dependencies
- Each tool is independent but shares common infrastructure
- Monolithic deployment for simplicity (can extract to microservices later)

## 📚 Documentation

- [Project Vision](docs/VISION.md) - Where we're heading
- [Architecture Decisions](.claude/knowledge/decisions/) - Why we built it this
  way
- [Team Conventions](.claude/context/team-conventions.md) - How we work
- [Contributing Guide](docs/design/meta/CONTRIBUTING.md) - How to contribute

## 🧠 AI-Native Development

This project embraces AI-assisted development with a special `.claude/`
directory containing:

- **instructions.md** - Core principles
- **context/** - Architecture, tech decisions, conventions
- **knowledge/** - Decision records and learnings
- **templates/** - RFC and tool templates
- **commands/** - Custom AI commands

This structure helps Claude Code and other AI assistants understand the codebase
deeply.

## 🚀 Roadmap

### Phase 1: Foundation (Current)

- [x] Documentation structure
- [x] Architecture decisions
- [ ] First MCP tool (Git Context)
- [ ] Tool registry system
- [ ] Basic CI/CD

### Phase 2: Essential Tools

- [ ] Code review tool
- [ ] Test generation tool
- [ ] Documentation generator
- [ ] Session memory tool

### Phase 3: Intelligence Layer

- [ ] Pattern recognition
- [ ] Cross-tool integration
- [ ] Learning system
- [ ] Provenance tracking

## 🤝 Contributing

We follow S-tier engineering practices from companies like Stripe, Google, and
Anthropic.

1. **Start with an RFC** - All features begin with a design document
2. **Follow conventions** - Check `.claude/context/team-conventions.md`
3. **Test thoroughly** - 70% unit, 20% integration, 10% e2e
4. **Document everything** - Code should be self-documenting

See [CONTRIBUTING.md](docs/design/meta/CONTRIBUTING.md) for details.

## 📊 Performance Targets

- **Tool execution**: P50 < 100ms, P99 < 500ms
- **Memory per tool**: < 10MB
- **Startup time**: < 1 second
- **Concurrent tools**: 1000+

## 🔒 Security

- OAuth 2.0 for authentication
- JWT with short expiration (15 min)
- Input validation on all external data
- Rate limiting per tool
- Audit logging for all operations

## 📄 License

[License details to be added]

## 🙏 Acknowledgments

Built with inspiration from:

- [Anthropic](https://anthropic.com) for MCP and Claude
- [Stripe](https://stripe.com) for API design principles
- [Google](https://google.com) for engineering practices
- The Rust community for excellent tooling

---

**Status**: 🚧 Under active development  
**Current Focus**: Implementing first MCP tool (Git Context)  
**Looking for**: Feedback on tool ideas and use cases

_Building the future of AI-assisted development, one tool at a time._
