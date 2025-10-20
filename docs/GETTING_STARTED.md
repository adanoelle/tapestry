# Getting Started with Tapestry

This guide will help you install and start using Tapestry's development tools.

## Prerequisites

- **For Nix users**: [Nix](https://nixos.org/) with flakes enabled
- **For non-Nix users**:
  - [Rust](https://rustup.rs/) (stable toolchain)
  - [Git](https://git-scm.com/)
  - Basic development tools (build-essential on Linux, Xcode on macOS)

## Installation

### Option 1: Nix (Recommended)

The Nix flake provides a complete, reproducible development environment with
automatic setup.

#### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/tapestry.git
cd tapestry
```

#### 2. Enter the Development Environment

```bash
nix develop
```

This automatically installs:

- Complete Rust toolchain with clippy, rustfmt, rust-analyzer
- Claude Code (always up-to-date)
- Development tools (cargo-watch, cargo-audit, cargo-llvm-cov)
- Git hooks for quality checks
- Cross-compilation targets
- All project dependencies

You'll see a welcome banner with available commands and tool versions.

#### 3. Build and Verify

```bash
# Build the workspace
cargo build --release

# Run tests
cargo test

# Try the RFD CLI
cd cli/rfd
cargo run -- --help
```

#### 4. Optional: Configure Cachix

For faster installations with pre-built binaries:

**On NixOS**, add to `/etc/nixos/configuration.nix`:

```nix
{
  nix.settings = {
    substituters = [
      "https://cache.nixos.org"
      "https://claude-code.cachix.org"
    ];
    trusted-public-keys = [
      "claude-code.cachix.org-1:YeXf2aNu7UTX8Vwrze0za1WEDS+4DuI2kVeWEE4fsRk="
    ];
  };
}
```

Then: `sudo nixos-rebuild switch`

**On other systems**, add to `~/.config/nix/nix.conf`:

```
extra-substituters = https://claude-code.cachix.org
extra-trusted-public-keys = claude-code.cachix.org-1:YeXf2aNu7UTX8Vwrze0za1WEDS+4DuI2kVeWEE4fsRk=
```

### Option 2: Manual Setup (Without Nix)

#### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. Install Additional Tools (Optional)

```bash
# Code quality tools
rustup component add clippy rustfmt

# Development helpers
cargo install cargo-watch cargo-audit cargo-llvm-cov cargo-edit
```

#### 3. Clone and Setup

```bash
# Clone repository
git clone https://github.com/yourusername/tapestry.git
cd tapestry

# Run setup script (installs git hooks, verifies tools)
./scripts/setup-dev.sh
```

#### 4. Build and Verify

```bash
# Build the workspace
cargo build --release

# Run tests
cargo test

# Try the RFD CLI
cd cli/rfd
cargo run -- --help
```

## Using Tapestry Tools

### RFD CLI - Document Management

The RFD CLI helps you manage Request for Discussion (RFD) documents.

#### Create Your First RFD

```bash
cd cli/rfd

# Create a new RFD
cargo run -- create \
  --title "My First Technical Proposal" \
  --author "Your Name <your.email@example.com>"

# The RFD is created in ./rfds/ directory
```

#### List RFDs

```bash
# Pretty output
cargo run -- list

# JSON output (for scripts/AI agents)
cargo run -- list --format json
```

#### View an RFD

```bash
# Show RFD details
cargo run -- show 0001

# JSON output
cargo run -- show 0001 --format json
```

#### Update RFD Status

```bash
# Move to discussion phase
cargo run -- status 0001 discussion

# Mark as accepted
cargo run -- status 0001 committed
```

For complete RFD CLI documentation, see
[cli/rfd/README.md](../cli/rfd/README.md).

## Development Workflow

### Making Changes

1. **Create a branch**

   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make your changes**

   - Edit code
   - Run tests: `cargo test`
   - Format code: `cargo fmt`
   - Lint code: `cargo clippy`

3. **Commit with conventional commits**

   ```bash
   git commit -m "feat(rfd): add search command"
   ```

   The commit-msg hook will validate your message format.

4. **Push and create PR**
   ```bash
   git push origin feature/my-feature
   # Then create a pull request on GitHub
   ```

### Git Hooks

Git hooks run automatically to ensure code quality:

- **Pre-commit**: Runs format check, clippy, tests
- **Commit-msg**: Validates conventional commit format

To bypass (use sparingly):

```bash
git commit --no-verify -m "message"
```

See [CI_CD.md](CI_CD.md) for details on hooks and CI/CD.

### Continuous Integration

When you push or create a PR, GitHub Actions will:

- Check code formatting
- Run clippy linting
- Execute tests on Linux, macOS, Windows
- Generate code coverage
- Check for security vulnerabilities

All checks must pass before merging.

## Building for Distribution

### Development Build

```bash
cargo build
# Binary in: target/debug/rfd
```

### Optimized Release Build

```bash
cargo build --release
# Binary in: target/release/rfd
# ~2.4MB, optimized for size and startup time
```

### Cross-Platform Builds

Using the Nix environment with cross-compilation targets:

```bash
# Linux (musl, static)
cargo build --release --target x86_64-unknown-linux-musl

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-gnu
```

For automated releases, see [CI_CD.md](CI_CD.md#release-process).

## Installing Tapestry Tools

### From Source (Current Method)

```bash
cd cli/rfd
cargo install --path .
# Installs to ~/.cargo/bin/rfd
```

### From crates.io (Future)

Once published:

```bash
cargo install rfd
```

### Pre-built Binaries (Future)

Download from GitHub Releases:

- Linux (x86_64, glibc and musl)
- macOS (Intel and Apple Silicon)
- Windows (x86_64)

All binaries include SHA256 checksums for verification.

## Updating

### Nix Users

```bash
# Update flake inputs (gets latest Claude Code, etc.)
nix flake update

# Re-enter shell
exit
nix develop
```

### Manual Users

```bash
# Update Rust toolchain
rustup update

# Update Tapestry
git pull origin main
cargo build --release
```

## Troubleshooting

### Nix Development Shell

**Problem**: Git hooks not installed

```bash
# Verify hooks path
git config core.hooksPath
# Should output: .githooks

# Re-enter shell
exit
nix develop
```

**Problem**: Claude Code not found

```bash
# Check if in Nix shell
echo $IN_NIX_SHELL

# Verify Claude Code
which claude

# Try rebuilding
nix develop --recreate-lock-file
```

### Build Issues

**Problem**: Compilation fails

```bash
# Clean and rebuild
cargo clean
cargo build

# Check Rust version
rustc --version
# Should be 1.70+
```

**Problem**: Tests fail

```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Check for uncommitted changes
git status
```

### Git Hook Issues

**Problem**: Pre-commit hook is slow

The hook runs format check, clippy, and all tests. For faster commits during
development, you can:

1. Temporarily bypass: `git commit --no-verify`
2. Comment out tests in `.githooks/pre-commit` (CI will still run them)
3. Use `cargo test --lib` for faster feedback

**Problem**: Commit message rejected

Ensure you're using conventional commits format:

```bash
# Good
git commit -m "feat(rfd): add search command"
git commit -m "fix(cli): handle missing config file"
git commit -m "docs: update installation guide"

# Bad
git commit -m "updates"
git commit -m "WIP"
git commit -m "fix stuff"
```

## Next Steps

### Using Tapestry

- **[RFD CLI Guide](../cli/rfd/README.md)** - Complete RFD CLI documentation
- **[Architecture](ARCHITECTURE.md)** - Understanding Tapestry's design
- **[CI/CD Guide](CI_CD.md)** - Releases, hooks, and automation

### Contributing

- **[Contributing Guide](design/meta/CONTRIBUTING.md)** - How to contribute
- **[Roadmap](ROADMAP.md)** - What we're building
- **[RFC Process](design/meta/CONTRIBUTING.md#rfc-process)** - Proposing
  features

### Community

- **Questions?**
  [GitHub Discussions](https://github.com/yourusername/tapestry/discussions)
- **Bug reports?**
  [GitHub Issues](https://github.com/yourusername/tapestry/issues)
- **Feature ideas?** Write an RFC!

## Additional Resources

- **[Claude Code Integration](CLAUDE_CODE_INTEGRATION.md)** - How we stay
  current with Claude Code
- **[Project Vision](VISION.md)** - Why Tapestry exists
- **[Performance Targets](ARCHITECTURE.md#performance-targets)** - Benchmarks
  and goals

---

Welcome to Tapestry! We're excited to have you here. If you run into any issues,
please don't hesitate to ask for help in our
[discussions](https://github.com/yourusername/tapestry/discussions).
