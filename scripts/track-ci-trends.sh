#!/usr/bin/env bash
#
# CI Trend Tracking Script
# Captures current CI performance metrics and appends to trend log
#
# Usage:
#   ./scripts/track-ci-trends.sh [workflow-name]
#
# Examples:
#   ./scripts/track-ci-trends.sh              # Track all workflows
#   ./scripts/track-ci-trends.sh ci.yml       # Track specific workflow
#
# Output:
#   Appends metrics to .github/ci-metrics.csv
#   Creates file if it doesn't exist

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

WORKFLOW="${1:-}"
METRICS_FILE=".github/ci-metrics.csv"
SAMPLE_SIZE=10  # Last N runs to analyze

# Check dependencies
if ! command -v gh &> /dev/null; then
    echo -e "${RED}Error: gh CLI is not installed${NC}"
    exit 1
fi

if ! command -v jq &> /dev/null; then
    echo -e "${RED}Error: jq is not installed${NC}"
    exit 1
fi

# Create metrics directory if needed
mkdir -p .github

# Initialize CSV if it doesn't exist
if [[ ! -f "$METRICS_FILE" ]]; then
    echo "date,workflow,avg_duration_min,median_duration_min,p95_duration_min,p99_duration_min,success_rate,total_runs,fast_runs,cache_hit_estimate" > "$METRICS_FILE"
    echo -e "${BLUE}Created new metrics file: $METRICS_FILE${NC}"
fi

# Function to capture metrics for a workflow
capture_metrics() {
    local workflow_name="$1"
    local today=$(date +%Y-%m-%d)

    echo -e "${YELLOW}Capturing metrics for: ${workflow_name:-all workflows}${NC}"

    # Fetch recent runs
    local workflow_filter=""
    if [[ -n "$workflow_name" ]]; then
        workflow_filter="--workflow=$workflow_name"
    fi

    local runs_json=$(gh run list $workflow_filter --limit "$SAMPLE_SIZE" --json conclusion,startedAt,updatedAt,workflowName,status 2>/dev/null || echo "[]")

    if [[ "$runs_json" == "[]" || -z "$runs_json" ]]; then
        echo -e "${RED}No runs found for workflow${NC}"
        return 1
    fi

    # Process each unique workflow
    local workflows=$(echo "$runs_json" | jq -r 'map(.workflowName) | unique | .[]')

    while IFS= read -r wf; do
        echo -e "${BLUE}Processing: $wf${NC}"

        # Calculate statistics
        local stats=$(echo "$runs_json" | jq --arg workflow "$wf" '
            map(select(.workflowName == $workflow and .status == "completed")) |
            map({
                duration: (((.updatedAt | fromdateiso8601) - (.startedAt | fromdateiso8601)) / 60 | floor),
                conclusion: .conclusion
            }) |
            {
                total: length,
                successful: [.[] | select(.conclusion == "success")] | length,
                durations: [.[].duration],
                fast_runs: [.[] | select(.duration < 5)] | length
            } |
            if .durations | length > 0 then
                {
                    total: .total,
                    successful: .successful,
                    avg: (.durations | add / length | floor),
                    median: (.durations | sort | .[length/2 | floor]),
                    p95: (.durations | sort | .[length * 0.95 | floor]),
                    p99: (.durations | sort | .[length * 0.99 | floor]),
                    success_rate: ((.successful * 100 / .total) | floor),
                    fast_runs: .fast_runs,
                    cache_hit_estimate: ((.fast_runs * 100 / .total) | floor)
                }
            else
                empty
            end
        ')

        if [[ -z "$stats" || "$stats" == "null" ]]; then
            echo -e "${YELLOW}  No completed runs found${NC}"
            continue
        fi

        # Extract values
        local avg=$(echo "$stats" | jq -r '.avg')
        local median=$(echo "$stats" | jq -r '.median')
        local p95=$(echo "$stats" | jq -r '.p95')
        local p99=$(echo "$stats" | jq -r '.p99')
        local success_rate=$(echo "$stats" | jq -r '.success_rate')
        local total=$(echo "$stats" | jq -r '.total')
        local fast_runs=$(echo "$stats" | jq -r '.fast_runs')
        local cache_hit=$(echo "$stats" | jq -r '.cache_hit_estimate')

        # Append to CSV
        echo "$today,$wf,$avg,$median,$p95,$p99,$success_rate,$total,$fast_runs,$cache_hit" >> "$METRICS_FILE"

        echo -e "${GREEN}  ✓ Captured: avg=${avg}m, median=${median}m, p95=${p95}m, success=${success_rate}%${NC}"

        # Detect regressions
        detect_regression "$wf" "$median" "$p95"

    done <<< "$workflows"
}

# Function to detect regressions
detect_regression() {
    local workflow="$1"
    local current_median="$2"
    local current_p95="$3"

    # Get baseline (average of last 7 days excluding today)
    local baseline=$(tail -n 50 "$METRICS_FILE" | grep "$workflow" | head -n -1 | tail -n 7 | awk -F',' '{sum+=$4; count++} END {if(count>0) print int(sum/count); else print 0}')

    if [[ -z "$baseline" || "$baseline" == "0" ]]; then
        return 0  # Not enough data for comparison
    fi

    # Calculate regression threshold (20% slower)
    local threshold=$(echo "$baseline * 1.2" | bc | awk '{print int($1)}')

    if [[ $current_median -gt $threshold ]]; then
        echo -e "${RED}  ⚠ REGRESSION DETECTED!${NC}"
        echo -e "${RED}    Median duration: ${current_median}m (was ~${baseline}m)${NC}"
        echo -e "${RED}    Performance degraded by >20%${NC}"
    fi

    # Check P95 threshold (absolute limit)
    if [[ $current_p95 -gt 15 ]]; then
        echo -e "${YELLOW}  ⚠ P95 above threshold${NC}"
        echo -e "${YELLOW}    P95: ${current_p95}m (target: <15m)${NC}"
    fi
}

# Main execution
echo -e "${BLUE}=== CI Trend Tracker ===${NC}\n"

capture_metrics "$WORKFLOW"

echo ""
echo -e "${GREEN}✓ Metrics captured and saved to: $METRICS_FILE${NC}"
echo -e "${BLUE}View trends with: cat $METRICS_FILE | column -t -s,${NC}"
echo ""

# Show summary of last entry
echo -e "${BOLD}Latest Metrics:${NC}"
tail -n 5 "$METRICS_FILE" | column -t -s,
