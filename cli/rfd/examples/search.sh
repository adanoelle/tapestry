#!/usr/bin/env bash
#
# RFD Search Command Examples
#
# This script demonstrates the search functionality:
# - Basic text search
# - Field-specific search (title, content, tags, metadata)
# - Multiple search terms with AND logic
# - Case-sensitive vs case-insensitive matching
# - Combining search with filters (status, author, limit)
# - JSON output for AI agents
#
# For junior developers: The search command helps you quickly find RFDs
# without manually browsing files. It's designed to work with both humans
# and AI agents (via JSON output).

set -euo pipefail  # Exit on error, undefined vars, pipe failures

echo "=== RFD Search Command Examples ==="
echo
echo "First, let's create some test RFDs to search through..."
echo

# Create test RFDs with varied content
rfd create \
    --title "Authentication System Design" \
    --author "Alice <alice@example.com>" \
    >/dev/null 2>&1

rfd create \
    --title "OAuth API Integration" \
    --author "Bob <bob@example.com>" \
    >/dev/null 2>&1

rfd create \
    --title "Database Security Model" \
    --author "Alice <alice@example.com>" \
    >/dev/null 2>&1

rfd create \
    --title "API Rate Limiting" \
    --author "Charlie <charlie@example.com>" \
    >/dev/null 2>&1

# Move some to different states
rfd status 2 --set review >/dev/null 2>&1
rfd status 3 --set accepted >/dev/null 2>&1

echo "✓ Created 4 test RFDs in various states"
echo

# Example 1: Basic search
echo "1. Basic search - Find all RFDs mentioning 'authentication'..."
echo "   Command: rfd search \"authentication\""
echo
rfd search "authentication"
echo
echo "   ✓ Searches title and content by default"
echo

# Example 2: Multiple terms (AND logic)
echo "2. Multiple search terms - Find RFDs with both 'oauth' AND 'api'..."
echo "   Command: rfd search \"oauth api\""
echo
rfd search "oauth api"
echo
echo "   ✓ All terms must match (AND logic)"
echo

# Example 3: Case-insensitive (default)
echo "3. Case-insensitive search (default)..."
echo "   Command: rfd search \"oauth\"  # Matches 'OAuth'"
echo
rfd search "oauth"
echo
echo "   ✓ Case-insensitive by default"
echo

# Example 4: Case-sensitive search
echo "4. Case-sensitive search..."
echo "   Command: rfd search \"OAuth\" --case-sensitive"
echo
rfd search "OAuth" --case-sensitive
echo
echo "   ✓ Exact case matching"
echo

# Example 5: Search specific field - title only
echo "5. Field-specific search - Title only..."
echo "   Command: rfd search \"api\" --in title"
echo
rfd search "api" --in title
echo
echo "   ✓ Searches only in title field"
echo

# Example 6: Search with status filter
echo "6. Search combined with status filter..."
echo "   Command: rfd search \"api\" --status review"
echo
rfd search "api" --status review
echo
echo "   ✓ Combines search with status filtering"
echo

# Example 7: Search with author filter
echo "7. Search combined with author filter..."
echo "   Command: rfd search \"security\" --author alice"
echo
rfd search "security" --author alice
echo
echo "   ✓ Finds RFDs by Alice containing 'security'"
echo

# Example 8: Search with limit
echo "8. Search with result limit..."
echo "   Command: rfd search \"a\" --limit 2"
echo
rfd search "a" --limit 2
echo
echo "   ✓ Returns maximum of 2 results"
echo

# Example 9: JSON output for agents
echo "9. JSON output for AI agents..."
echo "   Command: rfd search \"authentication\" --format json | jq ."
echo
rfd search "authentication" --format json | jq .
echo
echo "   ✓ Structured JSON output for programmatic use"
echo

# Example 10: Complex query combining everything
echo "10. Complex combined search..."
echo "    Command: rfd search \"api\" --in title --status accepted --limit 5"
echo
rfd search "api" --in title --status accepted --limit 5
echo
echo "    ✓ Combines search scope, filters, and limit"
echo

# Example 11: Search scopes demonstration
echo "11. Search scope comparison..."
echo
echo "    a) Search in title only:"
rfd search "security" --in title
echo
echo "    b) Search in content only:"
rfd search "security" --in content
echo
echo "    c) Search in tags only:"
rfd search "security" --in tags
echo
echo "    d) Search in metadata (title + tags + authors):"
rfd search "alice" --in metadata
echo
echo "    e) Search in all (title + content) - DEFAULT:"
rfd search "security"
echo
echo "    ✓ Different scopes search different fields"
echo

echo "=== Search Examples Complete! ==="
echo
echo "Summary of search capabilities:"
echo "  ✓ Basic text search across RFDs"
echo "  ✓ Multiple terms with AND logic"
echo "  ✓ Case-sensitive and case-insensitive modes"
echo "  ✓ Field-specific search (--in title|content|tags|metadata|all)"
echo "  ✓ Filter integration (--status, --author, --limit)"
echo "  ✓ JSON output for agents (--format json)"
echo
echo "Common search patterns:"
echo "  rfd search \"keyword\"                    # Basic search"
echo "  rfd search \"term1 term2\"                # Multiple terms (AND)"
echo "  rfd search \"api\" --in title            # Search only titles"
echo "  rfd search \"security\" --status draft   # Search draft RFDs"
echo "  rfd search \"oauth\" --author alice      # Search Alice's RFDs"
echo "  rfd search \"api\" --format json         # Get JSON output"
echo
echo "Search scopes:"
echo "  --in title      Search title field only"
echo "  --in content    Search markdown body only"
echo "  --in tags       Search tags only"
echo "  --in metadata   Search title + tags + authors"
echo "  --in all        Search title + content (DEFAULT)"
echo
echo "Tips:"
echo "  - Search is case-insensitive by default"
echo "  - Multiple terms must ALL match (AND logic)"
echo "  - Combine with filters for precise results"
echo "  - Use JSON output for scripting and AI agents"
echo
echo "Next steps:"
echo "  - Try searching your own RFDs"
echo "  - Experiment with different search scopes"
echo "  - Combine search with status/author filters"
echo "  - Pipe JSON output to jq for advanced filtering"
echo
