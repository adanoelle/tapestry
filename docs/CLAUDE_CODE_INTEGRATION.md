# Claude Code Integration with Nix

This document explains how Claude Code is integrated into the Tapestry
development environment via Nix flakes.

## Overview

Tapestry uses the
[`sadjow/claude-code-nix`](https://github.com/sadjow/claude-code-nix) flake to
provide always-up-to-date Claude Code in the development environment.

## Why This Approach?

### The Problem with Traditional Package Managers

Claude Code is distributed via npm and updates frequently. Traditional
approaches have issues:

1. **Manual Installation**: Installing via npm globally requires Node.js and
   manual updates
2. **Version Conflicts**: System-wide Node.js versions can conflict with project
   requirements
3. **Stale Packages**: Package manager versions (like in nixpkgs-unstable) lag
   behind npm releases
4. **Update Burden**: Manual checking and updating is tedious

### The Solution: sadjow/claude-code-nix

This flake provides several advantages:

1. **Automatic Updates**: Checks npm hourly and builds new versions within 30
   minutes of release
2. **Bundled Runtime**: Includes Node.js 22 LTS, so no system Node.js conflicts
3. **Binary Cache**: Pre-built binaries via Cachix for instant installation
4. **Nix Integration**: Works seamlessly with NixOS, Home Manager, and dev
   shells
5. **No Manual Updates**: Just `nix flake update` to get the latest version

## How It Works

### Flake Integration

In `flake.nix`, we add Claude Code as an input:

```nix
{
  inputs = {
    # ... other inputs ...

    # Claude Code with automatic updates
    claude-code-nix = {
      url = "github:sadjow/claude-code-nix";
      inputs.nixpkgs.follows = "nixpkgs";  # Use our nixpkgs version
    };
  };

  outputs = { self, nixpkgs, claude-code-nix, ... }: {
    devShells.default = pkgs.mkShell {
      buildInputs = [
        # ... other packages ...

        # Claude Code - always up to date!
        claude-code-nix.packages.${system}.default
      ];
    };
  };
}
```

### Update Mechanism

The `sadjow/claude-code-nix` repository:

1. Runs GitHub Actions hourly to check npm for new Claude Code versions
2. Automatically creates commits with version bumps
3. Publishes pre-built binaries to Cachix
4. You pull the latest with `nix flake update`

## Usage

### Quick Start

```bash
# Enter the development environment (Claude Code included)
nix develop

# Verify Claude Code is available
claude --version

# Start Claude Code
claude
```

### Updating Claude Code

To get the latest version:

```bash
# Update all flake inputs (including Claude Code)
nix flake update

# Or update just Claude Code
nix flake lock --update-input claude-code-nix

# Rebuild the dev shell
nix develop
```

### Using Cachix (Recommended)

For faster installs, configure the Cachix binary cache:

#### One-time Setup (NixOS)

Add to `/etc/nixos/configuration.nix`:

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

Then rebuild: `sudo nixos-rebuild switch`

#### One-time Setup (Non-NixOS with Cachix CLI)

```bash
# Install cachix
nix-env -iA cachix -f https://cachix.org/api/v1/install

# Use the claude-code cache
cachix use claude-code
```

#### Manual Configuration (nix.conf)

Add to `~/.config/nix/nix.conf` or `/etc/nix/nix.conf`:

```
extra-substituters = https://claude-code.cachix.org
extra-trusted-public-keys = claude-code.cachix.org-1:YeXf2aNu7UTX8Vwrze0za1WEDS+4DuI2kVeWEE4fsRk=
```

### Checking Version

```bash
# In the dev shell
claude --version

# Or directly via nix
nix run github:sadjow/claude-code-nix -- --version
```

## Alternative Installation Methods

If you don't want Claude Code in your dev shell, you can use it other ways:

### Run Once (No Installation)

```bash
nix run github:sadjow/claude-code-nix
```

### Install to User Profile

```bash
nix profile install github:sadjow/claude-code-nix
```

### Home Manager

Add to your `home.nix`:

```nix
{
  home.packages = [
    inputs.claude-code-nix.packages.${pkgs.system}.default
  ];
}
```

## Troubleshooting

### "Claude symlink points to invalid binary"

This is a **false positive warning**. Nix packages Claude Code as a wrapper
script, not a direct symlink. The warning can be safely ignored - Claude Code
works correctly.

### Claude Code Not Found in Shell

1. Verify you're in the Nix development shell: `echo $IN_NIX_SHELL`
2. Check if Claude Code is in PATH: `which claude`
3. Rebuild the dev shell: `nix develop --recreate-lock-file`

### PATH Issues with `nix develop`

Recent versions of Claude Code may have issues seeing the correct PATH in nix
shells. Workarounds:

1. **Use direnv** (recommended): Automatically loads the dev shell
2. **Explicit PATH**: Add to `shellHook` in `flake.nix`
3. **Report upstream**: File an issue at
   https://github.com/anthropics/claude-code

### Slow Installation

If installation is slow, ensure you have the Cachix binary cache configured (see
above). Without it, Nix will build Claude Code from source.

### Old Version After Update

After `nix flake update`, you may need to:

```bash
# Exit and re-enter the dev shell
exit
nix develop

# Or force garbage collection first
nix-collect-garbage -d
nix develop
```

## Benefits for Tapestry

### For Developers

1. **Zero Setup**: Just `nix develop` and Claude Code is ready
2. **Always Current**: Automatic updates mean you get new features fast
3. **Reproducible**: Flake lock file ensures everyone uses the same version
4. **No Conflicts**: Bundled Node.js prevents version conflicts

### For AI-Assisted Development

1. **Consistent Environment**: All contributors have the same Claude Code
   version
2. **Fast Onboarding**: New contributors get Claude Code automatically
3. **CI/CD Compatible**: Can use the same Claude Code version in CI (future)
4. **MCP Integration**: Works seamlessly with our MCP tools

## Version Pinning

To pin to a specific version (not recommended, but possible):

```nix
{
  inputs = {
    claude-code-nix = {
      url = "github:sadjow/claude-code-nix/COMMIT_HASH";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

Get the commit hash from: https://github.com/sadjow/claude-code-nix/commits/main

## Comparison with Other Approaches

| Approach                   | Updates       | Node.js     | Binary Cache | Ease of Use |
| -------------------------- | ------------- | ----------- | ------------ | ----------- |
| npm global                 | Manual        | System      | No           | Medium      |
| nixpkgs-unstable           | Delayed       | System      | Yes          | Easy        |
| **sadjow/claude-code-nix** | **Automatic** | **Bundled** | **Yes**      | **Easy**    |
| Manual download            | Manual        | None        | No           | Hard        |

## Future Enhancements

Potential improvements to our Claude Code integration:

1. **Auto-update in CI**: Keep CI environments current
2. **Version notifications**: Notify on new Claude Code releases
3. **MCP Server Presets**: Use `roman/claude-code.nix` for predefined MCP
   configs
4. **Home Manager Integration**: System-wide Claude Code for all Tapestry
   projects

## References

- [sadjow/claude-code-nix GitHub](https://github.com/sadjow/claude-code-nix)
- [Claude Code Official Docs](https://docs.claude.com/en/docs/claude-code)
- [Cachix Documentation](https://docs.cachix.org/)
- [Nix Flakes Documentation](https://nixos.wiki/wiki/Flakes)
- [DEV Article: Claude Code with Nix Flakes](https://dev.to/sadjow/claude-code-properly-packaged-and-always-fresh-with-nix-flakes-1ma8)

## Contributing

If you encounter issues with the Claude Code integration:

1. Check this document first
2. Check
   [sadjow/claude-code-nix issues](https://github.com/sadjow/claude-code-nix/issues)
3. For Tapestry-specific issues, open an issue in our repo
4. For Claude Code bugs, report at
   [anthropics/claude-code](https://github.com/anthropics/claude-code/issues)

---

**Summary**: We use `sadjow/claude-code-nix` for automatic, always-up-to-date
Claude Code in our dev environment. It's fast (Cachix cache), easy (automatic
setup), and stays current (hourly npm checks). Just `nix develop` and start
coding with Claude!
