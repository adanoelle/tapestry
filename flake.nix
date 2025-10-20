{
  description = "Tapestry - Claude Code development tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

    # Claude Code with automatic updates and binary cache
    # See: https://github.com/sadjow/claude-code-nix
    claude-code-nix = {
      url = "github:sadjow/claude-code-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, claude-code-nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [
            "x86_64-unknown-linux-gnu"
            "x86_64-unknown-linux-musl"
            "x86_64-apple-darwin"
            "aarch64-apple-darwin"
            "x86_64-pc-windows-gnu"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain

            # Development tools
            gh                    # GitHub CLI
            git

            # Rust development tools
            cargo-watch           # Auto-rebuild on file changes
            cargo-audit           # Security vulnerability scanning
            cargo-llvm-cov        # Code coverage
            cargo-edit            # Easily upgrade dependencies

            # MCP development
            nodejs_20             # For MCP testing tools

            # Documentation
            mdbook                # For potential docs site

            # System dependencies
            pkg-config
            openssl

            # CLI utilities
            jq                    # JSON processing (useful for testing CLI JSON output)
            ripgrep               # Fast searching
            fd                    # Fast file finding
            figlet                # ASCII art banners

            # AI assistance
            # Claude Code with automatic updates via sadjow/claude-code-nix
            # Updates hourly from npm, with Cachix binary cache
            claude-code-nix.packages.${system}.default
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # macOS-specific dependencies
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            # Linux-specific dependencies for static builds
            pkgs.musl
          ];
          
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          
          shellHook = ''
            # Set up git hooks
            git config core.hooksPath .githooks 2>/dev/null || true
            chmod +x .githooks/* 2>/dev/null || true

            # Environment variables
            export RUST_BACKTRACE=1
            export CARGO_TERM_COLOR=always

            # Create necessary directories
            mkdir -p .cache

            echo ""
            figlet -f slant "Tapestry" 2>/dev/null || echo "🧵 TAPESTRY"
            echo ""
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "  AI-Native Development Tools"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo ""
            echo "🔧 Development Environment"
            echo "  Rust:        $(rustc --version)"
            echo "  Cargo:       $(cargo --version)"
            echo "  Clippy:      $(cargo clippy --version 2>/dev/null || echo 'not found')"
            echo "  Claude Code: $(claude --version 2>/dev/null || echo 'checking...')"
            echo "  GitHub CLI:  $(gh --version | head -1)"
            echo "  Git hooks:   ✓ Installed (.githooks/)"
            echo ""
            echo "📚 Quick Start"
            echo "  cargo build              Build the project"
            echo "  cargo test               Run all tests"
            echo "  cargo clippy             Lint code"
            echo "  cargo fmt                Format code"
            echo ""
            echo "🚀 Advanced"
            echo "  cargo watch -x test      Auto-run tests on changes"
            echo "  cargo audit              Check for vulnerabilities"
            echo "  cargo llvm-cov           Generate code coverage"
            echo ""
            echo "🛠️  Tools"
            echo "  cd cli/rfd && cargo run  Run RFD CLI tool"
            echo ""
            echo "📖 Documentation"
            echo "  docs/CI_CD.md            CI/CD and release process"
            echo "  docs/VISION.md           Project vision"
            echo "  CLAUDE.md                Quick reference for Claude"
            echo ""
            echo "💡 Tip: Non-Nix users can run ./scripts/setup-dev.sh"
            echo ""
          '';
        };
      });
}
