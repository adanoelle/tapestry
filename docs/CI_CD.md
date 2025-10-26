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

### Nix Environment Check Workflow (`.github/workflows/nix.yml`)

**Separate workflow** that validates the Nix development environment. Runs in parallel with main CI for faster feedback.

**Triggered by:**
- Pushes to `main` (only when flake files change)
- Pull requests (only when flake files change)
- Manual trigger via workflow_dispatch

**Jobs:**

1. **Nix Flake Validation** - Tests on Linux and macOS
   - Validates `flake.nix` structure with `nix flake check`
   - Tests that `nix develop` shell can be created
   - Verifies all development tools are available (Rust, Cargo, Clippy, etc.)
   - Builds the workspace within the Nix environment
   - Runs tests within the Nix environment
   - Ensures Nix users have a working development environment

**Caching Strategy:**
- Nix store (via Cachix) - dramatically speeds up subsequent runs
- First run: ~10-15 minutes (builds Rust toolchain)
- Cached runs: ~2-5 minutes

**Why separate?**
- Doesn't slow down standard CI (runs in parallel)
- Only runs when flake files change (efficient)
- Clear separation between standard and Nix environments
- Non-Nix users get faster feedback

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

### Nix Development Environment

The project provides a Nix flake for reproducible development environments.

**Testing the Nix environment:**

```bash
# Validate flake structure
nix flake check

# Enter dev shell
nix develop

# Test tools are available
rustc --version
cargo --version
cargo clippy --version
```

**CI validates the Nix environment on every PR:**

- Ensures `flake.nix` is valid
- Verifies all tools are available in the dev shell
- Builds and tests the project within the Nix environment
- Tests on both Linux and macOS

**Common Nix issues:**

```bash
# Flake won't build
nix flake check --show-trace  # Get detailed error info

# Update flake inputs
nix flake update

# Clean Nix cache if corrupted
nix-collect-garbage -d
```

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

### Nix Flake Issues

**Nix flake check fails:**

```bash
# Get detailed error trace locally
nix flake check --show-trace

# Common issues:
# 1. Undefined variables - ensure all variables are properly scoped with pkgs.*
# 2. Missing packages - verify package exists: nix search nixpkgs <package-name>
# 3. Syntax errors - check Nix expression syntax
```

**Dev shell fails to build:**

```bash
# Test locally
nix develop --show-trace

# Update flake inputs
nix flake update

# Clear build cache
nix-collect-garbage -d
nix develop
```

**Tools missing in dev shell:**

```bash
# Verify tool is in buildInputs in flake.nix
# Test if available:
nix develop --command which <tool-name>

# Test tool runs:
nix develop --command <tool-name> --version
```

**CI Nix job slow:**

- Cachix is configured to cache Nix derivations
- First run will be slow (~10-15 min) while building Rust toolchain
- Subsequent runs should be much faster (~2-3 min)
- Check Cachix cache hit rate if consistently slow

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

**Nix Workflow (separate):**

- Nix Check (first run): < 15 minutes
- Nix Check (cached): < 3 minutes
- Runs in parallel with CI

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

6. **Test Nix flake changes locally**
   - Run `nix flake check` before pushing
   - Test `nix develop` works
   - Saves CI time on Nix jobs

## Performance Monitoring

Tapestry provides scripts to monitor CI performance over time, helping you track trends and detect regressions.

### Monitor Current Performance

Use `monitor-ci-performance.sh` to analyze recent workflow runs:

```bash
# Quick check - last 10 runs
./scripts/monitor-ci-performance.sh --limit 10

# Detailed report - last 30 runs
./scripts/monitor-ci-performance.sh --limit 30

# Analyze specific workflow
./scripts/monitor-ci-performance.sh --workflow "CI" --limit 30

# Compare CI and Nix workflows
./scripts/monitor-ci-performance.sh --compare "CI" "Nix Environment Check"

# Export data for analysis
./scripts/monitor-ci-performance.sh --limit 50 --export metrics.csv
```

**Output includes:**
- Success rate statistics
- Duration metrics (average, median, P95, P99)
- Cache hit rate estimates (for Nix workflows)
- Performance assessment (excellent/good/needs attention)
- Recent failures

### Track Trends Over Time

Use `track-ci-trends.sh` to capture daily metrics:

```bash
# Track all workflows
./scripts/track-ci-trends.sh

# Track specific workflow
./scripts/track-ci-trends.sh "CI"

# View historical trends
cat .github/ci-metrics.csv | column -t -s,
```

**The script will:**
- Capture metrics to `.github/ci-metrics.csv`
- Detect performance regressions (>20% slower than baseline)
- Alert if P95 exceeds 15 minutes
- Track cache hit rates over time

### Recommended Monitoring Schedule

**Weekly (manual):**
```bash
# Monday morning check
./scripts/monitor-ci-performance.sh --limit 30
```

**Daily (optional automation):**
```bash
# Add to cron or run manually
./scripts/track-ci-trends.sh
git add .github/ci-metrics.csv
git commit -m "chore: update CI metrics"
```

### Performance Targets

**Median duration:**
- ✅ Excellent: < 5 minutes
- ⚠️ Acceptable: < 10 minutes
- ❌ Needs attention: > 10 minutes

**P95 duration:**
- ✅ Excellent: < 10 minutes
- ⚠️ Acceptable: < 15 minutes
- ❌ Needs attention: > 15 minutes

**Success rate:**
- ✅ Excellent: > 95%
- ⚠️ Acceptable: > 90%
- ❌ Needs attention: < 90%

**Cache hit rate (Nix workflows):**
- ✅ Excellent: > 80%
- ⚠️ Acceptable: > 60%
- ❌ Needs attention: < 60%

### Interpreting Results

**Fast median, high P95:**
- Cache is working well
- Occasional cache misses or flake updates
- Normal behavior for Nix workflows

**Increasing trend:**
- Dependencies growing
- Flake.lock updates causing rebuilds
- Consider optimization

**High failure rate:**
- Flaky tests
- Environmental issues
- Code quality concerns

### When to Investigate

**Immediate action needed:**
- Median > 15 minutes
- P95 > 20 minutes
- Success rate < 85%
- Regression > 50% week-over-week

**Monitor closely:**
- Gradual upward trend in median
- Increasing cache miss rate
- Success rate declining

**Acceptable:**
- Temporary spikes in P95 (cache misses)
- Minor variations day-to-day
- Stable median over time

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [GitHub CLI Documentation](https://cli.github.com/manual/)
