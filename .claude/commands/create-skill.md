# Command: Create Skill

**Name**: create-skill
**Description**: Scaffold a new Skill following Claude Code Skills standards
**Parameters**: `$SKILL_NAME`, `$DESCRIPTION`
**Example**: `/create-skill rfd-manager "Manage RFD documents with structured operations"`

---

## Overview

This command scaffolds a new Skill for Claude Code. Skills are model-invoked capabilities that guide Claude's behavior through markdown documentation with YAML frontmatter.

**Key Characteristics of Skills**:
- **Model-invoked**: Claude autonomously decides when to use them
- **Token-efficient**: Few dozen tokens vs tens of thousands for MCP
- **Discoverable**: Based on `description` field in frontmatter
- **Composable**: Multiple Skills can work together

---

## Phase 1: Validate Skill Concept

### Step 1.1: Determine if a Skill is Appropriate

Ask yourself:
- ✅ Does this orchestrate multiple tools or provide workflow guidance?
- ✅ Is this something Claude should invoke automatically based on context?
- ✅ Does this encode team conventions or best practices?
- ❌ Does this require stateful operations? (Use MCP tool instead)
- ❌ Does this need < 10ms startup time for operations? (Use CLI tool instead)

**If mostly ✅**: Proceed with Skill creation

 **If mostly ❌**: Consider:
- CLI tool for fast, agent-friendly operations
- MCP tool for deep integration and stateful operations

### Step 1.2: Define Skill Scope

```yaml
skill_concept:
  name: {SKILL_NAME}
  purpose: {What problem does this solve?}
  triggers: {When should Claude invoke this?}
  tools_required: {Which tools will this use?}
  workflows: {What workflows will this orchestrate?}
```

---

## Phase 2: Create Skill Structure

### Step 2.1: Create Skill Directory

```bash
# Convert kebab-case to directory name
SKILL_NAME_DIR=$(echo "$SKILL_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '-')

# Determine location
# Personal Skills: ~/.claude/skills/$SKILL_NAME_DIR
# Project Skills: .claude/skills/$SKILL_NAME_DIR

# For Tapestry, use project skills
mkdir -p skills/$SKILL_NAME_DIR

echo "✅ Created directory: skills/$SKILL_NAME_DIR"
```

### Step 2.2: Create SKILL.md with Frontmatter

The `description` field is CRITICAL for discovery. It should include:
- **Functionality**: What the Skill does
- **Usage triggers**: When Claude should use it

**Good description**: *"Manage RFD documents with structured operations. Use when working with RFDs, creating technical documentation, or when the user mentions RFDs, design docs, or documentation workflows."*

**Bad description**: *"Helps with documents"* (too vague)

```bash
cat > skills/$SKILL_NAME_DIR/SKILL.md << 'EOF'
---
name: $SKILL_NAME
description: $DESCRIPTION
allowed-tools: Read, Write, Bash, Grep, Glob
---

# $SKILL_NAME

## Purpose

{Brief overview of what this Skill accomplishes}

## When to Use

Claude should invoke this Skill when:
- {Trigger scenario 1}
- {Trigger scenario 2}
- {Trigger scenario 3}

## Instructions

### Prerequisites

Check that:
- {Required tool or resource 1} is available
- {Required tool or resource 2} is configured

### Core Workflow

1. **{Step 1 Title}**
   ```bash
   # Example command
   {command}
   ```

   **What this does**: {Explanation}

   **Expected output**: {What to expect}

2. **{Step 2 Title}**
   {Instructions for step 2}

3. **{Step 3 Title}**
   {Instructions for step 3}

### Error Handling

If you encounter errors:

- **Error Type 1**: {Solution or workaround}
- **Error Type 2**: {Solution or workaround}

## Examples

### Example 1: {Common Use Case}

```{language}
{Example code or command}
```

**Outcome**: {What happens}

### Example 2: {Another Common Use Case}

```{language}
{Example code or command}
```

**Outcome**: {What happens}

## Common Workflows

### Workflow 1: {Workflow Name}

1. {Step 1}
2. {Step 2}
3. {Step 3}

### Workflow 2: {Workflow Name}

1. {Step 1}
2. {Step 2}
3. {Step 3}

## Best Practices

- **{Practice 1}**: {Details}
- **{Practice 2}**: {Details}
- **{Practice 3}**: {Details}

## Integration with Other Tools/Skills

This Skill works well with:
- **{Tool/Skill 1}**: {How they integrate}
- **{Tool/Skill 2}**: {How they integrate}

## Notes

{Additional information, caveats, or future enhancements}

---

**Skill Status**: 🚧 In Development
**Last Updated**: {DATE}
**Maintainer**: Tapestry Team
EOF

echo "✅ Created SKILL.md"
```

### Step 2.3: Add Supporting Files (Optional)

Skills can include additional resources:

```bash
# Create supporting file structure
mkdir -p skills/$SKILL_NAME_DIR/examples
mkdir -p skills/$SKILL_NAME_DIR/reference
mkdir -p skills/$SKILL_NAME_DIR/templates

# Example: Create a reference guide
cat > skills/$SKILL_NAME_DIR/reference/api-reference.md << 'EOF'
# API Reference for {SKILL_NAME}

{Detailed API documentation, command references, etc.}
EOF

# Example: Create a template
cat > skills/$SKILL_NAME_DIR/templates/example-template.txt << 'EOF'
{Template content that the Skill might use}
EOF

echo "✅ Created supporting files"
```

---

## Phase 3: Optimize for Discovery

### Step 3.1: Craft the Description

The `description` field determines when Claude invokes your Skill. Make it:

**Specific**: Include concrete keywords
- ✅ "Extract text from PDF files, fill forms, merge documents"
- ❌ "Helps with PDFs"

**Trigger-rich**: Include invocation scenarios
- ✅ "Use when working with PDF files or when the user mentions PDFs, forms, or document extraction"
- ❌ "PDF tool"

**Functional**: Describe what it does
- ✅ "Automates git workflow with conventional commits, branch management, and PR creation"
- ❌ "Git helper"

### Step 3.2: Define Tool Restrictions (Optional)

Use `allowed-tools` to limit which tools Claude can use when this Skill is active:

```yaml
# Example: Restrict to read-only operations
allowed-tools: Read, Grep, Glob

# Example: Allow file operations but no execution
allowed-tools: Read, Write, Edit, Grep, Glob

# Example: Full access (default if omitted)
# allowed-tools: Read, Write, Edit, Bash, Grep, Glob, WebFetch
```

**When to restrict**:
- Security-sensitive Skills
- Read-only analysis Skills
- Skills that shouldn't modify files

**When NOT to restrict**:
- Workflow orchestration Skills
- Skills that need flexibility
- Skills used across different contexts

---

## Phase 4: Test the Skill

### Step 4.1: Manual Invocation Test

```markdown
# In your Claude Code session:

Test the Skill by triggering its usage context:

"I need to {action that should trigger the Skill}"

# Claude should recognize and invoke your Skill automatically
```

### Step 4.2: Validate Discovery

Check that:
- [ ] Claude invokes the Skill when appropriate
- [ ] Skill instructions are clear and actionable
- [ ] Examples work as documented
- [ ] Error handling is adequate
- [ ] Supporting files are referenced correctly

### Step 4.3: Refine Based on Usage

After testing:
1. **Improve description** if Skill isn't invoked when expected
2. **Clarify instructions** if Claude misinterprets steps
3. **Add examples** for common failure modes
4. **Update workflows** based on real usage patterns

---

## Phase 5: Documentation & Integration

### Step 5.1: Document the Skill

Add to project documentation:

```bash
# Update Skills README (if it exists)
if [ -f skills/README.md ]; then
    echo "## $SKILL_NAME

$DESCRIPTION

**Location**: \`skills/$SKILL_NAME_DIR/\`
**Status**: Active
**Tools**: {List tools used}

" >> skills/README.md
fi
```

### Step 5.2: Create Usage Guide (Optional)

For complex Skills, create a user guide:

```bash
cat > skills/$SKILL_NAME_DIR/USAGE.md << 'EOF'
# Using the $SKILL_NAME Skill

## Quick Start

{Quick examples for common use cases}

## Advanced Usage

{Complex scenarios and edge cases}

## Troubleshooting

{Common issues and solutions}
EOF
```

### Step 5.3: Update Project State

Track the new Skill:

```bash
# Add to project state tracking
echo "- [x] Created $SKILL_NAME Skill" >> .claude/context/project-state.md
```

---

## Skill Development Best Practices

### Writing Effective Instructions

**Do**:
- Use clear, imperative language ("Run this command", "Check that...")
- Provide concrete examples
- Include expected outputs
- Explain *why* steps are needed
- Handle common errors

**Don't**:
- Assume prior knowledge
- Use vague language ("might need to...", "possibly...")
- Omit error handling
- Forget to explain outputs

### Structuring Workflows

**Good Workflow Structure**:
```markdown
### Workflow: {Name}

**When to use**: {Context}

1. **{Action}**: `command`
   - Why: {Reason}
   - Expected: {Output}

2. **{Action}**: `command`
   - Why: {Reason}
   - Expected: {Output}
```

**Poor Workflow Structure**:
```markdown
Do this, then that, then maybe this other thing.
```

### Choosing Tool Restrictions

| Skill Type | Suggested Tools | Rationale |
|------------|----------------|-----------|
| Read-only analysis | Read, Grep, Glob | Prevent unintended modifications |
| Documentation | Read, Write, Edit, Grep, Glob | File operations only |
| Workflow orchestration | * (all tools) | Needs flexibility |
| External integration | Read, Write, Bash, WebFetch | Allow external calls |

---

## Examples

### Example 1: Simple Skill (Read-Only Analysis)

```yaml
---
name: code-complexity-analyzer
description: Analyze code complexity metrics and identify refactoring opportunities. Use when analyzing code quality, reviewing complexity, or when user mentions cyclomatic complexity, code metrics, or refactoring candidates.
allowed-tools: Read, Grep, Glob
---

# Code Complexity Analyzer

## Purpose
Identify complex functions and suggest refactoring targets.

## Instructions
1. **Find all source files**: `glob "**/*.{js,ts,py,rs}"`
2. **Analyze function length**: Look for functions > 50 lines
3. **Report complexity**: List files with high complexity

## Examples
See SKILL.md for full examples.
```

### Example 2: Workflow Orchestration Skill

```yaml
---
name: api-endpoint-creator
description: Create new API endpoints following project conventions with routes, handlers, tests, and documentation. Use when adding new API endpoints, creating REST routes, or when user mentions API development.
allowed-tools: Read, Write, Edit, Bash, Grep, Glob
---

# API Endpoint Creator

## Purpose
Scaffold complete API endpoints with all necessary files.

## Workflows
### Workflow 1: Create GET Endpoint
1. Create route definition
2. Implement handler
3. Add validation
4. Write tests
5. Update API docs

See SKILL.md for detailed steps.
```

---

## Automation via Shell Script

For automated Skill creation, you can use the scaffolding script:

```bash
./.claude/scripts/scaffold-skill.sh "$SKILL_NAME" "$DESCRIPTION"
```

See `.claude/scripts/scaffold-skill.sh` for the implementation.

---

## Skill Lifecycle

```
Concept → Create → Test → Refine → Document → Maintain
   ↑                                              ↓
   └──────────── Iterate based on usage ─────────┘
```

### Maintenance

Skills should be updated when:
- Tool capabilities change
- New workflows emerge
- Better patterns are discovered
- Errors are encountered
- User feedback is received

---

## Troubleshooting

### Skill Not Being Invoked

**Symptoms**: Claude doesn't use the Skill when expected

**Solutions**:
1. **Improve description**: Add more trigger keywords
2. **Make it more specific**: Describe exact scenarios
3. **Check location**: Ensure SKILL.md is in correct directory
4. **Verify frontmatter**: YAML syntax must be valid

### Skill Instructions Unclear

**Symptoms**: Claude misinterprets or skips steps

**Solutions**:
1. **Add more examples**: Show expected vs actual
2. **Be more explicit**: Remove ambiguity
3. **Include outputs**: Show what success looks like
4. **Add error cases**: Handle common failures

### Tool Restrictions Too Tight

**Symptoms**: Skill can't complete tasks due to tool limitations

**Solutions**:
1. **Expand allowed-tools**: Add necessary tools
2. **Split the Skill**: Create separate Skills with different tool needs
3. **Remove restrictions**: Let Claude use all tools

---

## Final Checklist

Before marking a Skill as complete:

- [ ] SKILL.md has valid YAML frontmatter
- [ ] Description includes functionality AND triggers
- [ ] Instructions are clear and actionable
- [ ] Examples demonstrate common use cases
- [ ] Error handling is documented
- [ ] Tool restrictions are appropriate
- [ ] Skill has been tested in real scenarios
- [ ] Supporting files (if any) are referenced correctly
- [ ] Project documentation is updated
- [ ] Skill directory is in correct location

---

**Remember**: Skills are living documents. Start simple, test with real usage, and iterate based on what works!
