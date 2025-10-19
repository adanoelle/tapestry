# ADR-002: Hybrid Three-Layer Architecture (Skills + CLI + MCP)

## Status

**Accepted** (2025-10-17)

## Context

Tapestry was initially designed as a monolithic suite of MCP (Model Context Protocol) tools using hexagonal architecture (see ADR-001). However, after researching Anthropic's Skills feature and analyzing token efficiency, we identified an opportunity to create a more effective hybrid architecture.

### The Discovery

While planning the git-workflow MCP tool, we discovered that:

1. **Skills are lightweight**: Few dozen tokens vs tens of thousands for MCP
2. **Skills are model-invoked**: Claude decides when to use them based on description
3. **No built-in scaffolding**: Anthropic doesn't provide tooling for Skills
4. **Token efficiency matters**: Each MCP tool has protocol overhead
5. **Startup time is critical**: CLI tools can start in < 10ms vs 100ms+ for MCP

### The Problem

We needed to answer: **What's the right tool for each job?**

- When should we use Skills vs MCP tools?
- Do we need a third option for fast, stateless operations?
- How can we validate the Skills approach before heavy MCP investment?
- What architecture supports all three paradigms?

## Decision

We will adopt a **hybrid three-layer architecture** where:

1. **Layer 1: Skills** (Orchestration)
   - Markdown files with YAML frontmatter
   - Model-invoked by Claude based on context
   - Token-efficient (few dozen tokens)
   - Orchestrate workflows across tools

2. **Layer 2: CLI Tools** (Fast, Standalone)
   - Rust binaries with < 10ms startup
   - Agent-friendly (JSON output, idempotent)
   - Simple flat architecture (no hexagonal)
   - Invoked by Skills via Bash tool

3. **Layer 3: MCP Tools** (Deep Integration)
   - Hexagonal architecture (per ADR-001)
   - Stateful, complex operations
   - Protocol-based communication
   - Deep system integration

### Decision Matrix

| Need | CLI Tool | MCP Tool | Skill |
|------|----------|----------|-------|
| Startup < 10ms | ✅ | ❌ | ✅ |
| Agent invokes | ✅ | ✅ | ✅ |
| Stateful/complex | ❌ | ✅ | N/A |
| File CRUD | ✅ | ❌ | N/A |
| Orchestrate workflows | ❌ | ❌ | ✅ |
| Token efficiency | ✅ | ❌ | ✅ |
| Deep integration | ❌ | ✅ | N/A |

### Repository Structure

```
tapestry/
├── cli/                  # Fast, agent-friendly CLI tools
│   ├── rfd/             # RFD document management
│   └── {other-tools}/
├── mcp/                  # Deep integration MCP tools
│   └── git_workflow/    # Git workflow automation
├── skills/               # Workflow orchestration Skills
│   └── rfd-manager/     # RFD management workflows
└── .claude/              # Development tooling
    ├── commands/         # Scaffolding commands for all three
    ├── scripts/          # Automation scripts
    └── workflows/        # Development workflows
```

## Rationale

### Why This Hybrid Approach?

**1. Skills-First Validation**
- Validate Skills paradigm before heavy MCP investment
- Discover Skills limitations through real usage
- Build lightweight tools first
- Learn what works before committing

**2. Right Tool for the Job**
- Skills: Cheap orchestration, workflow guidance
- CLI: Fast operations, file CRUD, agent-friendly
- MCP: When you truly need stateful complexity

**3. Token Efficiency**
- Skills add minimal token overhead
- CLI tools have zero protocol overhead
- MCP tools only when complexity justifies cost

**4. Startup Performance**
- Skills: Instant (already loaded)
- CLI tools: < 10ms cold start
- MCP tools: 100ms+ (acceptable for complex operations)

**5. Development Experience**
- Skills: Markdown (easy to create)
- CLI tools: Simple Rust (clap + flat structure)
- MCP tools: Full hexagonal (when needed)

### Validation Strategy

We're dogfooding this approach with the RFD CLI tool:

1. **Build RFD CLI** (Layer 2) - Fast, agent-friendly
2. **Create rfd-manager Skill** (Layer 1) - Orchestrates RFD CLI
3. **Test integration** - Skills invoke CLI via Bash
4. **Document limitations** - What doesn't work?
5. **Resume git-workflow** (Layer 3) - MCP when needed

If this works well, we have a validated pattern. If not, we learned cheaply.

## Consequences

### Positive

✅ **Flexibility**: Right tool for each scenario
✅ **Token Efficiency**: Use cheap tools when possible
✅ **Fast Validation**: Test Skills before heavy MCP work
✅ **Better DX**: Simpler tools for simple tasks
✅ **Startup Performance**: CLI tools are blazing fast
✅ **Gradual Adoption**: Can add MCP tools as needed
✅ **Dogfooding**: Use our own tools immediately

### Negative

❌ **More Complexity**: Three paradigms vs one
❌ **Learning Curve**: Team needs to know when to use what
❌ **Tool Sprawl**: More tools to maintain
❌ **Unclear Boundaries**: When CLI vs MCP can be ambiguous

### Neutral

➖ **More Scaffolding**: Need commands for all three types
➖ **Documentation**: Must explain three approaches
➖ **Decision Making**: Need clear guidelines

## Implementation

### Phase 1: Skills Infrastructure (Complete)

- [x] Create Skills scaffolding command (`/create-skill`)
- [x] Create Skills template and workflow
- [x] Create Skills automation script
- [x] Document Skills best practices

### Phase 2: CLI Tools Infrastructure (Complete)

- [x] Create CLI scaffolding command (`/create-cli-tool`)
- [x] Create CLI template and workflow
- [x] Create CLI automation script
- [x] Establish performance targets (< 10ms, < 3MB)

### Phase 3: RFD Integration (Complete)

- [x] Build RFD CLI MVP
- [x] Create rfd-manager Skill
- [x] Create `/create-rfd` command
- [x] Deprecate RFC format

### Phase 4: Documentation (In Progress)

- [x] ADR-002 (this document)
- [x] Update architecture.md with three layers
- [x] Update VISION.md with hybrid approach
- [ ] Create specialized agents (cli-tool-expert, skills-designer)

### Phase 5: Validation (Upcoming)

- [ ] Dogfood RFD CLI for all Tapestry docs
- [ ] Convert existing RFCs to RFD format
- [ ] Document Skills limitations found
- [ ] Refine decision matrix based on usage

### Phase 6: Resume MCP Development (Future)

- [ ] Resume git-workflow MCP tool
- [ ] Apply learnings from Skills validation
- [ ] Build next MCP tool when complexity justifies it

## Alternatives Considered

### Alternative 1: MCP-Only (Original Plan)

**What**: Build everything as MCP tools with hexagonal architecture

**Pros**:
- Single paradigm to learn
- Consistent architecture
- Deep integration capabilities

**Cons**:
- High token cost for simple operations
- Slow startup times
- Over-engineering for simple tasks
- Heavy investment before validation

**Why not chosen**: Too expensive for simple operations, doesn't leverage Skills

### Alternative 2: Skills-Only

**What**: Use only Skills, no custom tools

**Pros**:
- Minimal token overhead
- Simplest approach
- Fastest to implement

**Cons**:
- Limited to built-in tools
- Can't build custom capabilities
- No fast CLI operations
- No stateful workflows

**Why not chosen**: Too limiting, can't build product features

### Alternative 3: CLI-Only

**What**: Build only CLI tools, no MCP or Skills

**Pros**:
- Fast startup
- Simple architecture
- Easy to distribute

**Cons**:
- No orchestration layer
- No stateful operations
- Manual invocation only
- Doesn't leverage MCP ecosystem

**Why not chosen**: Missing orchestration and deep integration

## Metrics for Success

We'll measure success by:

### Adoption Metrics

- **Skills Created**: Target 5+ Skills in first month
- **CLI Tools Built**: Target 3-5 CLI tools in first quarter
- **MCP Tools Built**: Only when complexity justifies (1-2 in first quarter)

### Performance Metrics

- **CLI Startup Time**: < 10ms for all CLI tools
- **Binary Size**: < 3MB for CLI tools
- **Token Efficiency**: Skills add < 100 tokens overhead

### Quality Metrics

- **Dogfooding**: Use our own tools for all documentation
- **Skills Invocation Rate**: Claude uses Skills automatically > 80% of time
- **CLI Test Coverage**: > 80% for all CLI tools

### Learning Metrics

- **Limitations Documented**: Document all Skills limitations found
- **Decision Matrix Refined**: Update based on real usage
- **Patterns Discovered**: Document successful patterns

## Decision Timeline

- **2025-10-15**: Discovery of Skills feature
- **2025-10-16**: Research and analysis phase
- **2025-10-17**: Decision made, ADR written
- **2025-10-17**: Implementation begins (Skills + CLI scaffolding)
- **2025-10-17**: RFD CLI started as validation
- **2025-10-18**: First validation results expected
- **2025-11-01**: Full validation complete (1 month target)

## Review and Evolution

This decision should be reviewed:

- **Monthly** for first 3 months (validation period)
- **Quarterly** thereafter
- **Ad-hoc** when significant limitations discovered

### Potential Evolution Paths

1. **If Skills prove insufficient**: Shift more to CLI/MCP
2. **If CLI is enough**: Reduce MCP usage
3. **If MCP becomes necessary**: Expand Layer 3
4. **If hybrid works well**: Codify patterns and scale

## References

- [ADR-001: Hexagonal Architecture](ADR-001-hexagonal-architecture.md) - Still applies to MCP tools
- [Architecture Guide](../../.claude/context/architecture.md) - Three-layer details
- [Skills Documentation](https://docs.claude.com/en/docs/claude-code/skills) - Anthropic docs
- [VISION.md](/docs/VISION.md) - Updated project vision

## Notes

### Key Insights

- **Start lightweight**: Skills and CLI before MCP
- **Validate early**: Test Skills paradigm before heavy investment
- **Dogfood everything**: Use our own tools
- **Document limitations**: Learn what doesn't work
- **Iterate quickly**: Adjust based on real usage

### Open Questions

1. What are the actual limitations of Skills? (Need usage to find out)
2. When does a CLI tool become complex enough to warrant MCP? (Will discover)
3. How do Skills and MCP tools interact? (Test with git-workflow)
4. Should CLI tools be able to call each other? (TBD)

---

**Decision**: Accepted
**Date**: 2025-10-17
**Author**: Tapestry Team
**Approvers**: Ada (Project Lead)
