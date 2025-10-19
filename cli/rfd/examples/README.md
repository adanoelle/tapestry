# RFD CLI Examples

This directory contains practical examples demonstrating how to use the RFD CLI
in various scenarios.

## Running Examples

All examples are executable bash scripts. Make sure you have the RFD CLI built:

```bash
# Build the CLI
cd ..
cargo build --release

# Add to PATH (or use cargo run --)
export PATH="$PWD/../../target/release:$PATH"

# Run an example
cd examples
./basic_workflow.sh
```

## Examples Overview

### 1. basic_workflow.sh

**Purpose**: Demonstrates fundamental RFD operations

**What it covers**:

- Creating a new RFD
- Listing RFDs (pretty and JSON formats)
- Showing RFD details
- Updating RFD status through workflow states
- Validating RFD structure

**Run time**: ~5 seconds

**Best for**: New users learning the CLI

```bash
./basic_workflow.sh
```

### 2. bulk_operations.sh

**Purpose**: Shows automation for managing multiple RFDs

**What it covers**:

- Creating RFDs programmatically in a loop
- Batch status updates
- Generating status reports
- Filtering by author
- Exporting to CSV format

**Run time**: ~10 seconds

**Best for**: Team leads, automation engineers

```bash
./bulk_operations.sh
```

### 3. filtering_and_search.sh

**Purpose**: Demonstrates filtering and querying capabilities

**What it covers**:

- Filtering by status
- Filtering by author
- Limiting results
- Advanced jq queries (grouping, formatting)
- Combining multiple filters
- Date-based filtering

**Run time**: ~8 seconds

**Best for**: Users needing to generate reports or find specific RFDs

**Prerequisites**: `jq` command-line tool

```bash
./filtering_and_search.sh
```

### 4. custom_templates.sh

**Purpose**: Shows how to create and use custom RFD templates

**What it covers**:

- Setting up template directory
- Creating minimal, detailed, and team-specific templates
- Template syntax (variables, filters, conditionals)
- Using custom templates
- Template best practices

**Run time**: ~5 seconds

**Best for**: Teams wanting to customize RFD format

```bash
./custom_templates.sh
```

### 5. agent_integration.sh

**Purpose**: Integration patterns for AI agents and automation

**What it covers**:

- Error handling and exit codes
- JSON parsing with jq
- State transition validation
- Integration with Git
- Integration with GitHub CLI
- Webhook notifications
- Generating daily digests
- CI/CD pipeline patterns

**Run time**: ~10 seconds

**Best for**: DevOps engineers, agent developers, CI/CD integration

**Prerequisites**: `jq`, optionally `gh` (GitHub CLI), git repository

```bash
./agent_integration.sh
```

## Prerequisites

### Required

- Bash shell
- RFD CLI (built from source or installed)

### Optional (for full functionality)

- **jq**: JSON processor

  ```bash
  # macOS
  brew install jq

  # Linux
  sudo apt install jq  # Debian/Ubuntu
  sudo dnf install jq  # Fedora
  ```

- **GitHub CLI** (for GitHub integration examples):

  ```bash
  # macOS
  brew install gh

  # Linux - see https://github.com/cli/cli#installation
  ```

## Running in a Clean Environment

Each example can be run in a temporary directory to avoid affecting your
existing RFDs:

```bash
# Create temp directory
tmp_dir=$(mktemp -d)
cd "$tmp_dir"

# Run example
/path/to/examples/basic_workflow.sh

# Cleanup
cd -
rm -rf "$tmp_dir"
```

## Output Formats

All examples demonstrate the three output formats:

- **Pretty** (default): Human-readable with colors
- **JSON** (`--format json`): Structured data for parsing
- **Quiet** (`--format quiet`): Errors only

## Common Patterns

### Error Handling

```bash
# Check exit code
rfd create --title "Test" --author "Me <me@test.com>" || {
    echo "Failed to create RFD"
    exit 1
}

# Capture output
output=$(rfd list --format json)
```

### JSON Parsing

```bash
# Extract field
total=$(rfd list --format json | jq '.total')

# Filter and transform
rfd list --format json | jq '.rfds[] | select(.state == "draft") | .title'

# Format as table
rfd list --format json | jq -r '.rfds[] | [.id, .title, .state] | @tsv' | column -t
```

### Combining Commands

```bash
# Create and immediately update
rfd_id=$(rfd create --title "Test" --author "Me" --format json | jq -r '.id')
rfd status "$rfd_id" --set review
```

## Integration Patterns

### CI/CD Pipeline

```bash
#!/bin/bash
# In your CI pipeline

# Validate all RFDs
rfd validate --all || exit 1

# Check for stale drafts
stale_count=$(rfd list --status draft --format json | \
    jq --arg cutoff "$(date -d '7 days ago' +%Y-%m-%d)" \
    '[.rfds[] | select(.created < $cutoff)] | length')

if [ "$stale_count" -gt 0 ]; then
    echo "⚠️  Found $stale_count stale draft RFDs"
fi
```

### Daily Report

```bash
#!/bin/bash
# Run daily via cron

{
    echo "# RFD Status Report - $(date +%Y-%m-%d)"
    echo
    rfd list --format json | jq -r '
        "## Summary\n\nTotal: \(.total) RFDs\n\n## By Status\n" +
        ([.rfds | group_by(.state)[] |
          "- \(.[0].state): \(length) RFDs"] | join("\n"))
    '
} | mail -s "Daily RFD Report" team@example.com
```

### GitHub Integration

```bash
#!/bin/bash
# Create GitHub issues for new RFDs

for rfd in $(rfd list --status draft --format json | jq -r '.rfds[].id'); do
    title=$(rfd show "$rfd" --format json | jq -r '.metadata.title')
    path=$(rfd show "$rfd" --format json | jq -r '.path')

    gh issue create \
        --title "[RFD $rfd] $title" \
        --body "New RFD: $path" \
        --label "rfd,needs-review"
done
```

## Troubleshooting

### Example fails with "command not found"

Make sure the RFD CLI is in your PATH:

```bash
# Build first
cargo build --release

# Add to PATH
export PATH="$PWD/../../target/release:$PATH"

# Or use cargo run
alias rfd='cargo run --release --'
```

### jq not found

Install jq (required for filtering_and_search.sh and agent_integration.sh):

```bash
brew install jq  # macOS
sudo apt install jq  # Linux
```

### Permission denied

Make scripts executable:

```bash
chmod +x *.sh
```

### RFD directory already exists

Examples create RFDs in the current directory. Run in a clean directory or use:

```bash
rm -rf rfds/  # Remove existing RFDs (careful!)
```

## Learning Path

Recommended order for learning:

1. **basic_workflow.sh** - Learn fundamental operations
2. **filtering_and_search.sh** - Learn querying and reporting
3. **bulk_operations.sh** - Learn automation
4. **custom_templates.sh** - Learn customization
5. **agent_integration.sh** - Learn integration patterns

## Additional Resources

- [README.md](../README.md) - User documentation
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Codebase architecture
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contributing guide
- [RFD CLI Documentation](https://github.com/yourusername/tapestry/tree/main/cli/rfd)

## Contributing Examples

Have a useful pattern? Add an example!

1. Create a new `.sh` file
2. Follow the existing format (header, steps, cleanup)
3. Add documentation to this README
4. Submit a PR

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

## License

MIT OR Apache-2.0
