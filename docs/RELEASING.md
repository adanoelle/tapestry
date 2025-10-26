# Release Process

This document describes how to create and publish new releases of Tapestry.

## Overview

Tapestry uses an automated release process triggered by version bumps in `Cargo.toml`. The process includes:

1. **Auto-tagging**: When version changes in `cli/rfd/Cargo.toml` on `main`, a git tag is automatically created
2. **Release workflow**: The tag triggers binary builds for all platforms
3. **GitHub Release**: Binaries are published with installation instructions and changelog

## Creating a Release

### 1. Update the Version

Edit `cli/rfd/Cargo.toml` and bump the version according to [Semantic Versioning](https://semver.org/):

```toml
[package]
name = "rfd"
version = "0.2.0"  # <- Change this
```

**Version Guidelines**:
- `MAJOR.MINOR.PATCH` (e.g., `1.2.3`)
- **MAJOR**: Breaking changes (incompatible API changes)
- **MINOR**: New features (backward-compatible)
- **PATCH**: Bug fixes (backward-compatible)
- Pre-release: `1.0.0-beta.1`, `1.0.0-rc.1`

### 2. Update the Changelog

Edit `CHANGELOG.md` to document changes for this version:

```markdown
## [0.2.0] - 2025-10-27

### Added
- New feature XYZ

### Fixed
- Bug in ABC

### Changed
- Updated dependency versions

[0.2.0]: https://github.com/adanoelle/tapestry/compare/v0.1.0...v0.2.0
```

**Changelog Sections** (use as applicable):
- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be-removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes

### 3. Commit and Push

```bash
git add cli/rfd/Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git push origin main
```

### 4. Automatic Release Process

Once pushed to `main`:

1. **Auto-tag workflow** (`auto-tag.yml`) runs:
   - Extracts version from `Cargo.toml`
   - Checks if tag already exists
   - Creates and pushes tag `v0.2.0`

2. **Release workflow** (`release.yml`) triggers:
   - Creates GitHub Release
   - Builds binaries for all platforms:
     - Linux (x86_64 GNU, x86_64 MUSL, ARM64)
     - macOS (Intel, Apple Silicon)
     - Windows (x86_64)
   - Generates SHA256 checksums
   - Uploads release assets
   - (Optional) Publishes to crates.io

3. **Release is published** at:
   - https://github.com/adanoelle/tapestry/releases/latest

## Manual Tag Creation (Not Recommended)

If the auto-tag workflow fails or you need manual control:

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push tag to trigger release
git push origin v0.2.0
```

## Pre-releases

For beta/RC releases:

1. Use pre-release version in `Cargo.toml`:
   ```toml
   version = "1.0.0-beta.1"
   ```

2. The release workflow automatically marks releases as "pre-release" if the version contains a hyphen

## Hotfix Releases

For urgent bug fixes:

1. Create a hotfix branch from the release tag:
   ```bash
   git checkout -b hotfix/0.1.1 v0.1.0
   ```

2. Make the fix and bump the PATCH version

3. Merge to main and follow normal release process

## Troubleshooting

### Tag Already Exists

If you push a version change and the tag already exists:

1. The auto-tag workflow skips tag creation
2. Delete the existing tag if you need to recreate it:
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```
3. Push again to trigger auto-tag

### Release Workflow Failed

Check the GitHub Actions tab for error details:
- https://github.com/adanoelle/tapestry/actions/workflows/release.yml

Common issues:
- **Build failure**: Check Rust compilation errors in logs
- **Upload failure**: Check GitHub token permissions
- **Crates.io failure**: Check `CARGO_TOKEN` secret is set

### Binary Not Building

If a specific platform fails:

1. Check the matrix configuration in `.github/workflows/release.yml`
2. Test locally with cross-compilation:
   ```bash
   # Install cross
   cargo install cross

   # Test build
   cross build --release --target x86_64-unknown-linux-musl
   ```

## Verification

After release is published:

1. **Check GitHub Release**:
   - Visit https://github.com/adanoelle/tapestry/releases/latest
   - Verify all binaries are present
   - Check installation instructions

2. **Test Installation**:
   ```bash
   # Quick install script
   curl -fsSL https://raw.githubusercontent.com/adanoelle/tapestry/main/scripts/install.sh | bash

   # Verify version
   rfd --version
   ```

3. **Test Binary Downloads**:
   - Download a platform-specific binary
   - Verify SHA256 checksum:
     ```bash
     sha256sum -c rfd-linux-amd64.tar.gz.sha256
     ```

## Release Checklist

- [ ] Version bumped in `cli/rfd/Cargo.toml`
- [ ] `CHANGELOG.md` updated with release notes
- [ ] Changes committed and pushed to `main`
- [ ] Auto-tag workflow succeeded
- [ ] Release workflow succeeded
- [ ] All platform binaries present in release
- [ ] Installation instructions work
- [ ] Version number correct in release
- [ ] Changelog rendered properly in release notes

## Future Enhancements

Planned improvements to the release process:

- [ ] Automated changelog generation from commit messages
- [ ] Homebrew tap for macOS installation
- [ ] APT/YUM repository for Linux distributions
- [ ] Automated announcement to Discord/Slack
- [ ] Release metrics and download tracking

## References

- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
