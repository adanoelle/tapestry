# Current Session: Skills-First Validation & RFD CLI Development

**Date**: 2025-10-17
**Focus**: Validate Skills paradigm with RFD CLI tool
**Goal**: Build RFD CLI MVP and first Skill (rfd-manager)

## Session Context

Pivoting Tapestry to a hybrid three-layer architecture (Skills → CLI tools + MCP tools). We're validating the Skills-first approach by building the RFD CLI tool before investing heavily in MCP infrastructure.

## Today's Objectives

### Immediate (Next Hour)

1. [x] Update `.claude/` directory for hybrid architecture
2. [ ] Complete Skills scaffolding infrastructure
3. [ ] Complete CLI tool scaffolding infrastructure
4. [ ] Create RFD command using our own tool

### Today's Goals

1. [ ] Skills scaffolding command `/create-skill` working
2. [ ] CLI tool scaffolding command `/create-cli-tool` working
3. [ ] RFD command `/create-rfd` working (dogfooding!)
4. [ ] ADR-002 documenting hybrid architecture decision
5. [ ] New specialized agents created

## Current Task

**Task**: Update `.claude/` directory infrastructure
**Status**: In progress (Phase 1 complete: MCP tool references fixed)
**Next Steps**:

1. Create Skills scaffolding (command, template, workflow, script)
2. Create CLI tool scaffolding (command, template, workflow, script)
3. Create RFD command (uses our RFD CLI tool)
4. Document hybrid architecture decision

## Decisions to Make

1. **Skills scaffolding automation level**: Shell script or just markdown command?
   - Leaning toward: Shell script for consistency with MCP scaffolding

2. **CLI tool structure**: How much simpler than MCP tools?
   - Recommendation: No hexagonal architecture, simple flat structure

3. **RFD template location**: In RFD CLI or in `.claude/templates/`?
   - Recommendation: Both - CLI has built-in, `.claude/` references it

## Code Snippets to Use

### Skills Template Structure

```yaml
---
name: {{skill_name}}
description: {{description_with_usage_triggers}}
allowed-tools: {{optional_tool_restrictions}}
---

# {{skill_name}}

## Purpose
{{what_this_skill_does}}

## When to Use
{{usage_triggers_for_claude}}

## Instructions
{{step_by_step_guidance}}

## Examples
{{concrete_examples}}
```

### CLI Tool Structure (Simple)

```rust
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "pretty")]
    format: String,  // pretty, json, quiet
}

#[derive(Subcommand)]
enum Commands {
    // Command definitions
}

fn main() -> Result<()> {
    // Simple, flat implementation
    Ok(())
}
```

## Questions for Next Session

1. What Skills limitations did we discover?
2. How well does the rfd-manager Skill work in practice?
3. Do we need to adjust the Skills vs CLI vs MCP decision matrix?
4. What patterns emerged from building the first CLI tool?

## Notes & Learnings

### From Architectural Pivot

- **Skills are model-invoked** - Claude decides when to use them based on description
- **Skills are token-efficient** - Few dozen tokens vs tens of thousands for MCP
- **No built-in scaffolding** - We need to build our own tooling
- **Hybrid approach makes sense** - Use the right tool for each job
  - Skills: Orchestration and workflow guidance
  - CLI tools: Fast, agent-friendly operations (< 10ms startup)
  - MCP tools: Deep integration, stateful operations

### Architecture Decisions

- Three-layer hybrid: Skills → CLI + MCP
- Skills-first validation strategy before heavy MCP investment
- Repository structure: `cli/`, `mcp/`, `skills/`
- RFDs will replace RFCs (dogfooding our own tool)

### For Claude Code

When implementing:

- Use `/create-skill` for new Skills (once created)
- Use `/create-cli-tool` for CLI tools (once created)
- Use `/create-mcp-tool` for MCP tools (updated for `mcp/` directory)
- Use `/create-rfd` for documentation (dogfooding!)
- Reference hybrid architecture in `.claude/context/architecture.md`

## Blockers

None currently! Making good progress on `.claude/` directory updates.

## Next Session Plan

**Tomorrow's Focus**: RFD CLI MVP Implementation

1. Implement RFD CLI commands (create, list, show, status)
2. Test rfd-manager Skill
3. Dogfood by converting RFC-001 and RFC-002 to RFD format
4. Document Skills limitations discovered

## Command Shortcuts

```bash
# Create new Skill
/create-skill "skill-name" "description"

# Create new CLI tool
/create-cli-tool "tool-name" "description"

# Create new MCP tool
/create-mcp-tool "tool-name" "description"

# Create new RFD
/create-rfd "RFD Title" "Summary"

# Run RFD CLI MVP
cargo run --bin rfd -- --help

# Check project status
cat .claude/context/project-state.md
```

## References

- [Project Vision](/docs/VISION.md) - Updated with hybrid architecture
- [Architecture Guide](/.claude/context/architecture.md) - Three-layer model
- [RFC-002: RFD CLI Tool](/docs/design/features/RFC-002-rfd-cli.md)
- [Skills Documentation](https://docs.claude.com/en/docs/claude-code/skills)

## Current Sprint Progress

**Week 1-2 (Late Oct 2025)** - Skills Infrastructure & RFD CLI MVP:

- [x] Hybrid architecture decision made
- [x] Repository restructured (cli/, mcp/, skills/)
- [x] RFD CLI skeleton created
- [x] VISION.md updated
- [x] RFC-002 written
- [x] rfd-manager Skill spec created
- [x] `.claude/` directory MCP references fixed
- [ ] Skills scaffolding infrastructure
- [ ] CLI tool scaffolding infrastructure
- [ ] RFD CLI MVP functional
- [ ] First Skill (rfd-manager) tested
- [ ] ADR-002 written

---

_Session notes are ephemeral. Important decisions should be moved to `context/` or `knowledge/`._
