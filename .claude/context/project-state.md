# Tapestry Project State

**Last Updated**: 2025-10-17
**Phase**: Skills-First Foundation (Pre-Alpha)
**Version**: 0.1.0-dev

## Project Status

### Current Sprint (2025-10-17 onwards)

#### Completed ✅

- [x] Initial project structure created
- [x] Documentation framework established
- [x] S-tier engineering principles researched
- [x] `.claude/` directory structure designed
- [x] Hexagonal architecture pattern selected
- [x] **Repository restructured** (cli/, mcp/, skills/)
- [x] **RFD CLI project initialized** with Cargo structure
- [x] **Cargo workspace configured** for multi-tool development
- [x] **VISION.md updated** to reflect hybrid architecture
- [x] **Skills research** completed (understand when to use vs MCP)

#### In Progress 🚧

- [ ] **RFD CLI MVP implementation** (create, list, show, status)
- [ ] **RFC-002: RFD CLI tool design document**
- [ ] **rfd-manager Skill creation**
- [ ] Template system design for RFDs

#### Upcoming 📋

- [ ] Dogfood RFD CLI on Tapestry documentation
- [ ] Convert existing RFCs to RFD format
- [ ] Create additional Skills for common workflows
- [ ] Evaluate Skills limitations through real usage
- [ ] Resume git-workflow MCP tool based on learnings
- [ ] CI/CD pipeline setup
- [ ] Binary distribution strategy

## Technical Stack Status

| Component         | Status         | Notes                                  |
| ----------------- | -------------- | -------------------------------------- |
| Rust Setup        | ✅ Complete    | Workspace with nix flake               |
| CLI Tools (clap)  | 🚧 In Progress | RFD CLI skeleton working               |
| Templating        | 🔄 Planning    | minijinja for RFD templates            |
| MCP Tools         | ⏸️ Paused      | git-workflow paused for Skills testing |
| rmcp Integration  | ⏸️ Paused      | Resume after Skills validation         |
| Skills            | 🚧 In Progress | First skill (rfd-manager) in dev       |
| Testing Framework | 🔄 Planning    | Will establish with RFD CLI            |
| Logging (tracing) | 🔄 Planning    | Add when needed for debugging          |
| Documentation     | ✅ Active      | Using own tools to document            |

## Codebase Metrics

```
Total Lines of Code: ~1,000 (Rust)
  - cli/rfd: ~200 lines (skeleton)
  - mcp/git_workflow: ~800 lines (paused)
Documentation Lines: ~1,500
Test Coverage: TBD (will establish with RFD CLI)
Number of CLI Tools: 1 (RFD CLI - in development)
Number of MCP Tools: 1 (git-workflow - paused)
Number of Skills: 0 (first one in progress)
```

## Known Issues 🐛

| Issue                                    | Priority | Status | Owner |
| ---------------------------------------- | -------- | ------ | ----- |
| RFD CLI not yet functional (MVP needed)  | High     | Open   | -     |
| No Skills created yet                    | High     | Open   | -     |
| git-workflow MCP tool incomplete         | Low      | Paused | -     |
| No binary distribution strategy          | Medium   | Open   | -     |
| Template system design needed            | High     | Open   | -     |

## Technical Debt 💳

### Immediate

- Implement RFD CLI MVP functionality
- Create first Skill (rfd-manager)
- Design template system for RFDs
- Write RFC-002 for RFD CLI tool

### Short-term (This Month)

- Establish testing patterns (start with RFD CLI)
- Set up binary build and distribution
- Document Skills → CLI workflow
- Create example Skills for reference

### Long-term (This Quarter)

- Evaluate when to use Skills vs MCP (document learnings)
- Resume git-workflow MCP tool development
- Performance benchmarking framework
- Comprehensive error handling strategy
- Tool versioning strategy

## Decisions Made 📝

1. **Hybrid Architecture**: Three layers (Skills → CLI tools + MCP tools)
2. **Skills-First Approach**: Validate Skills paradigm before heavy MCP
   investment
3. **Language**: Rust for CLI tools (startup time, distribution) and MCP tools
   (performance, safety)
4. **Repository Structure**: `cli/`, `mcp/`, `skills/` directories for clarity
5. **Documentation First**: Dogfooding our own RFD tool
6. **RFD Process**: Transitioning from RFC to RFD format (using our own tool)
7. **Pause git-workflow**: Focus on RFD CLI to validate Skills approach first

## Open Questions ❓

### Architectural

- What are the actual limitations of Skills? (Need real usage to find out)
- When should we use Skills vs MCP vs CLI? (Document decision matrix)
- How do Skills and MCP tools interact? (Can Skills invoke MCP tools?)
- Should CLI tools be able to call each other? (Composition strategy)

### RFD CLI Specific

- What RFD template sections are essential? (Iterate based on dogfooding)
- How to handle multiple document types? (RFDs, ADRs, specs)
- Should we support AsciiDoc export for Oxide compatibility?
- How to integrate with gh CLI for issue → RFD workflow?

### Process

- Should RFDs fully replace RFCs or complement them?
- What's our release cadence for CLI tools?
- How do we version Skills (they're just markdown)?
- How do we handle breaking changes in CLI tools?

### Infrastructure

- Where to host binary releases? (GitHub Releases? crates.io?)
- Should we build a Skills marketplace/registry?
- What's our documentation hosting strategy?
- Do we need telemetry/analytics? (Privacy considerations)

## Risk Register ⚠️

| Risk                          | Likelihood | Impact | Mitigation                           |
| ----------------------------- | ---------- | ------ | ------------------------------------ |
| Skills prove insufficient     | Medium     | High   | Quick validation before heavy invest |
| Scope creep                   | High       | High   | Focus on RFD CLI MVP first           |
| Performance issues (CLI)      | Low        | Medium | Rust + simple operations             |
| Complex setup for users       | Medium     | Medium | Single binary distribution           |
| Template system too rigid     | Medium     | Medium | Iterate based on dogfooding          |
| Skills/MCP ecosystem changes  | Medium     | Medium | Abstract interfaces, stay modular    |

## Team Notes 📓

### For Next Session

1. Implement RFD CLI MVP (create, list, show commands)
2. Design template system (default RFD template)
3. Create rfd-manager Skill
4. Write RFC-002 for RFD CLI design
5. Test Skills → CLI invocation workflow

### Blockers

- None currently (good momentum!)

### Recent Learnings

- **Skills are simpler than MCP** but have limitations (need to discover them)
- **Hybrid approach makes sense**: Use the right tool for each job
- **Dogfooding is essential**: Use our own tools to find pain points
- **Token efficiency matters**: Skills overhead is minimal vs MCP
- **Startup time is critical**: 5ms vs 100ms+ makes a difference for agents
- S-tier companies use RFC/RFD processes (now implementing our own)
- Documentation-first approach pays dividends
- Hexagonal architecture scales well (preserved in git-workflow)
- Start simple, iterate based on needs (Skills-first validates this)

## Resource Links 🔗

### Internal

- [Vision Document](/docs/VISION.md)
- [Design Documentation Guide](/docs/design/meta/design-documentation-guide.md)
- [CLAUDE.md Quick Reference](/CLAUDE.md)

### External

- [MCP Documentation](https://modelcontextprotocol.io/)
- [rmcp Crate](https://crates.io/crates/rmcp)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Next Milestones 🎯

### Week 1-2 (Late Oct 2025)

- [ ] RFD CLI MVP functional (create, list, show, status)
- [ ] First Skill (rfd-manager) working
- [ ] RFC-002 written and published
- [ ] Template system designed and implemented

### Month 1 (Nov 2025)

- [ ] Dogfood RFD CLI for all Tapestry docs
- [ ] 2-3 Skills created
- [ ] Binary distribution working (GitHub Releases)
- [ ] Skills limitations documented

### Quarter 1 (End of Dec 2025)

- [ ] 3-5 CLI tools operational
- [ ] 1-2 MCP tools operational (resume git-workflow)
- [ ] 5+ Skills encoding best practices
- [ ] Clear decision matrix for Skills vs MCP vs CLI
- [ ] First external users testing tools

---

_This document is updated weekly during sprint planning and daily during active
development._
