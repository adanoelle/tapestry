#!/bin/bash
# Scaffold Skill - Automated scaffolding for Claude Code Skills
# This script is invoked by the create-skill command

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ️${NC} $1"
}

# Check if we're in the right directory
if [ ! -d ".claude" ]; then
    print_error "Must be run from Tapestry project root (where .claude/ exists)"
    exit 1
fi

# Parse arguments
SKILL_NAME=$1
DESCRIPTION=$2

if [ -z "$SKILL_NAME" ] || [ -z "$DESCRIPTION" ]; then
    echo "Usage: $0 <skill-name> <description>"
    echo "Example: $0 \"rfd-manager\" \"Manage RFD documents with structured operations\""
    exit 1
fi

# Convert to directory name (kebab-case)
SKILL_DIR=$(echo "$SKILL_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '-')

print_info "Creating Skill: $SKILL_NAME"
print_info "Directory: skills/$SKILL_DIR"
print_info "Description: $DESCRIPTION"

# Phase 1: Create Skill directory structure
print_status "Phase 1: Creating directory structure..."

mkdir -p skills/$SKILL_DIR
mkdir -p skills/$SKILL_DIR/examples
mkdir -p skills/$SKILL_DIR/reference
mkdir -p skills/$SKILL_DIR/templates

print_status "Created directory structure"

# Phase 2: Create SKILL.md
print_status "Phase 2: Creating SKILL.md..."

TODAY=$(date +%Y-%m-%d)

cat > skills/$SKILL_DIR/SKILL.md << EOF
---
name: $SKILL_NAME
description: $DESCRIPTION
allowed-tools: Read, Write, Bash, Grep, Glob
---

# $SKILL_NAME

## Purpose

$DESCRIPTION

## When to Use

Claude should invoke this Skill when:
- TODO: Define trigger scenario 1
- TODO: Define trigger scenario 2
- TODO: Define trigger scenario 3

## Instructions

### Prerequisites

Check that:
- TODO: Define prerequisite 1
- TODO: Define prerequisite 2

### Core Workflow

1. **Step 1: TODO**
   \`\`\`bash
   # TODO: Add example command
   \`\`\`

   **What this does**: TODO: Explain purpose

   **Expected output**: TODO: Describe expected result

2. **Step 2: TODO**
   - TODO: Define step 2 instructions

3. **Step 3: TODO**
   - TODO: Define step 3 instructions

### Error Handling

If you encounter errors:

- **Error Type 1**: TODO: Define solution
- **Error Type 2**: TODO: Define solution

## Examples

### Example 1: TODO - Common Use Case

\`\`\`bash
# TODO: Add example code
\`\`\`

**Outcome**: TODO: Describe what happens

### Example 2: TODO - Another Use Case

\`\`\`bash
# TODO: Add example code
\`\`\`

**Outcome**: TODO: Describe what happens

## Common Workflows

### Workflow 1: TODO - Workflow Name

1. TODO: Step 1
2. TODO: Step 2
3. TODO: Step 3

### Workflow 2: TODO - Workflow Name

1. TODO: Step 1
2. TODO: Step 2
3. TODO: Step 3

## Best Practices

- **TODO: Practice 1**: Details here
- **TODO: Practice 2**: Details here
- **TODO: Practice 3**: Details here

## Integration with Other Tools/Skills

This Skill works well with:
- **TODO: Tool/Skill 1**: How they integrate
- **TODO: Tool/Skill 2**: How they integrate

## Notes

TODO: Additional information, caveats, or future enhancements

---

**Skill Status**: 🚧 In Development
**Last Updated**: $TODAY
**Maintainer**: Tapestry Team
EOF

print_status "Created SKILL.md with frontmatter"

# Phase 3: Create supporting files (optional templates)
print_status "Phase 3: Creating supporting files..."

# Create a README for the Skill directory
cat > skills/$SKILL_DIR/README.md << EOF
# $SKILL_NAME Skill

$DESCRIPTION

## Files

- **SKILL.md**: Main Skill definition (required)
- **examples/**: Example workflows and use cases
- **reference/**: Reference documentation
- **templates/**: Templates used by this Skill

## Status

**Status**: 🚧 In Development
**Created**: $TODAY
**Last Updated**: $TODAY

## Usage

Claude will automatically invoke this Skill when appropriate based on the description in SKILL.md frontmatter.

## Development

To test this Skill:
1. Trigger scenarios defined in "When to Use" section
2. Validate that Claude invokes the Skill
3. Refine description if needed
4. Update instructions based on testing

## Maintenance

Update this Skill when:
- Tool capabilities change
- New workflows emerge
- Better patterns are discovered
- User feedback is received
EOF

print_status "Created README.md"

# Create an example template
cat > skills/$SKILL_DIR/templates/example-template.txt << EOF
# Example Template for $SKILL_NAME

TODO: Add template content that this Skill might use
EOF

print_status "Created example template"

# Phase 4: Update project Skills README
print_status "Phase 4: Updating project documentation..."

# Create or update skills/README.md
if [ ! -f skills/README.md ]; then
    cat > skills/README.md << 'EOF'
# Tapestry Skills

This directory contains Skills for Claude Code. Skills are model-invoked capabilities that guide Claude's behavior through markdown documentation.

## What are Skills?

Skills are lightweight markdown files with YAML frontmatter that:
- Are **model-invoked**: Claude decides when to use them
- Are **token-efficient**: Few dozen tokens vs thousands for MCP
- Provide **workflow guidance**: Orchestrate tools and encode best practices
- Are **discoverable**: Based on description field

## Available Skills

EOF
fi

# Append new Skill
cat >> skills/README.md << EOF

### $SKILL_NAME

**Location**: \`skills/$SKILL_DIR/\`
**Description**: $DESCRIPTION
**Status**: 🚧 In Development
**Created**: $TODAY

EOF

print_status "Updated skills/README.md"

# Phase 5: Validation
print_status "Phase 5: Running validation..."

# Check YAML frontmatter validity (basic check)
if head -n 10 skills/$SKILL_DIR/SKILL.md | grep -q "^---$"; then
    print_status "YAML frontmatter format looks valid"
else
    print_warning "YAML frontmatter may have formatting issues"
fi

# Check for required sections
REQUIRED_SECTIONS=("Purpose" "When to Use" "Instructions" "Examples")
for section in "${REQUIRED_SECTIONS[@]}"; do
    if grep -q "## $section" skills/$SKILL_DIR/SKILL.md; then
        print_status "Found required section: $section"
    else
        print_warning "Missing section: $section (add it manually)"
    fi
done

# Final summary
echo ""
echo "======================================="
echo -e "${GREEN}✅ Skill '$SKILL_NAME' scaffolded successfully!${NC}"
echo "======================================="
echo ""
echo "📁 Structure created:"
echo "  - Skill definition: skills/$SKILL_DIR/SKILL.md"
echo "  - Examples directory: skills/$SKILL_DIR/examples/"
echo "  - Reference directory: skills/$SKILL_DIR/reference/"
echo "  - Templates directory: skills/$SKILL_DIR/templates/"
echo ""
echo "📝 Next steps:"
echo "  1. Edit skills/$SKILL_DIR/SKILL.md and remove all TODO markers"
echo "  2. Define clear trigger scenarios in 'When to Use' section"
echo "  3. Write detailed, actionable instructions"
echo "  4. Add concrete examples with expected outcomes"
echo "  5. Test the Skill by triggering its usage scenarios"
echo "  6. Refine the description if Claude doesn't invoke it"
echo "  7. Mark status as '✅ Active' when complete"
echo ""
echo "🔍 Testing tips:"
echo "  - Trigger: Say something that matches your description"
echo "  - Validate: Check that Claude invokes the Skill automatically"
echo "  - Refine: Update description if discovery fails"
echo ""
echo "📚 Documentation:"
echo "  - Command: .claude/commands/create-skill.md"
echo "  - Workflow: .claude/workflows/new-skill.md"
echo "  - Skills Guide: https://docs.claude.com/en/docs/claude-code/skills"
echo ""
echo "Happy Skill building! 🎉"
