# Command: Create RFD

**Name**: create-rfd
**Description**: Create a new RFD (Request for Discussion) document using the rfd CLI tool
**Parameters**: `$TITLE`, `$AUTHOR` (optional)
**Example**: `/create-rfd "Git Workflow Automation" "Ada <ada@example.com>"`

---

## Overview

This command creates a new RFD document using our own `rfd` CLI tool. This is **dogfooding** - we use our own tools to validate they work well for both humans and agents.

**RFD vs RFC**:
- **RFD** (Request for Discussion): Structured, machine-readable, managed via CLI tool
- **RFC** (Request for Comments): Legacy format, being phased out

**Why RFDs?**:
- Structured YAML frontmatter (agent-parseable)
- Consistent format across all docs
- CLI tool for management (create, list, status)
- State machine for lifecycle tracking
- Inspired by Oxide Computer's proven process

---

## Prerequisites

### Check RFD CLI is Available

```bash
# Check if RFD CLI exists
if ! cargo run --bin rfd -- --version &> /dev/null; then
    echo "❌ RFD CLI not available"
    echo "Build it first: cargo build --bin rfd"
    exit 1
fi

echo "✅ RFD CLI is available"
```

---

## Phase 1: Gather Information

### Step 1.1: Get Title

If not provided, ask the user:

```markdown
What is the title of this RFD?

Example: "Git Workflow Automation Tool"
```

### Step 1.2: Get Author

If not provided, try to get from git config:

```bash
# Try to get author from git config
AUTHOR=$(git config user.name)
EMAIL=$(git config user.email)

if [ -n "$AUTHOR" ] && [ -n "$EMAIL" ]; then
    AUTHOR_STRING="$AUTHOR <$EMAIL>"
    echo "Using git author: $AUTHOR_STRING"
else
    echo "Please provide author in format: Name <email@example.com>"
fi
```

### Step 1.3: Determine Template (Optional)

```bash
# Default template
TEMPLATE="default"

# Or ask user for specific template
# TEMPLATE="adr"  # Architecture Decision Record
# TEMPLATE="spec" # Technical Specification
```

---

## Phase 2: Create RFD

### Step 2.1: Run RFD CLI Create Command

```bash
# Create RFD using CLI tool
OUTPUT=$(cargo run --bin rfd -- create \
    --title "$TITLE" \
    --author "$AUTHOR_STRING" \
    --template "$TEMPLATE" \
    --format json)

# Check if successful
if [ $? -eq 0 ]; then
    echo "✅ RFD created successfully"
else
    echo "❌ Failed to create RFD"
    echo "$OUTPUT"
    exit 1
fi
```

### Step 2.2: Parse Response

```bash
# Extract RFD number from JSON response
RFD_NUMBER=$(echo "$OUTPUT" | jq -r '.data.rfd_number')
RFD_PATH=$(echo "$OUTPUT" | jq -r '.data.path')

echo "RFD Number: $RFD_NUMBER"
echo "RFD Path: $RFD_PATH"
```

**Expected JSON Response**:
```json
{
  "status": "success",
  "message": "RFD created successfully",
  "data": {
    "rfd_number": "003",
    "title": "Git Workflow Automation",
    "path": "rfds/0003-git-workflow-automation.md",
    "state": "draft",
    "created": "2025-10-17"
  }
}
```

---

## Phase 3: Populate RFD Content

### Step 3.1: Show RFD Location

```bash
echo ""
echo "📄 RFD created at: $RFD_PATH"
echo ""
echo "Next steps:"
echo "1. Edit the RFD file to fill in sections"
echo "2. Use the rfd-manager Skill for guidance"
echo "3. Update status when ready: cargo run --bin rfd -- status $RFD_NUMBER --set review"
```

### Step 3.2: Optionally Open for Editing

```bash
# Ask if user wants to open for editing
echo ""
echo "Would you like to open the RFD for editing? (y/n)"
read -r RESPONSE

if [ "$RESPONSE" = "y" ] || [ "$RESPONSE" = "Y" ]; then
    # Open in default editor
    ${EDITOR:-vi} "$RFD_PATH"
fi
```

### Step 3.3: Provide Template Guidance

```markdown
The RFD has been created with the following structure:

## Required Sections

1. **Summary**: Brief overview (1-2 paragraphs)
2. **Motivation**: Why this RFD exists, what problem it solves
3. **Proposal**: Detailed technical proposal
4. **Implementation**: How to build it
5. **Alternatives**: Other approaches considered
6. **Open Questions**: Unresolved issues

## Optional Sections

- **Security Considerations**: If applicable
- **Performance Targets**: Specific metrics
- **Migration Strategy**: For changes to existing systems
- **References**: External resources

Fill in each section with detailed information.
```

---

## Phase 4: Validation

### Step 4.1: Validate RFD Structure

```bash
# Validate the RFD
VALIDATION=$(cargo run --bin rfd -- validate $RFD_NUMBER --format json)

# Check validation result
IS_VALID=$(echo "$VALIDATION" | jq -r '.valid')

if [ "$IS_VALID" = "true" ]; then
    echo "✅ RFD structure is valid"
else
    echo "⚠️ RFD has validation issues:"
    echo "$VALIDATION" | jq -r '.issues[]'
    echo ""
    echo "Fix these issues before moving to review"
fi
```

**Validation Response**:
```json
{
  "valid": false,
  "issues": [
    "Missing required section: Summary",
    "Missing required section: Motivation"
  ]
}
```

---

## Phase 5: Next Steps

### Step 5.1: Provide Workflow Guidance

```bash
echo ""
echo "📋 RFD Workflow:"
echo ""
echo "Current state: draft"
echo ""
echo "Available transitions:"
echo "  1. Move to review:   cargo run --bin rfd -- status $RFD_NUMBER --set review"
echo "  2. Accept directly:  cargo run --bin rfd -- status $RFD_NUMBER --set accepted"
echo "  3. Reject:           cargo run --bin rfd -- status $RFD_NUMBER --set rejected"
echo ""
echo "RFD State Machine:"
echo "  draft → review → accepted → implemented"
echo "    │      │         │"
echo "    └──────┴─────────┴──> rejected → archived"
echo ""
```

### Step 5.2: Suggest Using rfd-manager Skill

```markdown
💡 Tip: Use the rfd-manager Skill for:
- Listing all RFDs by status
- Updating RFD metadata
- Moving RFDs through the review process
- Converting existing RFCs to RFD format

Example: "List all RFDs in draft state"
```

---

## Complete Example Workflow

```bash
#!/bin/bash
# Complete RFD creation workflow

TITLE="Git Workflow Automation Tool"
AUTHOR="Ada <ada@example.com>"

# 1. Create RFD
echo "Creating RFD: $TITLE"
OUTPUT=$(cargo run --bin rfd -- create \
    --title "$TITLE" \
    --author "$AUTHOR" \
    --format json)

# 2. Parse response
RFD_NUMBER=$(echo "$OUTPUT" | jq -r '.data.rfd_number')
RFD_PATH=$(echo "$OUTPUT" | jq -r '.data.path')

echo "✅ RFD $RFD_NUMBER created at $RFD_PATH"

# 3. Show the RFD
echo ""
echo "RFD Contents:"
cargo run --bin rfd -- show $RFD_NUMBER

# 4. Validate
echo ""
echo "Validating RFD structure..."
cargo run --bin rfd -- validate $RFD_NUMBER --format json | jq '.'

# 5. List all RFDs
echo ""
echo "All RFDs:"
cargo run --bin rfd -- list --format json | jq '.rfds[] | {id, title, state}'
```

---

## Integration with Skills

The rfd-manager Skill can invoke this command automatically:

```markdown
# In skills/rfd-manager/SKILL.md

When user wants to create an RFD:

1. Use the create-rfd command
2. Gather title and author
3. Create via: cargo run --bin rfd -- create --title "..." --author "..."
4. Parse JSON response
5. Guide user through filling in sections
```

---

## Error Handling

### RFD CLI Not Found

```bash
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Install Rust: https://rustup.rs/"
    exit 1
fi

if ! cargo run --bin rfd -- --version &> /dev/null; then
    echo "❌ RFD CLI not available"
    echo "Build it: cargo build --bin rfd"
    exit 1
fi
```

### Invalid Author Format

```bash
# Validate author format: Name <email@example.com>
if ! echo "$AUTHOR" | grep -qE '^.+ <.+@.+\..+>$'; then
    echo "❌ Invalid author format"
    echo "Expected: Name <email@example.com>"
    echo "Got: $AUTHOR"
    exit 1
fi
```

### RFD Creation Failed

```bash
if [ $? -ne 0 ]; then
    echo "❌ RFD creation failed"

    # Parse error from JSON
    ERROR_CODE=$(echo "$OUTPUT" | jq -r '.error')
    ERROR_MSG=$(echo "$OUTPUT" | jq -r '.message')

    echo "Error: $ERROR_CODE"
    echo "$ERROR_MSG"

    # Show suggestion if available
    if echo "$OUTPUT" | jq -e '.suggestion' > /dev/null; then
        SUGGESTION=$(echo "$OUTPUT" | jq -r '.suggestion.command')
        echo "Try: $SUGGESTION"
    fi

    exit 1
fi
```

---

## Automation Script

For automated RFD creation:

```bash
#!/bin/bash
# .claude/scripts/create-rfd.sh

TITLE="$1"
AUTHOR="${2:-$(git config user.name) <$(git config user.email)>}"

if [ -z "$TITLE" ]; then
    echo "Usage: $0 <title> [author]"
    exit 1
fi

# Create RFD
cargo run --bin rfd -- create \
    --title "$TITLE" \
    --author "$AUTHOR" \
    --format json | jq '{
        rfd_number: .data.rfd_number,
        path: .data.path,
        title: .data.title,
        state: .data.state
    }'
```

---

## Dogfooding Benefits

Using our own RFD CLI demonstrates:

1. **Agent-Friendly Design**: JSON output is parseable
2. **Idempotent Operations**: Safe to retry
3. **Actionable Errors**: Clear messages with suggestions
4. **Fast Execution**: CLI tools start quickly
5. **Skills Integration**: Skills can invoke CLI seamlessly

---

## Converting Existing RFCs to RFDs

To convert an existing RFC:

```bash
#!/bin/bash
# Convert RFC to RFD

RFC_FILE="$1"
TITLE=$(grep "^# RFC" "$RFC_FILE" | sed 's/^# RFC-[0-9]*: //')
AUTHOR="${2:-$(git log -1 --format='%an <%ae>' "$RFC_FILE")}"

# Create new RFD
OUTPUT=$(cargo run --bin rfd -- create \
    --title "$TITLE" \
    --author "$AUTHOR" \
    --format json)

RFD_NUMBER=$(echo "$OUTPUT" | jq -r '.data.rfd_number')
RFD_PATH=$(echo "$OUTPUT" | jq -r '.data.path')

# Copy content (adapting structure)
echo "Created RFD $RFD_NUMBER from $RFC_FILE"
echo "Manually copy content from RFC to $RFD_PATH"
echo "Adapt sections to RFD structure"
```

---

## Best Practices

### When to Create an RFD

Create an RFD for:
- New features or tools
- Architectural decisions
- Process changes
- Technical specifications
- Design proposals

**Don't create RFDs for**:
- Bug fixes (use issues/PRs)
- Minor changes
- Temporary decisions
- Urgent changes (document after)

### RFD Title Guidelines

**Good Titles**:
- "Git Workflow Automation Tool"
- "Hybrid Architecture for AI-Assisted Development"
- "RFD CLI with Agent-Friendly Design"

**Bad Titles**:
- "Tool" (too vague)
- "RFC-003" (not descriptive)
- "Thing we should do" (not specific)

### Author Attribution

Always include:
- Full name
- Email address
- Format: `Name <email@example.com>`

Can include multiple authors:
```bash
--author "Alice <alice@example.com>, Bob <bob@example.com>"
```

---

## Command Options Reference

```bash
# Basic creation
cargo run --bin rfd -- create \
    --title "TITLE" \
    --author "NAME <EMAIL>"

# With template
cargo run --bin rfd -- create \
    --title "TITLE" \
    --author "NAME <EMAIL>" \
    --template "adr"

# JSON output (for agents)
cargo run --bin rfd -- create \
    --title "TITLE" \
    --author "NAME <EMAIL>" \
    --format json

# Quiet mode (errors only)
cargo run --bin rfd -- create \
    --title "TITLE" \
    --author "NAME <EMAIL>" \
    --format quiet
```

---

## Final Checklist

- [ ] RFD CLI is built and available
- [ ] Title is descriptive and clear
- [ ] Author is in correct format
- [ ] RFD created successfully
- [ ] RFD number obtained
- [ ] RFD file exists at expected path
- [ ] Structure is valid
- [ ] Ready to fill in sections

---

**Remember**: The RFD CLI is designed for both humans and agents. Use JSON output when scripting, pretty output for interactive use!
