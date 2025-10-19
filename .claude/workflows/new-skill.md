# Workflow: Create New Skill

## Workflow Metadata

**Workflow ID**: new-skill
**Version**: 1.0
**Purpose**: Systematic process for creating a new Skill from concept to deployment
**Duration**: 1-2 hours
**Agents Required**: skills-designer (optional), documenter (optional)

## Prerequisites

- [ ] Skill concept validated (orchestration/workflow vs tool)
- [ ] Use cases identified
- [ ] Trigger scenarios defined
- [ ] Required tools identified

## Workflow Phases

### Phase 1: Concept & Design (30 min)

**Lead**: Skills Designer (or primary developer)
**Duration**: 20-30 minutes

#### Step 1.1: Define Skill Purpose

```yaml
skill_concept:
  name: {SKILL_NAME}
  purpose: |
    {What problem does this Skill solve?
     What workflows does it orchestrate?}
  triggers: |
    {When should Claude invoke this automatically?
     What keywords or contexts should trigger it?}
  tools_required:
    - {Tool 1}
    - {Tool 2}
  outputs:
    - {What the Skill produces}
```

**Prompt**:
```markdown
Define the Skill concept for {SKILL_NAME}. Consider:
- What workflows will this orchestrate?
- When should Claude invoke this automatically?
- Which tools will it use?
- What are the expected outputs?
```

#### Step 1.2: Identify Workflows

List all workflows this Skill will support:

1. **Workflow 1**: {Name}
   - Steps: {High-level steps}
   - Tools: {Tools needed}
   - Output: {What it produces}

2. **Workflow 2**: {Name}
   - Steps: {High-level steps}
   - Tools: {Tools needed}
   - Output: {What it produces}

#### Step 1.3: Define Discovery Description

Craft the description that will appear in YAML frontmatter:

**Template**:
```
{What it does} + {When to use it} + {Trigger keywords}
```

**Example**:
```
Manage RFD documents with structured operations. Use when working with RFDs, creating technical documentation, or when the user mentions RFDs, design docs, or documentation workflows.
```

**Gate**: Concept must be clear before proceeding to Phase 2

---

### Phase 2: Skill Creation (20-30 min)

**Lead**: Primary developer
**Duration**: 20-30 minutes

#### Step 2.1: Create Skill Directory Structure

```bash
# Convert name to directory format
SKILL_DIR=$(echo "$SKILL_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '-')

# Create directory in project skills
mkdir -p skills/$SKILL_DIR

# Optionally create supporting directories
mkdir -p skills/$SKILL_DIR/examples
mkdir -p skills/$SKILL_DIR/reference
mkdir -p skills/$SKILL_DIR/templates
```

**Output**:
- Created directory: `skills/{skill-name}/`

#### Step 2.2: Create SKILL.md

```yaml
agent: documenter (optional)
input:
  - Skill concept from Phase 1
  - Workflow definitions
  - Tool requirements
output:
  - skills/{skill-name}/SKILL.md
  - Complete frontmatter with description
  - Structured instructions
artifacts:
  - SKILL.md file
```

**Structure**:
```markdown
---
name: {SKILL_NAME}
description: {Crafted description with triggers}
allowed-tools: {Tool1, Tool2, Tool3}
---

# {SKILL_NAME}

## Purpose
{What this accomplishes}

## When to Use
{Trigger scenarios}

## Instructions
{Step-by-step workflows}

## Examples
{Concrete examples}

## Best Practices
{Guidelines and tips}
```

#### Step 2.3: Add Supporting Files (if needed)

Create additional resources:

```bash
# Example reference guide
cat > skills/$SKILL_DIR/reference/commands.md << 'EOF'
# Command Reference

{Detailed command documentation}
EOF

# Example template
cat > skills/$SKILL_DIR/templates/template.txt << 'EOF'
{Template content}
EOF

# Example workflows
cat > skills/$SKILL_DIR/examples/example-workflow.md << 'EOF'
# Example Workflow

{Complete example}
EOF
```

---

### Phase 3: Testing & Refinement (20-30 min)

**Lead**: Primary developer
**Duration**: 20-30 minutes

#### Step 3.1: Manual Invocation Test

Test the Skill by creating scenarios that should trigger it:

```markdown
# Test Scenario 1: Direct Mention
"I need to {action that matches Skill description}"

# Test Scenario 2: Contextual Trigger
{Provide context that should invoke the Skill}

# Test Scenario 3: Workflow Execution
{Walk through a complete workflow}
```

**Validation Checklist**:
- [ ] Claude invokes Skill automatically when appropriate
- [ ] Instructions are clear and actionable
- [ ] Examples work as documented
- [ ] Error handling is adequate
- [ ] Tool restrictions are appropriate

#### Step 3.2: Refine Based on Testing

Common refinements:

**If Skill isn't invoked**:
- Improve description with more trigger keywords
- Make triggers more specific
- Add explicit usage scenarios

**If instructions are unclear**:
- Add more examples
- Be more explicit in steps
- Include expected outputs
- Show error cases

**If tools are insufficient**:
- Expand `allowed-tools` list
- Document why specific tools are needed
- Consider splitting into multiple Skills

#### Step 3.3: Edge Case Handling

Test edge cases:
- Missing prerequisites
- Invalid inputs
- Tool failures
- Unexpected states

Document solutions in SKILL.md

---

### Phase 4: Documentation & Integration (15-20 min)

**Lead**: Documenter
**Duration**: 15-20 minutes

#### Step 4.1: Update Project Documentation

```bash
# Add to Skills README
if [ ! -f skills/README.md ]; then
    cat > skills/README.md << 'EOF'
# Tapestry Skills

This directory contains Skills for Claude Code.

## Available Skills

EOF
fi

# Append new Skill
cat >> skills/README.md << EOF

### $SKILL_NAME

**Location**: \`skills/$SKILL_DIR/\`
**Description**: $DESCRIPTION
**Status**: Active
**Tools Used**: {List}

EOF
```

#### Step 4.2: Create Usage Examples

For complex Skills, create comprehensive usage guide:

```bash
cat > skills/$SKILL_DIR/USAGE.md << 'EOF'
# Using {SKILL_NAME}

## Quick Start

{Common use cases with examples}

## Advanced Usage

{Complex scenarios}

## Troubleshooting

{Common issues and solutions}

## FAQ

{Frequently asked questions}
EOF
```

#### Step 4.3: Update Project Tracking

```bash
# Update project state
echo "- [x] Created $SKILL_NAME Skill ($(date +%Y-%m-%d))" >> .claude/context/project-state.md

# Update CHANGELOG if applicable
echo "### Added
- **Skill**: $SKILL_NAME - $DESCRIPTION" >> CHANGELOG.md
```

---

### Phase 5: Maintenance Plan (10 min)

**Lead**: Project maintainer
**Duration**: 10 minutes

#### Step 5.1: Define Update Triggers

Document when this Skill should be updated:

```yaml
maintenance_triggers:
  - Tool capabilities change
  - New workflows discovered
  - Better patterns emerge
  - User feedback received
  - Errors encountered
```

#### Step 5.2: Ownership

Assign responsibility:

```yaml
maintainer: {Team/Person}
review_frequency: {Monthly/Quarterly}
last_reviewed: {DATE}
```

---

## Workflow Summary

```
Phase 1: Concept & Design (30 min)
  ↓
Phase 2: Skill Creation (20-30 min)
  ↓
Phase 3: Testing & Refinement (20-30 min)
  ↓
Phase 4: Documentation & Integration (15-20 min)
  ↓
Phase 5: Maintenance Plan (10 min)
  ↓
Complete! (Total: 1-2 hours)
```

---

## Deliverables

### Required Files

- [ ] `skills/{skill-name}/SKILL.md` - Main Skill file
- [ ] `skills/README.md` - Updated with new Skill
- [ ] `.claude/context/project-state.md` - Updated tracking

### Optional Files

- [ ] `skills/{skill-name}/USAGE.md` - Usage guide
- [ ] `skills/{skill-name}/reference/*.md` - Reference docs
- [ ] `skills/{skill-name}/templates/*` - Templates
- [ ] `skills/{skill-name}/examples/*` - Examples

---

## Automation

For faster Skill creation, use the scaffolding script:

```bash
./.claude/scripts/scaffold-skill.sh "$SKILL_NAME" "$DESCRIPTION"
```

This automates:
- Directory creation
- SKILL.md scaffolding
- Supporting file structure
- Basic validation

---

## Quality Checklist

Before marking a Skill as complete:

### Content Quality

- [ ] Description includes functionality AND triggers
- [ ] Instructions are clear and actionable
- [ ] Examples demonstrate common use cases
- [ ] Error handling is documented
- [ ] Best practices are included

### Technical Quality

- [ ] YAML frontmatter is valid
- [ ] Tool restrictions are appropriate
- [ ] Supporting files are referenced correctly
- [ ] File paths are correct

### Testing Quality

- [ ] Tested with real scenarios
- [ ] Edge cases handled
- [ ] Works with other Skills (if applicable)
- [ ] User feedback incorporated

### Documentation Quality

- [ ] Project docs updated
- [ ] Usage examples provided
- [ ] Troubleshooting guide included
- [ ] Maintenance plan defined

---

## Examples

### Example 1: Simple Skill (rfd-manager)

**Phase 1 Output**:
```yaml
name: rfd-manager
purpose: Orchestrate RFD document operations using rfd CLI
triggers:
  - Working with RFDs
  - Creating technical documentation
  - User mentions RFDs, design docs
tools:
  - Bash (for rfd CLI)
  - Read (for reviewing RFDs)
  - Grep (for searching RFDs)
```

**Phase 2**: Create SKILL.md with workflows for create, list, show, status, update

**Phase 3**: Test by asking to "create a new RFD"

**Phase 4**: Document in skills/README.md

**Total Time**: ~1.5 hours

### Example 2: Complex Skill (api-endpoint-creator)

**Phase 1 Output**:
```yaml
name: api-endpoint-creator
purpose: Scaffold complete API endpoints with routes, handlers, tests
triggers:
  - Adding new API endpoints
  - Creating REST routes
  - User mentions API development
tools:
  - Read, Write, Edit (for code generation)
  - Bash (for running tests)
  - Grep, Glob (for finding patterns)
```

**Phase 2**: Create SKILL.md with multiple workflows (GET, POST, PUT, DELETE)

**Phase 3**: Test each endpoint type, refine error handling

**Phase 4**: Create comprehensive USAGE.md with examples

**Total Time**: ~2 hours

---

## Troubleshooting

### Common Issues

**Issue**: Skill not invoked automatically
**Solution**: Improve description with more specific triggers

**Issue**: Instructions too vague
**Solution**: Add concrete examples and expected outputs

**Issue**: Tool restrictions too limiting
**Solution**: Expand allowed-tools or split into separate Skills

**Issue**: Skill conflicts with others
**Solution**: Make descriptions more specific to reduce overlap

---

## Next Steps After Skill Creation

1. **Monitor Usage**: Track how often the Skill is invoked
2. **Gather Feedback**: Ask users about clarity and usefulness
3. **Iterate**: Refine based on real-world usage
4. **Share**: Consider contributing to Skills marketplace (future)
5. **Maintain**: Review quarterly and update as needed

---

**Remember**: Skills improve with usage. Start simple, deploy early, and iterate based on feedback!
