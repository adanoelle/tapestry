#!/usr/bin/env bash
#
# Filtering and Search Example
#
# This script demonstrates:
# - Filtering RFDs by status
# - Filtering by author
# - JSON output with jq parsing
# - Limiting results
# - Combining filters
#
# For junior developers: Shows how to query and filter RFDs
# for reporting and analysis.

set -euo pipefail

echo "=== Filtering and Search Example ==="
echo

# Prerequisites: Create some test RFDs
echo "Setting up test data..."

rfd create --title "Auth System" --author "Alice <alice@example.com>" --format quiet
rfd create --title "Database Schema" --author "Bob <bob@example.com>" --format quiet
rfd create --title "API Design" --author "Alice <alice@example.com>" --format quiet
rfd create --title "Frontend Refactor" --author "Charlie <charlie@example.com>" --format quiet
rfd create --title "Testing Strategy" --author "Bob <bob@example.com>" --format quiet

echo "✓ Created 5 test RFDs"
echo

# Step 1: List all RFDs
echo "1. Listing all RFDs (pretty format)..."
rfd list

echo
echo "✓ All RFDs listed"
echo

# Step 2: Filter by status (draft)
echo "2. Filtering by status: draft..."
rfd list --status draft

echo
echo "✓ Showed only draft RFDs"
echo

# Step 3: Update some RFDs to different states
echo "3. Changing states for variety..."
rfd status 1 --set review --format quiet
rfd status 2 --set review --format quiet
rfd status 3 --set accepted --format quiet

echo "✓ Updated RFD states"
echo

# Step 4: Filter by status with JSON
echo "4. Filtering by status with JSON output..."
echo "   Finding all RFDs in review:"
rfd list --status review --format json | jq '.rfds[] | {id, title, state}'

echo
echo "✓ JSON output allows precise data extraction"
echo

# Step 5: Filter by author
echo "5. Filtering by author (Alice)..."
rfd list --author Alice

echo
echo "✓ Filtered by author name"
echo

# Step 6: Filter by author with JSON
echo "6. Getting email addresses for author's RFDs..."
rfd list --author Bob --format json | jq -r '.rfds[] | .authors[]' | sort -u

echo
echo "✓ Extracted unique author emails"
echo

# Step 7: Limit results
echo "7. Limiting results (top 3)..."
rfd list --limit 3

echo
echo "✓ Limited to 3 results"
echo

# Step 8: Advanced jq queries
echo "8. Advanced jq query: group RFDs by state..."
rfd list --format json | jq '
    .rfds |
    group_by(.state) |
    map({
        state: .[0].state,
        count: length,
        rfds: map(.id)
    })
'

echo
echo "✓ Grouped and counted by state"
echo

# Step 9: Extract specific fields
echo "9. Creating a summary table..."
rfd list --format json | jq -r '
    ["ID", "Title", "State", "Authors"],
    (.rfds[] | [.id, .title, .state, (.authors | join("; "))]) |
    @tsv
' | column -t -s $'\t'

echo
echo "✓ Created formatted table"
echo

# Step 10: Find RFDs created today
echo "10. Finding RFDs created today..."
TODAY=$(date +%Y-%m-%d)
rfd list --format json | jq --arg today "$TODAY" '
    .rfds[] |
    select(.created | startswith($today)) |
    {id, title}
'

echo
echo "✓ Filtered by creation date"
echo

# Step 11: Combining filters with shell pipes
echo "11. Finding Alice's draft RFDs..."
rfd list --author Alice --status draft

echo
echo "✓ Combined multiple filters"
echo

echo "=== Filtering Complete! ==="
echo
echo "Key techniques demonstrated:"
echo "  - Filter by status: --status <state>"
echo "  - Filter by author: --author <name>"
echo "  - Limit results: --limit <n>"
echo "  - JSON output: --format json"
echo "  - jq for advanced queries (grouping, filtering, formatting)"
echo "  - Combining filters for precise queries"
echo
echo "Advanced jq patterns:"
echo "  - Extract fields: jq '.rfds[] | {id, title}'"
echo "  - Filter by date: jq 'select(.created | startswith(...))'  "
echo "  - Group and count: jq 'group_by(.state) | map({...})'"
echo "  - Format as table: jq -r '... | @tsv' | column -t"
echo
echo "Integration examples:"
echo "  - Daily reports: rfd list --format json | jq '...'"
echo "  - Team dashboards: rfd list --author 'TeamName' --format json"
echo "  - CI/CD checks: rfd list --status draft --limit 1"
