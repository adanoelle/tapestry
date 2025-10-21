# Changelog

All notable changes to the Tapestry project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **RFD Search Command** - Comprehensive search functionality for RFD CLI ([RFC-003](docs/design/features/RFC-003-rfd-search.md))
  - Basic text search across RFD documents
  - Multiple search terms with AND logic (all terms must match)
  - Field-specific search with `--in` flag (title, content, tags, metadata, all)
  - Case-sensitive search mode with `--case-sensitive` flag
  - Integration with existing filters (`--status`, `--author`, `--limit`)
  - JSON output support for AI agents
  - 15 integration tests covering all search scenarios
  - Comprehensive documentation and examples (`examples/search.sh`)
  - Performance: ~110ms for 100 RFDs (sequential search, no indexing)

## [0.1.0] - 2025-10-19

### Added

- **RFD CLI Tool** - Production-ready CLI for managing RFD documents
  - `create` command with template support
  - `list` command with filtering by status/author
  - `show` command for displaying RFD details
  - `status` command for state transitions
  - `update` command for metadata modifications
  - `validate` command for structure checking
  - JSON output mode for AI agents (`--format json`)
  - Three output formats: pretty, json, quiet
  - Idempotent operations for safe retries
  - Actionable error messages with suggestions
  - State machine for document lifecycle (draft → review → accepted → implemented → archived)
  - Template system with Jinja2 support
  - YAML frontmatter parsing
  - Configuration via `.rfd/config.toml`
  - 32 tests with >80% coverage
  - Performance: 2.4MB binary, 1ms startup, ~5MB memory

- **CI/CD Pipeline**
  - GitHub Actions for testing, linting, formatting
  - Cross-platform release workflow (Linux, macOS, Windows)
  - Git hooks for local development (pre-commit, commit-msg)
  - Security audit workflow

- **Documentation**
  - Complete README with examples
  - Architecture documentation
  - Contributing guidelines
  - 5 example scripts demonstrating usage patterns
  - RFC-002 specification

- **Project Infrastructure**
  - Workspace structure with `cli/` and `mcp/` directories
  - Nix flake for reproducible development environment
  - Setup script for non-Nix users
  - Comprehensive `.claude/` context for AI collaboration

### Changed

- N/A (initial release)

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A

---

## Release Notes Format

### Version Number Format

We use [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality in a backward compatible manner
- **PATCH** version for backward compatible bug fixes

### Categories

- **Added** - New features
- **Changed** - Changes in existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security fixes

---

[Unreleased]: https://github.com/adanoelle/tapestry/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/adanoelle/tapestry/releases/tag/v0.1.0
