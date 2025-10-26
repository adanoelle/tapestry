# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

- Initial release of RFD CLI tool
- Create, list, and manage RFD documents
- YAML front matter support
- Multiple output formats (pretty, JSON, quiet)
- Jinja2 template system
- Cross-platform binary builds (Linux, macOS, Windows)
- Production-ready optimizations (LTO, size optimization)
- Comprehensive test suite
- GitHub Actions CI/CD workflow

### Performance

- Binary size: 2.4MB (stripped, optimized)
- Startup time: ~1ms (cold start)
- Memory usage: ~5MB peak

[Unreleased]: https://github.com/adanoelle/tapestry/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/adanoelle/tapestry/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/adanoelle/tapestry/releases/tag/v0.1.0
