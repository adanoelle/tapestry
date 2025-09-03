# Design Documentation

Welcome to the Tapestry design documentation. This directory contains all
architectural decisions, feature RFCs, and implementation details for the
project.

## 📁 Directory Structure

```
design/
├── features/         # Feature RFCs and specifications
├── implementation/   # Detailed implementation guides
├── meta/            # Documentation about documentation
└── archive/         # Historical/superseded documents
```

## 🎯 Quick Links

### Getting Started

- [Project Vision](../VISION.md) - Overall project vision and goals
- [Contributing](meta/CONTRIBUTING.md) - How to contribute to Tapestry

### Active RFCs

- [RFC-001: Git Context Tool](features/RFC-001-git-context-tool.md) - MCP tool
  for rich git context

### Implementation Guides

- *Coming soon* - Implementation guides will be added as tools are built

### Archived Documents

- [Original Vision](../archive/original-vision/) - Initial provenance platform vision and related docs

## 📝 Creating New Design Documents

1. **For new MCP tools**: Create an RFC in `features/` using the RFC template
2. **For implementation details**: Add to `implementation/`
3. **For process/meta docs**: Add to `meta/`

Use the templates in `.claude/templates/` as starting points.

## 🔄 RFC Process

1. **Propose**: Create RFC with status `PROPOSED`
2. **Discuss**: Gather feedback (even if just self-review)
3. **Decide**: Mark as `ACCEPTED` or `REJECTED`
4. **Implement**: Update status to `IMPLEMENTING`
5. **Complete**: Mark as `IMPLEMENTED`

## 📊 RFC Status Overview

| RFC                                                               | Status   | Author | Description                        |
| ----------------------------------------------------------------- | -------- | ------ | ---------------------------------- |
| [RFC-001: Git Context Tool](features/RFC-001-git-context-tool.md) | PROPOSED | Ada    | Rich git context for AI assistants |

## 🏗️ Architecture Decisions

Major architectural decisions are documented as Architecture Decision Records
(ADRs) in `.claude/knowledge/decisions/`:

- [ADR-001: Hexagonal Architecture](../../.claude/knowledge/decisions/ADR-001-hexagonal-architecture.md)

## 🚀 Roadmap

### Phase 1: Foundation (Current)

- [x] Documentation structure
- [x] Architecture decisions
- [ ] First MCP tool (Git Context)
- [ ] Tool registry system

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

See [Contributing Guide](meta/contributing.md) for details on:

- RFC template and process
- Code review standards
- Documentation requirements
- Testing expectations

---

_Last updated: 2024-01-15_
