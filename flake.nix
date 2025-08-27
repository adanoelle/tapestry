{
  description = "Tapestry - Development Provenance Platform";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
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
            
            # MCP development
            nodejs_20             # For MCP testing tools
            
            # Documentation
            mdbook               # For potential docs site
            
            # System dependencies
            pkg-config
            openssl
            
            # Optional: Claude Code (if distributed via nix)
            # claude-code
          ];
          
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          
          shellHook = ''
            echo "🧶 Tapestry development environment"
            echo "Rust: $(rustc --version)"
            echo "GitHub CLI: $(gh --version | head -1)"
            
            # Set up git hooks if not already done
            if [ ! -f .git/hooks/pre-commit ]; then
              echo "Setting up git hooks..."
              # We can add pre-commit hooks here later
            fi
          '';
        };
      });
}
