#!/usr/bin/env bash
#
# Basic RFD Workflow Example
#
# This script demonstrates the fundamental operations:
# - Creating a new RFD
# - Listing RFDs
# - Showing RFD details
# - Updating RFD status
# - Validating RFDs
#
# For junior developers: This shows the typical day-to-day workflow
# of managing RFDs. Each command builds on the previous one.

set -euo pipefail  # Exit on error, undefined vars, pipe failures

echo "=== Basic RFD Workflow Example ==="
echo

# Step 1: Create a new RFD
echo "1. Creating a new RFD..."
rfd create \
    --title "Feature: Add User Authentication" \
    --author "Alice Developer <alice@example.com>"

echo "✓ RFD created!"
echo

# Step 2: List all RFDs
echo "2. Listing all RFDs..."
rfd list

echo
echo "✓ Listed RFDs in pretty format"
echo

# Step 3: List with JSON output (for scripts/agents)
echo "3. Listing RFDs in JSON format..."
rfd list --format json | jq .

echo
echo "✓ JSON output can be parsed by tools like jq"
echo

# Step 4: Show specific RFD details
echo "4. Showing details for RFD 1..."
rfd show 1

echo
echo "✓ Showed full RFD details"
echo

# Step 5: Update RFD status from draft to review
echo "5. Moving RFD to review status..."
rfd status 1 --set review

echo
echo "✓ Status updated to review"
echo

# Step 6: Verify the status change
echo "6. Verifying status change..."
rfd show 1 | grep -i "state"

echo
echo "✓ Status change confirmed"
echo

# Step 7: Validate the RFD
echo "7. Validating RFD structure..."
rfd validate 1

echo
echo "✓ RFD is valid"
echo

# Step 8: Continue the workflow - move to accepted
echo "8. Moving RFD to accepted status..."
rfd status 1 --set accepted

echo
echo "✓ RFD now accepted"
echo

# Step 9: Final listing to show progress
echo "9. Final listing showing RFD progression..."
rfd list

echo
echo "=== Workflow Complete! ==="
echo
echo "Summary of what we did:"
echo "  1. Created an RFD (starts in 'draft' state)"
echo "  2. Listed RFDs (both pretty and JSON formats)"
echo "  3. Showed full RFD details"
echo "  4. Moved RFD through workflow: draft → review → accepted"
echo "  5. Validated RFD structure"
echo
echo "Next steps:"
echo "  - Try: rfd status 1 --set implemented"
echo "  - Try: rfd update 1 --field tags --value \"authentication,security\""
echo "  - Read: ARCHITECTURE.md for how this works under the hood"
