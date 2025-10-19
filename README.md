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

#### RFD CLI - Request for Discussion Manager
**Status**: ✅ Production Ready | **Type**: CLI Tool

A fast, agent-friendly CLI for managing technical design documents in the Oxide RFD format.

**Key Features**:
- ⚡ Fast: 1ms startup time, 2.4MB binary
- 🤖 Agent-friendly: JSON output, idempotent operations
- 📝 Complete workflow: Create, list, update, validate RFDs
- 🎨 Custom templates: Jinja2-based templating system
- ✅ Production-ready: 32 tests, comprehensive documentation

**Quick Start**:
```bash
cd cli/rfd
cargo build --release
./target/release/rfd create --title "My Proposal" --author "Me <me@example.com>"
```

**Documentation**: [README](cli/rfd/README.md) | [Architecture](cli/rfd/ARCHITECTURE.md) | [Examples](cli/rfd/examples/)

---

### In Development

#### Git Context Tool (MCP)
**Status**: 🔄 Paused | **Type**: MCP Tool

Rich git context for AI assistants using the Model Context Protocol.

- **Location**: [`mcp/git_workflow/`](mcp/git_workflow/)
- **RFC**: [RFC-001](docs/design/features/RFC-001-git-context-tool.md)
- **Note**: Paused to prioritize CLI tools first (Skills-first approach)

---

### Planned Tools

- **Code Review Tool** - Automated code review with AI insights
- **Test Generator** - Intelligent test generation based on code patterns
- **Documentation Generator** - Living documentation from code and comments

## 🏗️ Architecture

### Hybrid Architecture

Tapestry uses a **hybrid architecture** combining lightweight CLI tools with deep-integration MCP tools, orchestrated by Claude Code Skills.

**Three-Layer Design**:

```
Skills Layer (Claude Code)
    ↓ orchestrates ↓
CLI Tools ←→ MCP Tools
```

**CLI Tools** (`cli/*`):
- ⚡ Fast startup (< 10ms)
- 📁 Simple file operations
- 🤖 Agent-friendly (JSON output)
- 🔄 Stateless operations
- **Example**: RFD CLI

**MCP Tools** (`mcp/*`):
- 🔌 Deep IDE/editor integration
- 🔄 Complex stateful operations
- 📡 Real-time collaboration
- 🌐 Persistent connections
- **Example**: Git Context (planned)

**Skills** (`.claude/skills/`):
- 🎯 Orchestrate CLI + MCP tools
- 🧠 Multi-step workflows
- 📋 Context management
- **Example**: rfd-manager (planned)

### Tool Architecture Pattern

Each tool follows hexagonal architecture for testability:

```
src/
├── domain.rs    # Pure business logic (no external dependencies)
├── port.rs      # Interface definitions (traits)
└── adapter.rs   # CLI/MCP implementation (infrastructure)
```

**Key principles:**

- Dependencies flow inward (Infrastructure → Application → Domain)
- Domain logic has zero external dependencies
- Each tool is independent but shares common patterns
- Start simple (CLI) → Add complexity as needed (MCP)

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

### Phase 1: Foundation ✅ COMPLETE

- [x] Documentation structure
- [x] Architecture decisions
- [x] **RFD CLI tool** - Production ready! 🎉
  - [x] Core commands (create, list, show, status, update, validate)
  - [x] Template system with Jinja2
  - [x] JSON output for agents
  - [x] Comprehensive tests (32 tests)
  - [x] Full documentation (ARCHITECTURE.md, CONTRIBUTING.md, examples/)
  - [x] Performance optimization (2.4MB, 1ms startup)
- [ ] First MCP tool (Git Context) - paused
- [ ] Tool registry system
- [ ] Basic CI/CD

### Phase 2: Essential CLI Tools (Next)

- [ ] **RFD CLI enhancements**:
  - [ ] Full-text search across RFD content
  - [ ] Export to HTML/PDF
  - [ ] Git integration (auto-commit on changes)
  - [ ] GitHub issue integration
  - [ ] Dependency tracking between RFDs
- [ ] **New CLI tools**:
  - [ ] Code review tool
  - [ ] Test generation tool
  - [ ] Documentation generator

### Phase 3: MCP Tools & Intelligence

- [ ] Resume Git Context MCP tool
- [ ] Session memory tool
- [ ] Pattern recognition
- [ ] Cross-tool integration
- [ ] Learning system

## 🤝 Contributing

We follow S-tier engineering practices from companies like Stripe, Google, and
Anthropic.

1. **Start with an RFC** - All features begin with a design document
2. **Follow conventions** - Check `.claude/context/team-conventions.md`
3. **Test thoroughly** - 70% unit, 20% integration, 10% e2e
4. **Document everything** - Code should be self-documenting

See [CONTRIBUTING.md](docs/design/meta/CONTRIBUTING.md) for details.

## 📊 Performance Targets

### CLI Tools
- **Startup time**: < 10ms (RFD CLI: 1ms ✅)
- **Binary size**: < 3MB (RFD CLI: 2.4MB ✅)
- **Memory usage**: < 10MB (RFD CLI: ~5MB ✅)
- **Execution**: < 50ms for simple operations

### MCP Tools (Future)
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

**Status**: ✅ First CLI tool production-ready!
**Latest**: RFD CLI v0.1.0 - Complete with documentation and examples
**Current Focus**: Gathering feedback on RFD CLI and planning next tools
**Looking for**: Feature requests, use cases, and contributions

_Building the future of AI-assisted development, one tool at a time._
