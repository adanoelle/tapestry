#!/usr/bin/env bash
#
# Bulk Operations Example
#
# This script demonstrates:
# - Creating multiple RFDs in a loop
# - Batch status updates
# - Filtering and reporting
# - Generating summaries
#
# For junior developers: Shows how to automate RFD management
# for larger teams or projects.

set -euo pipefail

echo "=== Bulk Operations Example ==="
echo

# Array of RFD ideas to create
declare -a rfds=(
    "API Design:api-redesign:Bob <bob@example.com>"
    "Database Migration:db-migration:Charlie <charlie@example.com>"
    "Frontend Refactor:frontend-refactor:Diana <diana@example.com>"
    "Testing Strategy:testing-strategy:Eve <eve@example.com>"
    "Documentation Update:docs-update:Frank <frank@example.com>"
)

# Step 1: Bulk create RFDs
echo "1. Creating multiple RFDs..."
for rfd_spec in "${rfds[@]}"; do
    # Split spec into title, slug, and author
    IFS=':' read -r title slug author <<< "$rfd_spec"

    echo "   Creating: $title"
    rfd create --title "$title" --author "$author" --format quiet

done

echo "✓ Created ${#rfds[@]} RFDs"
echo

# Step 2: List all RFDs with JSON and count them
echo "2. Counting created RFDs..."
count=$(rfd list --format json | jq '.total')
echo "✓ Total RFDs: $count"
echo

# Step 3: Batch update - move all drafts to review
echo "3. Batch updating: moving all drafts to review..."

# Get all draft RFD IDs
draft_ids=$(rfd list --status draft --format json | jq -r '.rfds[].id' | sed 's/^0*//')

for id in $draft_ids; do
    echo "   Moving RFD $id to review..."
    rfd status "$id" --set review --format quiet
done

echo "✓ All drafts moved to review"
echo

# Step 4: Generate status report
echo "4. Generating status report..."
echo

for state in draft review accepted rejected implemented archived; do
    count=$(rfd list --status "$state" --format json 2>/dev/null | jq '.total' || echo "0")
    printf "   %-15s: %d RFDs\n" "$state" "$count"
done

echo
echo "✓ Status report generated"
echo

# Step 5: Simulate approval process - accept some RFDs
echo "5. Simulating approval: accepting first 3 RFDs..."

review_ids=$(rfd list --status review --format json | jq -r '.rfds[0:3][].id' | sed 's/^0*//')

for id in $review_ids; do
    echo "   Accepting RFD $id..."
    rfd status "$id" --set accepted --format quiet
done

echo "✓ Approved 3 RFDs"
echo

# Step 6: Find RFDs by author
echo "6. Finding RFDs by author (Bob)..."
rfd list --author Bob

echo
echo "✓ Filtered by author"
echo

# Step 7: Export RFD list to CSV-like format
echo "7. Exporting RFD list to CSV format..."

{
    echo "ID,Title,State,Authors,Created,Updated"
    rfd list --format json | jq -r '
        .rfds[] |
        [.id, .title, .state, (.authors | join(";")), .created, .updated] |
        @csv
    '
} > rfds_export.csv

echo "✓ Exported to rfds_export.csv"
cat rfds_export.csv
echo

# Cleanup
rm -f rfds_export.csv

echo "=== Bulk Operations Complete! ==="
echo
echo "Summary:"
echo "  - Created ${#rfds[@]} RFDs programmatically"
echo "  - Batch updated all drafts to review"
echo "  - Generated status report"
echo "  - Approved multiple RFDs"
echo "  - Filtered by author"
echo "  - Exported to CSV"
echo
echo "Use cases:"
echo "  - Automating RFD creation from planning sessions"
echo "  - Generating weekly/monthly reports"
echo "  - Batch processing state transitions"
echo "  - Integrating with CI/CD pipelines"
