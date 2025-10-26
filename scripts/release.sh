#!/usr/bin/env bash
#
# Release Helper Script
#
# Automates the release process for Tapestry:
# 1. Validates current state
# 2. Updates CHANGELOG.md
# 3. Bumps version in Cargo.toml
# 4. Creates release commit
# 5. Pushes to main (triggers auto-tag and release)
#
# Usage:
#   ./scripts/release.sh          # Interactive mode
#   ./scripts/release.sh 0.3.0    # Specify version
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

error() {
    echo -e "${RED}ERROR: $1${NC}" >&2
    exit 1
}

info() {
    echo -e "${BLUE}INFO: $1${NC}"
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

# Check we're on main branch
current_branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$current_branch" != "main" ]; then
    error "Must be on 'main' branch to release. Currently on: $current_branch"
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    error "Uncommitted changes detected. Commit or stash them first."
fi

# Check we're up to date with remote
info "Fetching latest from remote..."
git fetch origin main
if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
    error "Local main is not up to date with origin/main. Pull first."
fi

# Get current version
CARGO_TOML="cli/rfd/Cargo.toml"
current_version=$(grep -m1 '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/')
info "Current version: $current_version"

# Determine new version
if [ $# -eq 0 ]; then
    # Interactive mode
    echo ""
    echo -e "${BLUE}What type of release is this?${NC}"
    echo "  1) MAJOR (breaking changes)    - $current_version → $(echo "$current_version" | awk -F. '{print $1+1".0.0"}')"
    echo "  2) MINOR (new features)         - $current_version → $(echo "$current_version" | awk -F. '{print $1"."$2+1".0"}')"
    echo "  3) PATCH (bug fixes)            - $current_version → $(echo "$current_version" | awk -F. '{print $1"."$2"."$3+1}')"
    echo "  4) Custom version"
    echo ""
    read -p "Enter choice (1-4): " choice

    case $choice in
        1)
            new_version=$(echo "$current_version" | awk -F. '{print $1+1".0.0"}')
            ;;
        2)
            new_version=$(echo "$current_version" | awk -F. '{print $1"."$2+1".0"}')
            ;;
        3)
            new_version=$(echo "$current_version" | awk -F. '{print $1"."$2"."$3+1}')
            ;;
        4)
            read -p "Enter version (e.g., 1.0.0): " new_version
            ;;
        *)
            error "Invalid choice"
            ;;
    esac
else
    new_version="$1"
fi

# Validate version format
if ! [[ "$new_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
    error "Invalid version format: $new_version (expected: X.Y.Z or X.Y.Z-prerelease)"
fi

info "New version will be: $new_version"

# Check if tag already exists
if git rev-parse "v$new_version" >/dev/null 2>&1; then
    error "Tag v$new_version already exists!"
fi

# Check if CHANGELOG has unreleased changes
if ! grep -q "## \[Unreleased\]" CHANGELOG.md; then
    error "CHANGELOG.md doesn't have an [Unreleased] section"
fi

# Check if there are actually changes in Unreleased
unreleased_content=$(awk '/## \[Unreleased\]/,/## \[/' CHANGELOG.md | sed '1d;$d' | grep -v '^$' | grep -v '^\s*$' || true)
if [ -z "$unreleased_content" ]; then
    warning "No changes found in [Unreleased] section of CHANGELOG.md"
    read -p "Continue anyway? (y/N): " confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        error "Release cancelled"
    fi
fi

# Show what will be released
echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Changes to be released in v$new_version:${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
awk '/## \[Unreleased\]/,/## \[/' CHANGELOG.md | sed '1d;$d'
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo ""

# Confirm
read -p "Proceed with release v$new_version? (y/N): " confirm
if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
    error "Release cancelled"
fi

# Get today's date
release_date=$(date +%Y-%m-%d)

# Update CHANGELOG.md
info "Updating CHANGELOG.md..."

# Create temp file
temp_file=$(mktemp)

# Process CHANGELOG
awk -v version="$new_version" -v date="$release_date" '
/## \[Unreleased\]/ {
    print "## [Unreleased]"
    print ""
    print "## [" version "] - " date
    next
}
{ print }
' CHANGELOG.md > "$temp_file"

# Update version links at bottom
sed -i "s|\[Unreleased\]:.*|\[Unreleased\]: https://github.com/adanoelle/tapestry/compare/v${new_version}...HEAD\n[${new_version}]: https://github.com/adanoelle/tapestry/compare/v${current_version}...v${new_version}|" "$temp_file"

mv "$temp_file" CHANGELOG.md
success "CHANGELOG.md updated"

# Update Cargo.toml
info "Updating $CARGO_TOML..."
sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML"
success "$CARGO_TOML updated"

# Show changes
info "Changes made:"
git diff CHANGELOG.md "$CARGO_TOML"

# Create commit
info "Creating release commit..."
git add CHANGELOG.md "$CARGO_TOML"
git commit -m "chore: bump version to $new_version

Release v$new_version

See CHANGELOG.md for full details."

success "Release commit created"

# Ask about pushing
echo ""
read -p "Push to origin/main to trigger release? (y/N): " push_confirm
if [[ "$push_confirm" =~ ^[Yy]$ ]]; then
    info "Pushing to origin/main..."
    git push origin main
    success "Pushed to origin/main"

    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}Release process initiated!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Auto-tag workflow will create tag: v$new_version"
    echo "2. Release workflow will build binaries for all platforms"
    echo "3. GitHub Release will be published"
    echo ""
    echo "Monitor progress:"
    echo "  gh run watch"
    echo ""
    echo "View release when ready:"
    echo "  https://github.com/adanoelle/tapestry/releases/tag/v$new_version"
    echo ""
else
    warning "Commit created but not pushed"
    echo ""
    echo "To push later, run:"
    echo "  git push origin main"
    echo ""
    echo "To undo this release:"
    echo "  git reset --hard HEAD^"
fi
