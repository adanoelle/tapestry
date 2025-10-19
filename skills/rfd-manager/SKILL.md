---
name: rfd-manager
description: Manage RFD (Request for Discussion) documents with structured operations
version: 1.0.0
tools: Bash, Read, Write, Grep, Glob
---

# RFD Management Skill

This skill helps manage RFD (Request for Discussion) documents using the `rfd` CLI tool. It provides structured workflows for creating, updating, and managing technical documentation following the Oxide Computer RFD format.

## Prerequisites

The `rfd` CLI tool must be available in your PATH. Check with:

```bash
cargo run --bin rfd -- --help
```

## Core Operations

### Creating a New RFD

When the user wants to create a new RFD:

```bash
# Full command with all options
rfd create --title "Feature Proposal" --author "Name <email@example.com>" --format json

# Minimal command (uses defaults)
rfd create --title "Feature Proposal" --author "Name <email@example.com>"
```

**Workflow**:
1. Ask user for title if not provided
2. Ask user for author (or use git config)
3. Create the RFD using the CLI
4. Parse JSON response to get RFD number
5. Open the created file for user to edit

### Listing RFDs

To see all RFDs or filter them:

```bash
# List all RFDs
rfd list --format json

# Filter by status
rfd list --status draft --format json
rfd list --status review --format json
rfd list --status accepted --format json

# Filter by author
rfd list --author "Name" --format json

# Limit results
rfd list --limit 10 --format json
```

**JSON Output**:
```json
{
  "rfds": [
    {
      "id": "001",
      "title": "Feature Proposal",
      "state": "draft",
      "authors": ["Alice <alice@example.com>"],
      "created": "2025-10-17",
      "updated": "2025-10-17",
      "path": "rfds/0001-feature-proposal.md"
    }
  ],
  "total": 1
}
```

### Showing RFD Details

To view a specific RFD:

```bash
# Show with JSON output
rfd show 003 --format json

# Show with pretty output (human-readable)
rfd show 003
```

### Updating RFD Status

RFD lifecycle follows this state machine:
```
draft → review → accepted → implemented
  │       │         │
  └───────┴─────────┴──> rejected → archived
```

```bash
# Move to review
rfd status 003 --set review --format json

# Accept an RFD
rfd status 003 --set accepted --format json

# Mark as implemented
rfd status 003 --set implemented --format json

# Reject an RFD
rfd status 003 --set rejected --format json

# Archive an RFD (terminal state)
rfd status 003 --set archived --format json
```

**Idempotency**: These commands succeed whether the state changes or is already correct.

**Error Handling**: If transition is invalid, the tool returns JSON with:
```json
{
  "error": "INVALID_TRANSITION",
  "message": "Cannot transition from 'draft' to 'archived'",
  "details": {
    "current_state": "draft",
    "valid_next_states": ["review", "accepted", "rejected"],
    "suggestion": {
      "command": "rfd status 003 --set rejected",
      "description": "Move to rejected state instead"
    }
  }
}
```

### Updating RFD Metadata

To update specific fields:

```bash
# Update title
rfd update 003 --field title --value "New Title" --format json

# Add discussion link
rfd update 003 --field discussion --value "https://github.com/org/repo/issues/123" --format json

# Add tags
rfd update 003 --field tags --value "architecture,api" --format json
```

### Validating an RFD

To check if an RFD follows the correct structure:

```bash
rfd validate 003 --format json
```

**JSON Output**:
```json
{
  "valid": true,
  "issues": []
}

# Or if invalid:
{
  "valid": false,
  "issues": [
    "Missing required field: summary",
    "State 'in-progress' is not valid"
  ]
}
```

## Common Workflows

### Workflow 1: Create and Populate RFD

1. Create RFD: `rfd create --title "..." --author "..." --format json`
2. Parse response to get RFD number (e.g., "003")
3. Read the created file: `rfd show 003`
4. Help user fill in sections (Summary, Motivation, Proposal, etc.)
5. Validate: `rfd validate 003 --format json`

### Workflow 2: Move RFD Through Review Process

1. List draft RFDs: `rfd list --status draft --format json`
2. User selects RFD for review
3. Update status: `rfd status 003 --set review --format json`
4. After review, accept: `rfd status 003 --set accepted --format json`
5. After implementation: `rfd status 003 --set implemented --format json`

### Workflow 3: Find RFDs by Criteria

1. List all RFDs: `rfd list --format json`
2. Filter by author: `rfd list --author "Alice" --format json`
3. Filter by status: `rfd list --status accepted --format json`
4. Show specific RFD: `rfd show 003 --format json`

### Workflow 4: Convert Existing RFC to RFD

1. Read existing RFC file (e.g., `docs/design/features/RFC-001-git-workflow.md`)
2. Create new RFD with appropriate title: `rfd create --title "..." --author "..."`
3. Copy content from RFC to RFD (adapt to RFD structure)
4. Validate: `rfd validate <number> --format json`
5. Update status to match RFC state

## Best Practices

1. **Always use `--format json`** when you need to parse the output
2. **Check exit codes**: 0 = success, 1 = general error, 2 = validation error, 3 = state error
3. **Handle errors gracefully**: Parse error JSON and show suggestions to user
4. **Validate before committing**: Run `rfd validate` before git operations
5. **Use idempotent operations**: Safe to retry commands
6. **Check state transitions**: Use error suggestions to fix invalid transitions

## Error Handling

All errors return structured JSON when using `--format json`:

```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable message",
  "details": {
    /* Context-specific details */
  },
  "suggestion": {
    "command": "Command to try instead",
    "description": "Why this might work"
  }
}
```

**Common Error Codes**:
- `INVALID_INPUT`: Missing or malformed input
- `INVALID_TRANSITION`: State transition not allowed
- `NOT_FOUND`: RFD doesn't exist
- `VALIDATION_ERROR`: RFD structure invalid
- `FILE_ERROR`: Filesystem operation failed

## Integration with Other Tools

### With git-workflow Tool (Future)

Once git-workflow MCP tool is operational:
1. Create RFD
2. Use git-workflow to create feature branch
3. Commit RFD
4. Create PR with RFD as description source

### With gh CLI

```bash
# Create RFD from GitHub issue (future feature)
gh issue view 123 | rfd create --from-issue --format json

# Link RFD to issue
rfd update 003 --field discussion --value "$(gh issue view 123 --json url -q .url)"
```

## Notes

- RFDs are stored in `rfds/` directory by default (configurable via `.rfd/config.toml`)
- RFD numbers are zero-padded 4 digits (e.g., 0001, 0002, 0003)
- RFDs use YAML front matter + Markdown body
- Template system allows customization (`.rfd/templates/`)

## Future Enhancements

- Search functionality across all RFDs
- Automatic cross-referencing between RFDs
- Export to AsciiDoc (for Oxide compatibility)
- Template selection (RFD, ADR, spec templates)
- Bulk operations (e.g., archive all rejected RFDs)

## Troubleshooting

**RFD CLI not found**:
```bash
# Check if binary exists
cargo run --bin rfd -- --help

# If not, the tool may not be built yet
cd cli/rfd && cargo build
```

**Invalid state transition**:
- Check current state: `rfd show <number> --format json`
- Review state machine diagram above
- Use suggested command from error response

**Validation errors**:
- Run `rfd validate <number> --format json` to see all issues
- Fix each issue in the RFD file
- Re-validate until `valid: true`

---

**Skill Status**: 🚧 In Development (RFD CLI MVP in progress)
**Last Updated**: 2025-10-17
**Maintainer**: Tapestry Team
