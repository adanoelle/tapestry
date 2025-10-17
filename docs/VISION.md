# Tapestry: AI-Native Development Tools

## Vision

Build a suite of AI-native development tools that supercharge AI-assisted
workflows through a hybrid approach: lightweight CLI tools for Skills,
deep-integration MCP tools for complex operations, and Skills that orchestrate
both. Start simple, iterate based on real-world usage.

## Mission

Create practical, high-quality AI-native tools that:

- Solve real development pain points with the right tool for each job
- Learn from actual usage patterns to inform architecture decisions
- Demonstrate S-tier engineering practices across multiple paradigms
- Serve as a learning platform for Skills, CLI tools, and MCP integration

## Core Philosophy

### Start Simple, Iterate

Following Anthropic's principle: "Don't build a spaceship when a bicycle
suffices." Each tool should:

- Solve one problem well
- Ship when useful, not perfect
- Evolve based on feedback
- Maintain backward compatibility

### Developer-First

Every tool must:

- Provide immediate value
- Be simple to understand
- Have excellent error messages
- Work reliably in real development scenarios

### Learning Platform

This project serves as:

- A practical way to master MCP and AI-assisted development
- A portfolio demonstrating engineering excellence
- A foundation for joining teams building the future of development

## Initial Focus Areas

### Phase 1: Skills-First Foundation (Current)

**Strategy**: Build lightweight CLI tools first to validate the Skills paradigm,
then invest in heavier MCP tools once we understand the limitations.

**Current Tools**:

- **RFD CLI** (🚧 In Development): Documentation management for RFDs, RFCs, ADRs
  - Agent-friendly (JSON output, idempotent operations, structured errors)
  - Fast startup (< 10ms), single binary distribution
  - Template-based document generation with Jinja2
- **git-workflow MCP** (⏸️ Paused): Conventional commits and change analysis
  - Solid hexagonal architecture for future reference
  - Will resume after Skills validation

**Next Tools** (Choose based on Skills learnings):

- Test generation and validation (CLI or MCP?)
- Code review and analysis (likely MCP for deep integration)
- Issue-to-RFD workflow (gh CLI integration)

### Phase 2: Hybrid Maturity

Once Skills approach is validated, expand strategically:

**CLI Tools** (for Skills):

- Documentation generators (RFDs, specs, READMEs)
- Report formatters (test results, metrics, changelogs)
- Quick validation tools (linting, format checking)

**MCP Tools** (for deep integration):

- Pattern recognition in codebases
- Architectural decision tracking
- Code analysis and refactoring
- Development session memory

**Skills** (orchestration layer):

- Compose CLI + MCP tools for sophisticated workflows
- Encode team conventions and best practices
- Guide Claude through complex multi-step tasks

### Phase 3: Intelligent Platform

Eventually compose into a cohesive platform:

- Full development history tracking
- AI-human collaboration insights
- Institutional knowledge preservation
- Team productivity analytics
- Seamless integration across all tool types

## Success Metrics

### Short-term (3 months)

- 3-5 working CLI tools used daily (RFD, test runner, formatter)
- 5+ Skills orchestrating workflows
- RFD tool dogfooded for all Tapestry documentation
- 1-2 MCP tools demonstrating deep integration
- Measurable productivity improvements
- Clean, maintainable codebase

### Medium-term (6 months)

- 10+ CLI tools covering common needs
- 5+ MCP tools for complex operations
- 15+ Skills encoding best practices
- External developers using tools
- Contributions to Skills and MCP ecosystems
- Recognition for hybrid architecture approach

### Long-term (12 months)

- Comprehensive tool suite
- Platform capabilities emerging
- Industry adoption
- Foundation for next career move

## Technical Approach

### Three-Layer Architecture

**Layer 1: Skills** (Orchestration)

- Markdown files with YAML frontmatter
- Guide Claude's behavior and tool selection
- Encode team conventions and workflows
- Token-efficient (few dozen tokens overhead)

**Layer 2: CLI Tools** (Fast, Standalone)

- Rust binaries with < 10ms startup time
- Agent-friendly (JSON output, idempotent, non-interactive)
- Single-binary distribution (no dependencies)
- Perfect for Skills to invoke

**Layer 3: MCP Tools** (Deep Integration)

- Hexagonal architecture for each tool
- Deep system integration (databases, complex state)
- Stateful operations requiring protocol support
- Used when CLI tools insufficient

### Repository Structure

```
tapestry/
├── cli/           # Standalone CLI tools for Skills
├── mcp/           # MCP protocol-based tools
├── skills/        # Skills that orchestrate both
└── src/           # Shared registry (future)
```

### Tool Selection Criteria

| Need                        | Use CLI Tool | Use MCP Tool |
| --------------------------- | ------------ | ------------ |
| Fast startup required       | ✅           | ❌           |
| Invoked by Skills           | ✅           | ✅ (can be)  |
| Complex stateful operations | ❌           | ✅           |
| Deep system integration     | ❌           | ✅           |
| Simple CRUD on files        | ✅           | ❌           |
| Token efficiency critical   | ✅           | ❌           |

### Quality Bar

- S-tier engineering practices
- Comprehensive testing (70/20/10 pyramid)
- Performance targets (P50 <100ms)
- Security-first design

### Development Process

- **RFD-driven design** (transitioning from RFC format)
- Documentation-first approach (dogfooding our own tools)
- Skills-first exploration (validate paradigm before heavy investment)
- Continuous deployment
- Real-world validation through daily use

## Why This Matters

### For Me

- Practical learning through building
- Portfolio of production-quality work
- Deep expertise in AI-assisted development
- Path to joining leading AI companies

### For the Community

- Open-source CLI tools, MCP tools, and Skills
- Patterns for hybrid AI-native development
- Practical examples of when to use Skills vs MCP
- Bridge between AI and traditional development
- Contribution to Skills and MCP ecosystems

## Next Steps

1. ✅ Restructure repository (cli/, mcp/, skills/)
2. ✅ Initialize RFD CLI tool structure
3. 🚧 Implement RFD CLI MVP (create, list, show, status)
4. 🚧 Create rfd-manager Skill
5. 📋 Dogfood RFD tool on Tapestry documentation
6. 📋 Evaluate Skills limitations
7. 📋 Resume MCP tool development based on learnings

---

_"The best way to predict the future is to build it."_

This is Tapestry: Not just tools, but a journey toward the future of
development.
