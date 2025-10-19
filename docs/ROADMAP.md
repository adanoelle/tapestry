# Tapestry Roadmap

This document outlines Tapestry's development roadmap, current status, and future plans.

## Vision

Build a suite of AI-native development tools that enhance developer productivity through a hybrid architecture of lightweight CLI tools, deep-integration MCP tools, and orchestrating Skills.

## Current Status

**Phase 1: Foundation** ✅ **COMPLETE**

We have successfully completed our foundation phase with a production-ready CLI tool and comprehensive infrastructure.

## Roadmap

### Phase 1: Foundation ✅ COMPLETE

**Goal**: Establish project structure, architecture, and deliver first production tool

- [x] **Project Infrastructure**
  - [x] Repository structure and organization
  - [x] Documentation framework (`.claude/` directory)
  - [x] Architecture decision records
  - [x] Team conventions and coding standards

- [x] **RFD CLI Tool** - Production Ready! 🎉
  - [x] Core commands (create, list, show, status, update, validate)
  - [x] Template system with Jinja2 support
  - [x] JSON output mode for AI agents
  - [x] Comprehensive test suite (32 tests, >80% coverage)
  - [x] Full documentation (README, ARCHITECTURE, CONTRIBUTING, examples)
  - [x] Performance optimization (2.4MB binary, 1ms startup)

- [x] **CI/CD Pipeline** - Complete! 🎉
  - [x] GitHub Actions for CI (test, lint, format, security audit)
  - [x] Cross-platform release workflow (Linux, macOS, Windows)
  - [x] Git hooks for local development (pre-commit, commit-msg)
  - [x] Developer setup script for non-Nix users
  - [x] Nix flake with Claude Code integration
  - [x] Comprehensive CI/CD documentation

**Outcomes**:
- ✅ Proven CLI tool pattern
- ✅ Development workflow established
- ✅ Quality standards demonstrated
- ✅ Foundation for rapid tool development

---

### Phase 2: Essential CLI Tools 🎯 CURRENT FOCUS

**Goal**: Expand CLI tool suite and enhance existing tools based on user feedback

**Timeline**: Q2-Q3 2025

#### RFD CLI Enhancements

- [ ] **Search and Discovery**
  - [ ] Full-text search across RFD content
  - [ ] Search by status, author, tags
  - [ ] Fuzzy matching for titles
  - [ ] `rfd search <query>` command

- [ ] **Export Capabilities**
  - [ ] Export to HTML (single file and site)
  - [ ] Export to PDF (with styling)
  - [ ] Export to Markdown (consolidated)
  - [ ] `rfd export <format>` command

- [ ] **Git Integration**
  - [ ] Auto-commit on RFD changes
  - [ ] Generate commit messages
  - [ ] Track RFD history via git
  - [ ] `--git` flag for commands

- [ ] **GitHub Integration**
  - [ ] Link RFDs to GitHub issues
  - [ ] Sync status with PR state
  - [ ] Auto-create discussion issues
  - [ ] `rfd github` subcommands

- [ ] **Dependency Tracking**
  - [ ] Declare dependencies between RFDs
  - [ ] Visualize dependency graph
  - [ ] Check for circular dependencies
  - [ ] `rfd deps` command

#### New CLI Tools

- [ ] **Code Review CLI**
  - [ ] Automated code review with AI insights
  - [ ] Style and best practice checking
  - [ ] Security vulnerability detection
  - [ ] Integration with git diff
  - [ ] JSON output for AI processing

- [ ] **Test Generator CLI**
  - [ ] Intelligent test generation from code
  - [ ] Multiple test frameworks support
  - [ ] Edge case identification
  - [ ] Coverage gap analysis
  - [ ] Property-based test suggestions

- [ ] **Documentation Generator CLI**
  - [ ] Extract docs from code comments
  - [ ] Generate API documentation
  - [ ] Create usage examples
  - [ ] Maintain living documentation
  - [ ] Multiple output formats

**Success Criteria**:
- RFD CLI has 5+ additional commands
- At least 2 new CLI tools in production
- User feedback incorporated
- All tools follow established patterns

---

### Phase 3: MCP Tools & Intelligence 🔮 FUTURE

**Goal**: Introduce stateful MCP tools and cross-tool intelligence

**Timeline**: Q4 2025 - Q1 2026

#### MCP Tools

- [ ] **Git Context Tool**
  - [ ] Resume paused development (RFC-001)
  - [ ] Rich git history and context
  - [ ] Conflict resolution assistance
  - [ ] Branch and merge insights
  - [ ] Integration with Claude Code

- [ ] **Session Memory Tool**
  - [ ] Persistent context across sessions
  - [ ] Project state tracking
  - [ ] Decision history
  - [ ] Learning from past interactions

- [ ] **Pattern Recognition Tool**
  - [ ] Identify code patterns
  - [ ] Suggest refactoring opportunities
  - [ ] Detect anti-patterns
  - [ ] Architecture recommendations

#### Skills Layer

- [ ] **RFD Manager Skill**
  - [ ] End-to-end RFD lifecycle management
  - [ ] Orchestrate CLI and MCP tools
  - [ ] Automated workflows (create → review → approve)
  - [ ] Context-aware suggestions

- [ ] **Code Review Skill**
  - [ ] Coordinate Code Review CLI + Git Context
  - [ ] Multi-file context
  - [ ] Automated PR reviews
  - [ ] Learning from feedback

#### Intelligence Features

- [ ] **Cross-Tool Integration**
  - [ ] Shared context between tools
  - [ ] Workflow automation
  - [ ] Event-driven updates
  - [ ] Tool chaining

- [ ] **Learning System**
  - [ ] Adapt to team patterns
  - [ ] Personalized recommendations
  - [ ] Improving suggestions over time
  - [ ] Privacy-preserving learning

**Success Criteria**:
- At least 2 MCP tools in production
- Skills successfully orchestrate tools
- Demonstrable intelligence improvements
- Positive user feedback on AI features

---

### Phase 4: Ecosystem & Scale 🌍 VISION

**Goal**: Build a thriving ecosystem and scale to larger teams

**Timeline**: 2026+

#### Platform

- [ ] **Tool Registry**
  - [ ] Discover and install tools
  - [ ] Version management
  - [ ] Dependency resolution
  - [ ] Quality ratings

- [ ] **Plugin System**
  - [ ] Third-party tool development
  - [ ] Sandboxed execution
  - [ ] Clear plugin API
  - [ ] Documentation and examples

- [ ] **Marketplace**
  - [ ] Community-contributed tools
  - [ ] Tool reviews and ratings
  - [ ] Commercial tool support
  - [ ] Revenue sharing model

#### Enterprise Features

- [ ] **Team Collaboration**
  - [ ] Shared context and state
  - [ ] Role-based access control
  - [ ] Audit logging
  - [ ] Compliance features

- [ ] **Self-Hosted Option**
  - [ ] On-premise deployment
  - [ ] Air-gapped environments
  - [ ] Custom integrations
  - [ ] Enterprise support

- [ ] **Analytics & Insights**
  - [ ] Usage metrics
  - [ ] Productivity tracking
  - [ ] Tool effectiveness
  - [ ] ROI measurement

**Success Criteria**:
- Active community of tool developers
- Enterprise customers using Tapestry
- Self-sustaining ecosystem
- Proven ROI for teams

---

## Release Strategy

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Release Cycle

- **RFD CLI**: Monthly minor releases, weekly patches
- **New Tools**: As they reach production readiness
- **MCP Tools**: Beta period before stable release
- **Skills**: Continuous deployment (no versioning)

### Long-Term Support

- **Current**: Full support (features + fixes)
- **Current - 1**: Security fixes only
- **Older**: No support (upgrade recommended)

---

## How to Influence the Roadmap

We build based on user needs. Here's how you can help shape Tapestry:

### 1. Use Tapestry and Share Feedback

- Try the tools and report issues
- Share what works and what doesn't
- Suggest improvements

### 2. Write an RFC

Propose new features or tools using our RFC process:

1. Read [RFC template](.claude/templates/rfc-template.md)
2. Write your proposal in `docs/design/features/`
3. Open a PR for discussion
4. Iterate based on feedback

### 3. Contribute Code

- Pick an issue labeled `good-first-issue`
- Implement a planned feature
- Submit a pull request
- See [CONTRIBUTING.md](design/meta/CONTRIBUTING.md)

### 4. Share Use Cases

Tell us about your workflows:
- What tools would help you?
- What integrations do you need?
- What pain points can we solve?

Open a [discussion](https://github.com/yourusername/tapestry/discussions) to share your ideas.

---

## Milestones

### Q1 2025 ✅
- [x] RFD CLI v0.1.0 released
- [x] CI/CD pipeline complete
- [x] Foundation phase complete

### Q2 2025 🎯
- [ ] RFD CLI v0.2.0 with search and export
- [ ] Code Review CLI beta
- [ ] 100+ GitHub stars

### Q3 2025
- [ ] Test Generator CLI release
- [ ] Documentation Generator CLI beta
- [ ] First enterprise pilot

### Q4 2025
- [ ] Git Context MCP tool beta
- [ ] First Skills released
- [ ] 1,000+ users

### 2026
- [ ] Tool Registry launched
- [ ] Plugin API stable
- [ ] Enterprise tier available

---

## Metrics & Goals

### User Adoption

| Metric | Q2 2025 | Q3 2025 | Q4 2025 |
|--------|---------|---------|---------|
| GitHub Stars | 100 | 500 | 1,000 |
| Active Users | 50 | 200 | 500 |
| Tools Released | 2 | 4 | 6 |

### Quality

| Metric | Target |
|--------|--------|
| Test Coverage | > 80% |
| CI Pass Rate | > 95% |
| User Satisfaction | > 4.5/5 |
| Issue Response Time | < 24 hours |

### Performance

| Tool Type | Startup | Binary Size | Memory |
|-----------|---------|-------------|--------|
| CLI | < 10ms | < 3MB | < 10MB |
| MCP | < 1s | N/A | < 10MB |

---

## Archive

### Completed

- **Q1 2025**: Foundation phase
  - RFD CLI v0.1.0
  - CI/CD pipeline
  - Documentation infrastructure

### Deferred

- **Git Context MCP Tool**: Deferred to Phase 3 (Skills-first approach)

### Cancelled

_None yet_

---

## Questions?

- **When will feature X be ready?** Check the phase and timeline above
- **Can I help with Y?** Yes! See [CONTRIBUTING.md](design/meta/CONTRIBUTING.md)
- **Will you support Z?** Open a discussion to propose it

---

**Last Updated**: 2025-10-19
**Status**: Phase 2 planning in progress
**Next Milestone**: RFD CLI v0.2.0 (Q2 2025)
