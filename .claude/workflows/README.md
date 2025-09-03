# Workflow Documentation

## What Are Workflows?

Workflows are structured, multi-phase processes that coordinate multiple agents
to accomplish complex tasks. They define:

- **Phases**: Sequential stages of development
- **Steps**: Specific tasks within each phase
- **Agents**: Which specialized agent handles each step
- **Gates**: Checkpoints that must pass before proceeding
- **Artifacts**: Documents and code produced at each step
- **Handoffs**: How work transfers between agents

## Available Workflows

### 1. [new-mcp-tool.md](new-mcp-tool.md)

**Purpose**: Create a new MCP tool from RFC to deployment  
**Duration**: 3-5 days  
**Agents**: Design Reviewer, Rust Expert, Test Writer, Security Auditor

### 2. code-review.md (Coming Soon)

**Purpose**: Comprehensive review of existing code  
**Duration**: 2-3 hours  
**Agents**: Rust Expert, Security Auditor

### 3. refactor-tool.md (Coming Soon)

**Purpose**: Refactor existing tool to new requirements  
**Duration**: 2-3 days  
**Agents**: Design Reviewer, Rust Expert, Test Writer

## How Workflows Work

### 1. Structured Execution

Each workflow defines:

```yaml
phase:
  step:
    agent: Which agent to use
    input: What the agent needs
    output: What the agent produces
    artifacts: Files created/updated
```

### 2. Phase Gates

Critical checkpoints between phases:

```yaml
gate:
  - Design must be approved
  - Code must compile
  - Tests must pass
  - No critical vulnerabilities
```

### 3. Artifact Management

All workflow artifacts stored in:

```
.claude/workflows/artifacts/{workflow-name}/
├── design-review.md
├── architecture.md
├── test-results.md
└── final-report.md
```

### 4. Agent Coordination

Workflows coordinate multiple agents:

```
Design Reviewer → Rust Expert → Test Writer → Security Auditor
      ↓               ↓              ↓              ↓
   Design      Implementation     Tests       Security
  Approved        Complete        Pass         Clear
```

## Using Workflows

### Starting a Workflow

1. Choose appropriate workflow document
2. Create artifact directory:
   ```bash
   mkdir -p .claude/workflows/artifacts/{tool-name}
   ```
3. Begin with Phase 1, Step 1
4. Use the provided prompts with each agent
5. Save artifacts as specified
6. Check gates before proceeding

### Following a Workflow

```markdown
# Example: Starting new-mcp-tool workflow

You: "I want to create a new MCP tool called 'git-workflow'. Let's follow the
new-mcp-tool workflow starting with Phase 1."

Claude: "I'll help you follow the new-mcp-tool workflow. Let me start with Phase
1: Design & Architecture as the Design Reviewer agent..."

You: [After Phase 1] "Phase 1 complete. Let's proceed to Phase 2:
Implementation"

Claude: "Moving to Phase 2. As the Rust Expert agent, I'll now implement the
domain logic based on the approved design..."
```

### Tracking Progress

Create a progress log:

```markdown
# .claude/workflows/artifacts/{tool-name}/progress.md

## Git Workflow Tool - Progress

- [x] Phase 1: Design & Architecture
  - [x] RFC Review - APPROVED
  - [x] Architecture Design - COMPLETE
  - [x] API Contract - DEFINED
- [ ] Phase 2: Implementation
  - [x] Domain Logic - COMPLETE
  - [ ] Port Implementation - IN PROGRESS
  - [ ] Adapter Implementation
  - [ ] Module Integration
```

## Workflow Benefits

### Consistency

- Same process for every tool
- Predictable quality outcomes
- Standardized artifacts

### Quality Gates

- Can't skip critical steps
- Issues caught early
- Systematic validation

### Knowledge Capture

- Decisions documented
- Rationale preserved
- Learning transferred

### Parallel Work

- Some phases can overlap
- Multiple agents can work simultaneously
- Faster overall completion

## Creating Custom Workflows

### Template Structure

```markdown
# Workflow: [Name]

## Workflow Metadata

- ID, Version, Purpose, Duration, Agents

## Prerequisites

- What must be ready before starting

## Phases

### Phase N: [Name]

#### Step N.N: [Task]

- Agent, Input, Output, Artifacts
- Prompt template
- Validation criteria

## Gates

- Checkpoints between phases

## Artifacts

- What gets produced and where

## Success Criteria

- How to know it's done right
```

### Best Practices

1. **Keep phases focused** - One major outcome per phase
2. **Define clear gates** - Objective pass/fail criteria
3. **Specify artifacts** - Where to save everything
4. **Include prompts** - Exact text to use with agents
5. **Allow flexibility** - Options for different scenarios

## Workflow vs Agent Guide

### Workflows

- **What**: Step-by-step process documentation
- **When**: Following a defined process
- **Who**: Coordinates multiple agents
- **Output**: Completed task with artifacts

### Agent Guide (WORKFLOW-GUIDE.md)

- **What**: Reference for using agents
- **When**: Ad-hoc agent invocation
- **Who**: Individual agent usage
- **Output**: Specific agent responses

## Tips for Success

1. **Follow the workflow strictly** first time through
2. **Save all artifacts** for future reference
3. **Document deviations** when you adapt the process
4. **Update workflows** based on lessons learned
5. **Use gates seriously** - don't skip quality checks

## Future Workflows

Planned workflows to be added:

- **emergency-fix.md**: Rapid bug fix process
- **performance-optimization.md**: Systematic performance improvement
- **documentation-update.md**: Comprehensive doc refresh
- **tool-deprecation.md**: Safely retiring old tools
- **integration-test.md**: End-to-end testing process

---

Workflows turn complex, multi-day processes into systematic, repeatable
successes.
