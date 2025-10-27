# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2025-10-26

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

## [0.2.0] - 2025-10-26

### Added

- Automatic release tagging when version is bumped in Cargo.toml
- CHANGELOG.md for tracking version changes
- Comprehensive release documentation (docs/RELEASING.md)
- Release badge in README showing latest version
- Improved installation instructions with working download links
- Nix flake for reproducible development environments
- Comprehensive CI/CD pipeline with cross-platform builds
- Security auditing in CI workflow

### Fixed

- Nix environment compatibility issues with glibc 2.40
- macOS build failure due to cargo-llvm-cov incompatibility

### Changed

- Modernized release workflow to use softprops/action-gh-release@v2
- Auto-extract release notes from CHANGELOG.md
- Temporarily disabled Nix Environment Check workflow pending nixpkgs fixes

## [0.1.0] - 2025-10-26

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

### Performance

- Binary size: 2.4MB (stripped, optimized)
- Startup time: ~1ms (cold start)
- Memory usage: ~5MB peak

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

[Unreleased]: https://github.com/adanoelle/tapestry/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/adanoelle/tapestry/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/adanoelle/tapestry/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/adanoelle/tapestry/releases/tag/v0.1.0
