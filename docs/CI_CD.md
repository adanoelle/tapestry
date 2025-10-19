# CI/CD and Release Process

This document describes the continuous integration, continuous deployment, and
release processes for the Tapestry project.

## Table of Contents

- [Overview](#overview)
- [GitHub Actions Workflows](#github-actions-workflows)
- [Git Hooks](#git-hooks)
- [Release Process](#release-process)
- [Binary Distribution](#binary-distribution)
- [Troubleshooting](#troubleshooting)

## Overview

Tapestry uses GitHub Actions for CI/CD and provides local git hooks for
pre-commit validation. The system is designed to:

- Ensure code quality through automated checks
- Test across multiple platforms (Linux, macOS, Windows)
- Build optimized release binaries for distribution
- Publish releases to GitHub and crates.io

## GitHub Actions Workflows

### CI Workflow (`.github/workflows/ci.yml`)

Runs on every push to `main` and on all pull requests.

**Jobs:**

1. **Check** - Validates that code compiles

   - Runs `cargo check` on the entire workspace
   - Fast feedback for compilation errors

2. **Format** - Ensures consistent code style

   - Runs `rustfmt` on all Rust files
   - Fails if code is not formatted correctly
   - Fix with: `cargo fmt --all`

3. **Clippy** - Linting and code quality

   - Runs Clippy with `-D warnings` (all warnings are errors)
   - Catches common mistakes and anti-patterns
   - Fix with: `cargo clippy --workspace --all-targets --all-features --fix`

4. **Test** - Cross-platform testing

   - Tests on Linux, macOS, and Windows
   - Tests with stable and beta Rust toolchains
   - Runs both unit tests and doc tests
   - Matrix strategy ensures compatibility

5. **Coverage** - Code coverage reporting

   - Uses `cargo-llvm-cov` to generate coverage reports
   - Uploads to Codecov (requires `CODECOV_TOKEN` secret)
   - Non-blocking (won't fail CI if upload fails)

6. **Build** - Build release binaries

   - Builds optimized binaries for all supported platforms
   - Uploads artifacts for download
   - Verifies release builds work correctly

7. **Audit** - Security vulnerability scanning
   - Uses `cargo-audit` to check dependencies
   - Fails on known security vulnerabilities
   - Runs on every CI build

**Caching Strategy:**

All jobs use GitHub Actions cache to speed up builds:

- Cargo registry (`~/.cargo/registry`)
- Cargo index (`~/.cargo/git`)
- Build artifacts (`target/`)

### Release Workflow (`.github/workflows/release.yml`)

Triggered when you push a tag matching `v*.*.*` (e.g., `v1.0.0`,
`v1.0.0-beta.1`).

**Jobs:**

1. **Create Release**

   - Creates a GitHub release from the tag
   - Marks as pre-release if version contains `-` (e.g., `-beta`, `-rc`)
   - Generates release notes template

2. **Build Release**

   - Builds optimized binaries for all platforms:
     - Linux: x86_64 (glibc), x86_64 (musl), aarch64
     - macOS: x86_64 (Intel), aarch64 (Apple Silicon)
     - Windows: x86_64
   - Creates archives (.tar.gz for Unix, .zip for Windows)
   - Generates SHA256 checksums
   - Uploads all assets to the GitHub release

3. **Publish Crate**
   - Publishes the `rfd` CLI to crates.io
   - Requires `CARGO_TOKEN` secret
   - Non-blocking (continues if already published)

**Binary Optimization:**

Release binaries are optimized for size and startup time:

- LTO (Link-Time Optimization) enabled
- Single codegen unit for maximum optimization
- Size optimization (`opt-level = "z"`)
- Debug symbols stripped
- Panic set to "abort" for smaller binaries

## Git Hooks

Tapestry provides custom git hooks in `.githooks/` for local development.

### Installation

#### With Nix (Automatic)

If you're using the Nix development environment, git hooks are automatically
installed when you run `nix develop`. No manual setup required!

#### Without Nix (Manual)

Enable the hooks directory:

```bash
git config core.hooksPath .githooks
```

Or run the setup script:

```bash
./scripts/setup-dev.sh
```

### Pre-commit Hook (`.githooks/pre-commit`)

Runs automatically before each commit to ensure code quality.

**Checks:**

1. **Formatting** - `cargo fmt --all -- --check`

   - Ensures code is properly formatted
   - Fix with: `cargo fmt --all`

2. **Clippy** -
   `cargo clippy --workspace --all-targets --all-features -- -D warnings`

   - Lints code for common issues
   - All warnings treated as errors

3. **Tests** - `cargo test --workspace --all-features`

   - Runs all tests in the workspace
   - Prevents broken code from being committed

4. **Common Issues**
   - Warns about debug prints (`println!`, `dbg!`)
   - Notes TODO/FIXME comments
   - Prompts for confirmation if issues found

**Bypass (use sparingly):**

```bash
git commit --no-verify -m "message"
```

### Commit Message Hook (`.githooks/commit-msg`)

Validates commit messages follow
[Conventional Commits](https://www.conventionalcommits.org/) format.

**Format:**

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes
- `build`: Build system changes

**Scope (optional):**

- `rfd` - RFD CLI tool
- `git-workflow` - Git workflow tool
- etc.

**Examples:**

```bash
# Good
git commit -m "feat(rfd): add create command"
git commit -m "fix(rfd): handle missing config file"
git commit -m "docs: update installation instructions"
git commit -m "chore: bump dependencies"

# Breaking change
git commit -m "feat(rfd)!: change config file format"

# Bad (will be rejected)
git commit -m "add feature"
git commit -m "WIP"
git commit -m "updates"
```

**Exceptions:**

The hook automatically allows:

- Merge commits
- Commits co-authored by Claude Code

## Release Process

### Creating a Release

1. **Update Version**

   Update the version in `cli/rfd/Cargo.toml`:

   ```toml
   [package]
   version = "1.0.0"  # Update this
   ```

2. **Update Changelog**

   Update `CHANGELOG.md` with the new version and changes:

   ```markdown
   ## [1.0.0] - 2025-10-19

   ### Added

   - New feature X
   - New feature Y

   ### Fixed

   - Bug fix A
   - Bug fix B
   ```

3. **Commit Changes**

   ```bash
   git add cli/rfd/Cargo.toml CHANGELOG.md
   git commit -m "chore(rfd): bump version to 1.0.0"
   git push
   ```

4. **Create and Push Tag**

   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```

5. **Monitor Release Workflow**

   - Go to GitHub Actions tab
   - Watch the "Release" workflow
   - Verify all jobs complete successfully
   - Check that binaries are uploaded to the release

6. **Update Release Notes**

   - Go to the GitHub release
   - Edit the auto-generated release notes
   - Add a proper description of changes
   - Highlight breaking changes if any

### Pre-releases

For beta/RC releases:

```bash
# Update version to include pre-release identifier
# In cli/rfd/Cargo.toml:
version = "1.0.0-beta.1"

# Tag with pre-release identifier
git tag -a v1.0.0-beta.1 -m "Beta release v1.0.0-beta.1"
git push origin v1.0.0-beta.1
```

The release workflow will automatically mark it as a pre-release.

### Hotfix Releases

For patch releases:

```bash
# Create hotfix branch from main
git checkout -b hotfix/v1.0.1 main

# Make fixes
git commit -m "fix(rfd): critical bug fix"

# Update version
# In cli/rfd/Cargo.toml:
version = "1.0.1"

# Commit and tag
git commit -m "chore(rfd): bump version to 1.0.1"
git tag -a v1.0.1 -m "Hotfix v1.0.1"

# Push
git push origin hotfix/v1.0.1 --tags

# Merge back to main
git checkout main
git merge hotfix/v1.0.1
git push
```

## Binary Distribution

### Download from GitHub Releases

Binaries are available on the
[Releases page](https://github.com/yourusername/tapestry/releases).

**Linux (glibc):**

```bash
curl -LO https://github.com/yourusername/tapestry/releases/latest/download/rfd-linux-amd64.tar.gz
tar xzf rfd-linux-amd64.tar.gz
sudo mv rfd /usr/local/bin/
```

**Linux (musl, static):**

```bash
curl -LO https://github.com/yourusername/tapestry/releases/latest/download/rfd-linux-musl-amd64.tar.gz
tar xzf rfd-linux-musl-amd64.tar.gz
sudo mv rfd /usr/local/bin/
```

**macOS (Intel):**

```bash
curl -LO https://github.com/yourusername/tapestry/releases/latest/download/rfd-macos-amd64.tar.gz
tar xzf rfd-macos-amd64.tar.gz
sudo mv rfd /usr/local/bin/
```

**macOS (Apple Silicon):**

```bash
curl -LO https://github.com/yourusername/tapestry/releases/latest/download/rfd-macos-arm64.tar.gz
tar xzf rfd-macos-arm64.tar.gz
sudo mv rfd /usr/local/bin/
```

**Windows:** Download `rfd-windows-amd64.exe.zip` and extract to a directory in
your PATH.

### Verify Downloads

All releases include SHA256 checksums:

```bash
# Download binary and checksum
curl -LO https://github.com/yourusername/tapestry/releases/latest/download/rfd-linux-amd64.tar.gz
curl -LO https://github.com/yourusername/tapestry/releases/latest/download/rfd-linux-amd64.tar.gz.sha256

# Verify
sha256sum -c rfd-linux-amd64.tar.gz.sha256
```

### Install from crates.io

```bash
cargo install rfd
```

### Future: Package Managers

**Homebrew (planned):**

```bash
brew tap yourusername/tapestry
brew install rfd
```

**Scoop (Windows, planned):**

```powershell
scoop bucket add tapestry https://github.com/yourusername/scoop-tapestry
scoop install rfd
```

## Troubleshooting

### CI Failures

**Formatting failures:**

```bash
cargo fmt --all
git add .
git commit --amend --no-edit
git push --force-with-lease
```

**Clippy failures:**

```bash
cargo clippy --workspace --all-targets --all-features --fix
git add .
git commit -m "fix: address clippy warnings"
```

**Test failures:**

```bash
# Run tests locally to debug
cargo test --workspace --all-features

# Run specific test
cargo test test_name -- --nocapture
```

### Hook Issues

**Hooks not running:**

```bash
# Verify hooks are installed
git config core.hooksPath
# Should output: .githooks

# Reinstall hooks
git config core.hooksPath .githooks

# Make hooks executable
chmod +x .githooks/*
```

**Slow pre-commit hook:**

```bash
# Skip clippy in hook for faster commits
# Edit .githooks/pre-commit and comment out the clippy step
# (CI will still run it)
```

### Release Issues

**Release workflow fails:**

- Ensure the tag follows `v*.*.*` format
- Check that `GITHUB_TOKEN` has proper permissions
- Verify `CARGO_TOKEN` secret is set (for crates.io publish)

**Binary build fails:**

- Check the build matrix in `.github/workflows/release.yml`
- Verify cross-compilation dependencies are correct
- Test locally: `cargo build --release --target <target-triple>`

**Checksum verification fails:**

- Re-download the binary and checksum
- Ensure you're using the same checksum algorithm (SHA256)
- Report the issue if the checksum is incorrect

## Secrets Configuration

Required GitHub secrets:

1. **CODECOV_TOKEN** (optional)

   - For code coverage uploads
   - Get from https://codecov.io
   - Settings → Secrets → New repository secret

2. **CARGO_TOKEN** (required for crates.io publish)
   - Generate at https://crates.io/settings/tokens
   - Settings → Secrets → New repository secret

## Performance Targets

**CI Workflow:**

- Check: < 2 minutes
- Format: < 30 seconds
- Clippy: < 3 minutes
- Test (per platform): < 5 minutes
- Total CI time: < 10 minutes (parallel jobs)

**Release Workflow:**

- Build per platform: < 5 minutes
- Total release time: < 10 minutes (parallel builds)

**Binary Sizes (stripped):**

- Linux/macOS: < 3 MB
- Windows: < 3 MB

## Best Practices

1. **Always run hooks before pushing**

   - Saves CI time and resources
   - Catches issues early

2. **Test on multiple platforms locally**

   - Use Docker for Linux targets
   - Use GitHub Actions for full matrix testing

3. **Keep changelog updated**

   - Update with each PR
   - Makes releases easier

4. **Use semantic versioning**

   - MAJOR.MINOR.PATCH
   - Breaking changes → MAJOR
   - New features → MINOR
   - Bug fixes → PATCH

5. **Write good commit messages**
   - Follow Conventional Commits
   - Makes changelogs easier to generate
   - Helps with automated releases

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
