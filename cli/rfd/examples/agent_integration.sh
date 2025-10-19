#!/usr/bin/env bash
#
# Agent Integration Example
#
# This script demonstrates:
# - Using RFD CLI in automated scripts
# - Parsing JSON output
# - Error handling
# - Integration with other tools (gh, git, etc.)
# - Building automated workflows
#
# For junior developers: Shows how to integrate RFD management
# into CI/CD pipelines and agent workflows.

set -euo pipefail

echo "=== Agent Integration Example ==="
echo

# Configuration
GITHUB_REPO="org/repo"  # Change to your repo
ERROR_LOG="rfd_errors.log"

# Helper function: Check if command succeeded
check_result() {
    local exit_code=$1
    local command=$2

    if [ $exit_code -ne 0 ]; then
        echo "❌ Error: $command failed with exit code $exit_code" >&2
        echo "$(date): $command failed" >> "$ERROR_LOG"
        return 1
    fi
    return 0
}

# Helper function: Parse JSON safely
parse_json() {
    local json=$1
    local query=$2

    echo "$json" | jq -r "$query" 2>/dev/null || {
        echo "Error: Failed to parse JSON" >&2
        return 1
    }
}

# Step 1: Create RFD with error handling
echo "1. Creating RFD with error handling..."

create_output=$(rfd create \
    --title "Automated RFD" \
    --author "CI Bot <ci@example.com>" \
    --format json 2>&1) || {
        echo "❌ Failed to create RFD"
        exit 1
    }

rfd_id=$(parse_json "$create_output" '.id')
rfd_path=$(parse_json "$create_output" '.path')

echo "✓ Created RFD $rfd_id at $rfd_path"
echo

# Step 2: Validate RFD before proceeding
echo "2. Validating RFD..."

validate_output=$(rfd validate "$rfd_id" --format json)
is_valid=$(parse_json "$validate_output" '.valid')

if [ "$is_valid" = "true" ]; then
    echo "✓ RFD is valid"
else
    echo "❌ RFD validation failed"
    issues=$(parse_json "$validate_output" '.issues | join(", ")')
    echo "Issues: $issues"
    exit 1
fi
echo

# Step 3: Check if RFD can transition to review
echo "3. Checking state transition eligibility..."

# Get current RFD details
show_output=$(rfd show "$rfd_id" --format json)
current_state=$(parse_json "$show_output" '.metadata.state')

echo "Current state: $current_state"

# Try to update status (with error handling)
if rfd status "$rfd_id" --set review --format json > /dev/null 2>&1; then
    echo "✓ Transitioned to review"
else
    exit_code=$?
    if [ $exit_code -eq 3 ]; then
        echo "⚠️  State transition not allowed (exit code 3)"
    else
        echo "❌ Unexpected error (exit code $exit_code)"
    fi
fi
echo

# Step 4: Generate report for multiple RFDs
echo "4. Generating RFD status report..."

list_output=$(rfd list --format json)
total=$(parse_json "$list_output" '.total')

echo "Total RFDs: $total"
echo

# Generate summary by state
for state in draft review accepted rejected implemented archived; do
    count=$(rfd list --status "$state" --format json 2>/dev/null | jq '.total' || echo "0")
    if [ "$count" -gt 0 ]; then
        printf "  %-12s: %2d RFD(s)\n" "$state" "$count"
    fi
done
echo

# Step 5: Find RFDs needing review (draft > 7 days old)
echo "5. Finding stale draft RFDs (> 7 days old)..."

seven_days_ago=$(date -d '7 days ago' +%Y-%m-%d 2>/dev/null || date -v-7d +%Y-%m-%d)

stale_rfds=$(rfd list --status draft --format json | jq -r --arg cutoff "$seven_days_ago" '
    .rfds[] |
    select(.created < $cutoff) |
    "\(.id): \(.title) (created \(.created))"
')

if [ -n "$stale_rfds" ]; then
    echo "⚠️  Found stale draft RFDs:"
    echo "$stale_rfds"
else
    echo "✓ No stale drafts"
fi
echo

# Step 6: Export RFD metadata for external tools
echo "6. Exporting RFD metadata for tracking..."

# Export to JSON file
rfd list --format json > rfds_export.json
echo "✓ Exported to rfds_export.json"

# Convert to CSV for spreadsheets
{
    echo "ID,Title,State,Authors,Created,Updated"
    jq -r '.rfds[] | [.id, .title, .state, (.authors | join(";")), .created, .updated] | @csv' rfds_export.json
} > rfds_export.csv

echo "✓ Exported to rfds_export.csv"
echo

# Step 7: Integration with GitHub CLI (if available)
echo "7. GitHub integration example..."

if command -v gh &> /dev/null; then
    echo "GitHub CLI detected - could create issues for RFDs"

    # Example: Create GitHub issue for new RFD
    # gh issue create \
    #     --title "[RFD $rfd_id] Automated RFD" \
    #     --body "New RFD created: $rfd_path" \
    #     --label "rfd,needs-review" \
    #     --repo "$GITHUB_REPO"

    echo "✓ GitHub CLI available (example code commented)"
else
    echo "ℹ️  GitHub CLI not installed (skipping)"
fi
echo

# Step 8: Integration with git (if in repo)
echo "8. Git integration example..."

if git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Git repository detected"

    # Check if RFD file exists in git
    if git ls-files --error-unmatch "$rfd_path" > /dev/null 2>&1; then
        echo "✓ RFD is tracked by git"
    else
        echo "ℹ️  RFD not yet tracked (would run: git add $rfd_path)"
        # git add "$rfd_path"
        # git commit -m "feat: add RFD $rfd_id - Automated RFD"
    fi
else
    echo "ℹ️  Not a git repository (skipping)"
fi
echo

# Step 9: Webhook notification example
echo "9. Webhook notification example..."

# Example: Send notification to Slack/Discord/etc
notify_webhook() {
    local webhook_url=${WEBHOOK_URL:-}
    local rfd_id=$1
    local action=$2

    if [ -n "$webhook_url" ]; then
        payload=$(jq -n \
            --arg id "$rfd_id" \
            --arg action "$action" \
            '{
                "text": "RFD Update",
                "blocks": [
                    {
                        "type": "section",
                        "text": {
                            "type": "mrkdwn",
                            "text": "*RFD \($id)*: \($action)"
                        }
                    }
                ]
            }')

        curl -X POST -H 'Content-Type: application/json' \
            -d "$payload" "$webhook_url" 2>/dev/null || true

        echo "✓ Notification sent"
    else
        echo "ℹ️  WEBHOOK_URL not set (would send: RFD $rfd_id - $action)"
    fi
}

notify_webhook "$rfd_id" "Created"
echo

# Step 10: Daily digest generation
echo "10. Generating daily digest..."

digest_date=$(date +%Y-%m-%d)

cat > rfd_digest_$digest_date.md <<EOF
# RFD Daily Digest - $digest_date

## Summary

- **Total RFDs**: $total
- **New Today**: $(rfd list --format json | jq --arg today "$digest_date" '[.rfds[] | select(.created | startswith($today))] | length')
- **Updated Today**: $(rfd list --format json | jq --arg today "$digest_date" '[.rfds[] | select(.updated | startswith($today))] | length')

## Status Breakdown

$(for state in draft review accepted rejected implemented archived; do
    count=$(rfd list --status "$state" --format json 2>/dev/null | jq '.total' || echo "0")
    echo "- **$state**: $count"
done)

## Recently Updated

$(rfd list --format json | jq -r '.rfds | sort_by(.updated) | reverse | .[0:5][] | "- \(.id): \(.title) (\(.state))"')

## Action Items

- [ ] Review drafts older than 7 days
- [ ] Update accepted RFDs to implemented
- [ ] Archive rejected RFDs

---
Generated by RFD CLI on $digest_date
EOF

echo "✓ Created digest: rfd_digest_$digest_date.md"
cat rfd_digest_$digest_date.md
echo

# Cleanup
rm -f rfds_export.json rfds_export.csv rfd_digest_*.md

echo "=== Agent Integration Complete! ==="
echo
echo "Key patterns demonstrated:"
echo
echo "1. Error Handling"
echo "   - Check exit codes (0=success, 1=error, 2=validation, 3=state)"
echo "   - Use --format json for parsing"
echo "   - Log errors to file"
echo
echo "2. JSON Parsing"
echo "   - Use jq for robust parsing"
echo "   - Always handle parse failures"
echo "   - Validate data before using"
echo
echo "3. State Management"
echo "   - Check transitions before attempting"
echo "   - Handle transition errors gracefully"
echo "   - Track state changes"
echo
echo "4. Integration Points"
echo "   - Git: Auto-commit RFD changes"
echo "   - GitHub: Create issues/PRs for RFDs"
echo "   - Webhooks: Notify on RFD updates"
echo "   - CI/CD: Validate RFDs in pipeline"
echo
echo "5. Automation Use Cases"
echo "   - Daily digest emails"
echo "   - Stale RFD alerts"
echo "   - Status reports"
echo "   - Workflow automation"
echo
echo "Example CI/CD Pipeline:"
echo "  1. Validate all RFDs: rfd validate --all"
echo "  2. Check for stale drafts: (script above)"
echo "  3. Generate report: rfd list --format json > report.json"
echo "  4. Post to dashboard: curl -X POST ... < report.json"
echo
echo "Exit codes:"
echo "  0 - Success"
echo "  1 - General error (file not found, etc.)"
echo "  2 - Validation error (invalid RFD)"
echo "  3 - State transition error (illegal transition)"
