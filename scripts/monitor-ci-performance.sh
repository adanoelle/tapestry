#!/usr/bin/env bash
#
# CI Performance Monitoring Script
# Analyzes GitHub Actions workflow performance over time
#
# Usage:
#   ./scripts/monitor-ci-performance.sh [options]
#
# Options:
#   --limit N        Analyze last N workflow runs (default: 30)
#   --workflow NAME  Specific workflow to analyze (default: all)
#   --export FILE    Export raw data to CSV file
#   --compare W1 W2  Compare two workflows
#   --help           Show this help message

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m'

# Default options
LIMIT=30
WORKFLOW=""
EXPORT_FILE=""
COMPARE_MODE=false
WORKFLOW1=""
WORKFLOW2=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --limit)
            LIMIT="$2"
            shift 2
            ;;
        --workflow)
            WORKFLOW="$2"
            shift 2
            ;;
        --export)
            EXPORT_FILE="$2"
            shift 2
            ;;
        --compare)
            COMPARE_MODE=true
            WORKFLOW1="$2"
            WORKFLOW2="$3"
            shift 3
            ;;
        --help)
            grep "^#" "$0" | grep -v "#!/bin/bash" | sed 's/^# //'
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if gh is installed
if ! command -v gh &> /dev/null; then
    echo -e "${RED}Error: gh CLI is not installed${NC}"
    echo "Install it from: https://cli.github.com/"
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo -e "${RED}Error: jq is not installed${NC}"
    echo "Install it with: brew install jq (macOS) or apt install jq (Linux)"
    exit 1
fi

echo -e "${BLUE}${BOLD}=== CI Performance Monitor ===${NC}\n"

# Function to analyze a workflow
analyze_workflow() {
    local workflow_name="$1"
    local limit="$2"

    echo -e "${YELLOW}Analyzing workflow: ${BOLD}$workflow_name${NC}"
    echo -e "${YELLOW}Fetching last $limit runs...${NC}\n"

    # Fetch workflow runs
    local workflow_filter=""
    if [[ -n "$workflow_name" ]]; then
        workflow_filter="--workflow=$workflow_name"
    fi

    local runs_json=$(gh run list $workflow_filter --limit "$limit" --json conclusion,startedAt,updatedAt,workflowName,displayTitle,status | jq -c)

    if [[ -z "$runs_json" || "$runs_json" == "[]" ]]; then
        echo -e "${RED}No workflow runs found${NC}"
        return 1
    fi

    # Calculate durations
    local durations=$(echo "$runs_json" | jq -r '
        map(select(.status == "completed")) |
        map({
            workflow: .workflowName,
            conclusion: .conclusion,
            duration: (((.updatedAt | fromdateiso8601) - (.startedAt | fromdateiso8601)) / 60 | floor),
            title: .displayTitle
        })
    ')

    # Statistics
    local total=$(echo "$durations" | jq 'length')
    local successful=$(echo "$durations" | jq '[.[] | select(.conclusion == "success")] | length')
    local failed=$(echo "$durations" | jq '[.[] | select(.conclusion == "failure")] | length')
    local success_rate=$(awk "BEGIN {printf \"%.1f\", ($successful * 100 / $total)}")

    # Duration statistics (in minutes)
    local avg=$(echo "$durations" | jq '[.[].duration] | add / length | floor')
    local median=$(echo "$durations" | jq '[.[].duration] | sort | .[length/2 | floor]')
    local min=$(echo "$durations" | jq '[.[].duration] | min')
    local max=$(echo "$durations" | jq '[.[].duration] | max')
    local p95=$(echo "$durations" | jq '[.[].duration] | sort | .[length * 0.95 | floor]')
    local p99=$(echo "$durations" | jq '[.[].duration] | sort | .[length * 0.99 | floor]')

    # Estimate cache hit rate (for Nix workflows)
    local fast_runs=$(echo "$durations" | jq '[.[] | select(.duration < 5)] | length')
    local cache_hit_rate=$(awk "BEGIN {printf \"%.1f\", ($fast_runs * 100 / $total)}")

    # Display results
    echo -e "${BOLD}Summary Statistics (Last $total runs)${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${GREEN}Success Rate:${NC} $success_rate% ($successful/$total runs)"
    echo -e "${RED}Failed:${NC} $failed runs"
    echo ""

    echo -e "${BOLD}Duration Statistics${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${GREEN}Average:${NC}   ${avg}m"
    echo -e "${GREEN}Median:${NC}    ${median}m"
    echo -e "${GREEN}Min:${NC}       ${min}m"
    echo -e "${GREEN}Max:${NC}       ${max}m"
    echo -e "${YELLOW}P95:${NC}       ${p95}m"
    echo -e "${YELLOW}P99:${NC}       ${p99}m"
    echo ""

    # Cache hit estimate (if applicable)
    if [[ "$workflow_name" == *"nix"* || "$workflow_name" == *"Nix"* ]]; then
        echo -e "${BOLD}Cache Performance${NC}"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo -e "${GREEN}Estimated Cache Hit Rate:${NC} $cache_hit_rate%"
        echo -e "${BLUE}(Runs < 5 minutes considered cached)${NC}"
        echo ""
    fi

    # Recent failures
    local recent_failures=$(echo "$durations" | jq -r '[.[] | select(.conclusion == "failure")] | .[0:3] | .[] | "  - \(.title) (\(.duration)m)"')
    if [[ -n "$recent_failures" ]]; then
        echo -e "${BOLD}Recent Failures${NC}"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "$recent_failures"
        echo ""
    fi

    # Performance assessment
    echo -e "${BOLD}Performance Assessment${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    if [[ $median -lt 5 ]]; then
        echo -e "${GREEN}✓ Excellent${NC} - Median duration < 5 minutes"
    elif [[ $median -lt 10 ]]; then
        echo -e "${YELLOW}✓ Good${NC} - Median duration < 10 minutes"
    else
        echo -e "${RED}⚠ Needs attention${NC} - Median duration > 10 minutes"
    fi

    if [[ $p95 -lt 10 ]]; then
        echo -e "${GREEN}✓ P95 within target${NC} - < 10 minutes"
    elif [[ $p95 -lt 15 ]]; then
        echo -e "${YELLOW}⚠ P95 acceptable${NC} - < 15 minutes"
    else
        echo -e "${RED}⚠ P95 high${NC} - > 15 minutes (investigate)"
    fi

    if [[ $success_rate =~ ^([0-9]+) ]] && [[ ${BASH_REMATCH[1]} -gt 95 ]]; then
        echo -e "${GREEN}✓ Success rate excellent${NC} - > 95%"
    elif [[ ${BASH_REMATCH[1]} -gt 90 ]]; then
        echo -e "${YELLOW}⚠ Success rate good${NC} - > 90%"
    else
        echo -e "${RED}⚠ Success rate needs attention${NC} - < 90%"
    fi

    echo ""

    # Export if requested
    if [[ -n "$EXPORT_FILE" ]]; then
        echo -e "${BLUE}Exporting data to: $EXPORT_FILE${NC}"
        echo "date,workflow,conclusion,duration_minutes,title" > "$EXPORT_FILE"
        echo "$durations" | jq -r '.[] | [now|todate, .workflow, .conclusion, .duration, .title] | @csv' >> "$EXPORT_FILE"
        echo -e "${GREEN}✓ Export complete${NC}\n"
    fi

    # Store values for comparison
    WORKFLOW_STATS["$workflow_name"]="$avg:$median:$p95:$success_rate"
}

# Function to compare two workflows
compare_workflows() {
    local wf1="$1"
    local wf2="$2"

    declare -gA WORKFLOW_STATS

    echo -e "${BOLD}${BLUE}Comparing Workflows${NC}\n"

    analyze_workflow "$wf1" "$LIMIT"
    echo ""
    analyze_workflow "$wf2" "$LIMIT"
    echo ""

    # Extract stats
    IFS=':' read -r avg1 median1 p951 success1 <<< "${WORKFLOW_STATS[$wf1]}"
    IFS=':' read -r avg2 median2 p952 success2 <<< "${WORKFLOW_STATS[$wf2]}"

    echo -e "${BOLD}${BLUE}Comparison Summary${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    printf "%-20s %-15s %-15s %-15s\n" "Metric" "$wf1" "$wf2" "Difference"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    local avg_diff=$((avg2 - avg1))
    local median_diff=$((median2 - median1))
    local p95_diff=$((p952 - p951))

    printf "%-20s %-15s %-15s " "Average" "${avg1}m" "${avg2}m"
    if [[ $avg_diff -gt 0 ]]; then
        echo -e "${RED}+${avg_diff}m${NC}"
    elif [[ $avg_diff -lt 0 ]]; then
        echo -e "${GREEN}${avg_diff}m${NC}"
    else
        echo "same"
    fi

    printf "%-20s %-15s %-15s " "Median" "${median1}m" "${median2}m"
    if [[ $median_diff -gt 0 ]]; then
        echo -e "${RED}+${median_diff}m${NC}"
    elif [[ $median_diff -lt 0 ]]; then
        echo -e "${GREEN}${median_diff}m${NC}"
    else
        echo "same"
    fi

    printf "%-20s %-15s %-15s " "P95" "${p951}m" "${p952}m"
    if [[ $p95_diff -gt 0 ]]; then
        echo -e "${RED}+${p95_diff}m${NC}"
    elif [[ $p95_diff -lt 0 ]]; then
        echo -e "${GREEN}${p95_diff}m${NC}"
    else
        echo "same"
    fi

    printf "%-20s %-15s %-15s\n" "Success Rate" "${success1}%" "${success2}%"
    echo ""
}

# Main execution
if [[ "$COMPARE_MODE" == true ]]; then
    compare_workflows "$WORKFLOW1" "$WORKFLOW2"
else
    analyze_workflow "$WORKFLOW" "$LIMIT"
fi

echo -e "${BLUE}${BOLD}Monitor complete!${NC}"
