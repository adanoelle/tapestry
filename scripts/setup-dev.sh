#!/bin/bash
#
# Developer setup script for Tapestry
# Sets up git hooks and installs necessary tools
#

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}=== Tapestry Developer Setup ===${NC}\n"

# 1. Install git hooks
echo -e "${YELLOW}[1/4] Installing git hooks...${NC}"
if git config core.hooksPath .githooks; then
    echo -e "${GREEN}✓ Git hooks installed${NC}"
else
    echo -e "${RED}✗ Failed to install git hooks${NC}"
    exit 1
fi

# Make hooks executable
chmod +x .githooks/*
echo -e "${GREEN}✓ Hooks made executable${NC}"

# 2. Check for required tools
echo -e "\n${YELLOW}[2/4] Checking for required tools...${NC}"

check_tool() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✓ $1 found${NC}"
        return 0
    else
        echo -e "${YELLOW}✗ $1 not found${NC}"
        return 1
    fi
}

missing_tools=()

check_tool "cargo" || missing_tools+=("cargo (Rust)")
check_tool "rustfmt" || missing_tools+=("rustfmt")
check_tool "cargo-clippy" || missing_tools+=("clippy")

# 3. Install optional tools
echo -e "\n${YELLOW}[3/4] Installing optional development tools...${NC}"

install_optional_tool() {
    tool_name=$1
    cargo_name=$2

    if ! command -v "$tool_name" &> /dev/null; then
        echo -e "${YELLOW}Installing $tool_name...${NC}"
        if cargo install "$cargo_name"; then
            echo -e "${GREEN}✓ $tool_name installed${NC}"
        else
            echo -e "${YELLOW}⚠ Failed to install $tool_name (non-critical)${NC}"
        fi
    else
        echo -e "${GREEN}✓ $tool_name already installed${NC}"
    fi
}

install_optional_tool "cargo-watch" "cargo-watch"
install_optional_tool "cargo-audit" "cargo-audit"
install_optional_tool "cargo-llvm-cov" "cargo-llvm-cov"

# 4. Run initial checks
echo -e "\n${YELLOW}[4/4] Running initial workspace checks...${NC}"

echo -e "${YELLOW}Running cargo check...${NC}"
if cargo check --workspace; then
    echo -e "${GREEN}✓ Workspace compiles${NC}"
else
    echo -e "${YELLOW}⚠ Workspace has compilation issues${NC}"
fi

echo -e "${YELLOW}Running cargo test...${NC}"
if cargo test --workspace; then
    echo -e "${GREEN}✓ All tests pass${NC}"
else
    echo -e "${YELLOW}⚠ Some tests fail${NC}"
fi

# Summary
echo -e "\n${BLUE}=== Setup Complete ===${NC}\n"

if [ ${#missing_tools[@]} -eq 0 ]; then
    echo -e "${GREEN}✓ All required tools are installed${NC}"
else
    echo -e "${YELLOW}⚠ Missing required tools:${NC}"
    for tool in "${missing_tools[@]}"; do
        echo -e "  - $tool"
    done
    echo ""
    echo "Please install missing tools before contributing."
    echo "Visit https://rustup.rs/ to install Rust and its tools."
fi

echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "  1. Read docs/CI_CD.md for CI/CD information"
echo "  2. Run 'cargo build' to build the project"
echo "  3. Run 'cargo test' to run tests"
echo "  4. Run 'cargo run --bin rfd -- --help' to test the RFD CLI"
echo ""
echo -e "${GREEN}Happy coding!${NC}"
