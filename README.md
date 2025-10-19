# Tapestry

**AI-native development tools for modern software teams**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Nix](https://img.shields.io/badge/nix-flake-7EBAE4?logo=nixos&logoColor=white)](https://nixos.org/)
[![MCP](https://img.shields.io/badge/MCP-Protocol-5865F2)](https://modelcontextprotocol.io/)
[![CI](https://github.com/adanoelle/tapestry/actions/workflows/ci.yml/badge.svg)](https://github.com/adanoelle/tapestry/actions/workflows/ci.yml)
[![codecov](https://img.shields.io/codecov/c/github/adanoelle/tapestry?logo=codecov)](https://codecov.io/gh/adanoelle/tapestry)

---

## The Problem

AI assistants are transforming how we write code, but they need better tools.
Traditional development tools weren't designed for AI collaboration, creating
friction in AI-assisted workflows.

## The Solution

Tapestry provides a suite of purpose-built tools that enable seamless human-AI
collaboration through a hybrid architecture:

- **CLI Tools** for fast, lightweight operations
- **MCP Tools** for deep IDE integration
- **Skills** to orchestrate complex workflows

Built with Rust for performance and reliability. Designed for both humans and
AI.

---

## Why Tapestry?

### Fast by Design

Optimized for instant startup and minimal resource usage. RFD CLI launches in
1ms with a 2.4MB footprint.

### AI-First Architecture

Every tool provides structured output for AI consumption and accepts both human
and programmatic input.

### Production Quality

Comprehensive testing, cross-platform support, and S-tier engineering practices
from day one.

---

## Get Started

### Installation

**With Nix** (recommended):

```bash
git clone https://github.com/yourusername/tapestry.git
cd tapestry
nix develop
```

**Without Nix**:

```bash
git clone https://github.com/yourusername/tapestry.git
cd tapestry
./scripts/setup-dev.sh
```

[Complete installation guide →](docs/GETTING_STARTED.md)

### Quick Example

```bash
# Create a technical design document
rfd create --title "API Rate Limiting" --author "Jane Doe <jane@example.com>"

# List all RFDs
rfd list

# Update status
rfd status 0001 discussion
```

[RFD CLI documentation →](cli/rfd/README.md)

---

## Available Tools

### RFD CLI

**Manage technical design documents with speed and precision**

Request for Discussion (RFD) documents help teams make better technical
decisions through written proposals and collaborative review.

- Create, update, and track RFDs
- JSON output for AI workflows
- Custom templates with Jinja2
- Production-ready (v0.1.0)

[Learn more →](cli/rfd/README.md)

---

## Roadmap

**Now**: RFD CLI enhancements (search, export, GitHub integration)

**Next**: Code review and test generation tools

**Future**: MCP tools for git workflows and intelligent orchestration

[Full roadmap →](docs/ROADMAP.md)

---

## Documentation

- [Getting Started](docs/GETTING_STARTED.md) - Installation and first steps
- [Architecture](docs/ARCHITECTURE.md) - Technical deep-dive
- [Contributing](docs/design/meta/CONTRIBUTING.md) - How to contribute
- [Vision](docs/VISION.md) - Why Tapestry exists

[Documentation hub →](docs/README.md)

---

## Community

- [GitHub Discussions](https://github.com/yourusername/tapestry/discussions) -
  Questions and ideas
- [GitHub Issues](https://github.com/yourusername/tapestry/issues) - Bug reports
- [RFC Process](docs/design/meta/CONTRIBUTING.md#rfc-process) - Feature
  proposals

---

## License

[License details to be added]

## Acknowledgments

Built with inspiration from [Anthropic](https://anthropic.com),
[Stripe](https://stripe.com), [Google](https://google.com), and
[Oxide Computer](https://oxide.computer).

---

<sub>Tapestry is in active development. We welcome contributors and early
adopters.</sub>
