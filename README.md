# Tapestry 🧶

**Development Provenance Platform for AI-Assisted Coding**

Tapestry transforms AI-assisted software development from a black box into a transparent, learnable process. By capturing the complete context of human-AI collaboration during coding sessions, it creates institutional memory that makes development teams more effective over time.

## Overview

Modern software development increasingly relies on AI assistants like Claude Code, but this creates new challenges: teams lose track of architectural decisions, knowledge disappears when developers leave, and the reasoning behind AI suggestions remains invisible. Tapestry solves these problems by building a comprehensive provenance system that captures not just *what* code was written, but *why* and *how* development decisions were made.

## Key Features

- **🔍 Complete Decision Tracking** - Capture the reasoning behind every architectural choice, including alternatives considered and trade-offs made
- **🧠 Institutional Memory** - Build persistent knowledge that survives team changes and project handovers  
- **🤝 Human-AI Collaboration Intelligence** - Understand how AI assistance influences development decisions and outcomes
- **📊 Pattern Recognition** - Learn which development approaches work best for different types of problems
- **📚 Living Documentation** - Automatically generate and maintain documentation that reflects actual development practices
- **🔄 Cross-Session Context** - Maintain development context and continue conversations across multiple coding sessions

## Architecture

Tapestry is built as a collection of specialized MCP (Model Context Protocol) servers that work together to create comprehensive development intelligence:

### Core Components

- **Provenance Tracker** - Central hub that orchestrates data collection from all sources
- **AI Interaction Logger** - Captures Claude Code interactions with full reasoning context
- **File System Monitor** - Tracks code changes with semantic analysis and classification  
- **Git Intelligence** - Enhanced commit analysis and repository pattern recognition
- **Decision Graph** - Structures architectural decisions and tracks their outcomes over time
- **Pattern Recognition** - Identifies successful development patterns and anti-patterns
- **Context Bridge** - Maintains session continuity and intelligent context restoration

### Integration

All components integrate seamlessly with [Claude Code](https://docs.anthropic.com/en/docs/claude-code) through the Model Context Protocol, enabling:

- Automatic instrumentation of development sessions
- Real-time decision capture during AI-assisted coding  
- Context-aware suggestions based on project history
- Cross-session memory and pattern learning

## Quick Start

> **Note:** Tapestry is currently in early development. This section will be updated as we build the initial implementation.

```bash
# Clone the repository
git clone https://github.com/yourusername/tapestry.git
cd tapestry

# Install dependencies (Rust required)
cargo build

# Configure MCP servers with Claude Code
claude mcp add tapestry-logger ./target/release/tapestry-logger
claude mcp add tapestry-tracker ./target/release/tapestry-tracker
```

## Development Status

🚧 **Early Development Phase** - We're currently building the foundational components:

- [x] Project vision and architecture design
- [x] Documentation structure and standards
- [ ] Core event model and data structures  
- [ ] AI Interaction Logger MCP server
- [ ] File System Monitor implementation
- [ ] Basic provenance data collection

See our [Roadmap](docs/VISION.md#implementation-roadmap) for detailed development phases.

## Documentation

- **[Vision Document](docs/VISION.md)** - Complete project vision, goals, and long-term impact
- **[Design Documentation](docs/design/README.md)** - Technical specifications and architecture details
- **[Contributing Guide](docs/design/meta/contributing.md)** - How to contribute to the platform *(coming soon)*

### For Contributors

- **[Design Documentation Guide](docs/design/meta/design-documentation-guide.md)** - Templates and standards for creating design documents
- **[File Structure Reference](docs/design/meta/file-structure-reference.md)** - Complete guide to documentation organization

## Technology Stack

- **Language:** Rust - for performance, memory safety, and robust error handling
- **Protocol:** Model Context Protocol (MCP) - native integration with Claude Code
- **Architecture:** Event-driven microservices with composable intelligence
- **Storage:** Event sourcing with pluggable storage backends

## Use Cases

### For Development Teams
- **Onboard new developers faster** with complete project context and decision history
- **Maintain architectural consistency** through understanding of past decisions
- **Improve code reviews** with historical context about similar changes
- **Learn from successful patterns** and avoid repeating costly mistakes

### For Individual Developers  
- **Resume work efficiently** with full context from previous sessions
- **Understand legacy codebases** through traced decision history
- **Make informed architectural choices** based on similar past decisions
- **Collaborate better with AI** through improved context sharing

### For Engineering Leaders
- **Track architectural evolution** and decision-making patterns over time
- **Identify knowledge gaps** and risks when team members leave
- **Measure development velocity** and identify optimization opportunities  
- **Build data-driven processes** based on actual development patterns

## Research Vision

Tapestry isn't just a development tool - it's a research platform for understanding effective human-AI collaboration in software development. By capturing high-fidelity data about development processes, we aim to answer questions like:

- What documentation patterns correlate with effective AI assistance?
- How do successful teams naturally evolve their development practices?
- Which types of AI suggestions lead to better long-term architectural outcomes?
- How can development provenance improve software quality and team productivity?

## Contributing

We welcome contributions from developers, researchers, and anyone interested in improving AI-assisted development workflows. 

**Ways to contribute:**
- 🛠️ **Implementation** - Help build MCP servers and core components
- 📝 **Documentation** - Improve guides, examples, and specifications  
- 🔬 **Research** - Investigate questions about development provenance and AI collaboration
- 🐛 **Testing** - Try Tapestry with your development workflows and provide feedback

See our [Contributing Guide](docs/design/meta/contributing.md) *(coming soon)* for detailed information.

## Community

- **Discussions** - Share ideas and ask questions in [GitHub Discussions](https://github.com/yourusername/tapestry/discussions)
- **Issues** - Report bugs and request features in [GitHub Issues](https://github.com/yourusername/tapestry/issues)
- **Discord** - Join our development community *(coming soon)*

## License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details.

## Acknowledgments

- [Anthropic](https://anthropic.com) for Claude Code and the Model Context Protocol
- The broader AI-assisted development community for inspiration and feedback
- Contributors and early adopters who help shape this platform

---

**Status:** Early Development | **Version:** 0.1.0-alpha | **Last Updated:** January 2025
