# Command: Write RFC

> **⚠️ DEPRECATED**: Use `/create-rfd` instead
>
> This command is being replaced by `/create-rfd` which uses the RFD (Request for Discussion) format.
>
> **New command**: `/create-rfd "Title" "Author <email@example.com>"`
>
> **Why?** RFDs provide:
> - Structured YAML frontmatter (agent-parseable)
> - CLI tool for management (`cargo run --bin rfd`)
> - State machine for lifecycle tracking
> - Better agent-friendliness
>
> **Documentation**: `.claude/commands/create-rfd.md`
>
> This command is kept for reference during the transition period.

---

**Name**: write-rfc
**Description**: Generate an RFC for a new feature or significant change
**Parameters**: `$FEATURE_NAME`, `$PROBLEM_STATEMENT`
**Example**: `/write-rfc "Tool Registry" "Need a way to discover and manage all MCP tools"`

---

## Instructions

Write RFC-{NEXT_NUMBER} for $FEATURE_NAME to address: $PROBLEM_STATEMENT

## Process

### Step 1: Determine RFC Number

Check `docs/design/features/` for the highest RFC number and increment by 1.

### Step 2: Gather Context

Before writing, understand:

- Why is this needed now?
- What's the current pain point?
- Who are the stakeholders?
- What's the desired outcome?

### Step 3: Generate RFC

Use the template from `.claude/templates/rfc-template.md` and fill in each
section:

1. **Executive Summary**: Write this LAST. One paragraph that anyone can
   understand.

2. **Context and Problem**:

   - Start with the user's perspective
   - Explain current limitations
   - Why solving this now is important

3. **Goals & Non-Goals**:

   - Be specific about what you WILL do
   - Be explicit about what you WON'T do
   - This prevents scope creep

4. **Proposed Solution**:

   - Start high-level (architecture diagram)
   - Then get specific (implementation details)
   - Include code examples

5. **Alternatives**:

   - Always consider at least 2 alternatives
   - "Do nothing" is often alternative #1
   - Explain WHY your solution is better

6. **Trade-offs**:
   - Every decision has trade-offs
   - Be honest about downsides
   - Explain why trade-offs are acceptable

### Step 4: Focus Areas for Tapestry

For Tapestry RFCs, always address:

#### Architecture Fit

- How does this fit with hexagonal architecture?
- Which layer(s) does this affect?
- Does this maintain our monolithic approach?

#### Tool Integration

- How do tools interact with this feature?
- Does this affect tool discovery/registration?
- Impact on existing tools?

#### Performance

- Expected latency (target: <100ms P50)
- Memory usage per tool (<10MB)
- Concurrent operation handling

#### Developer Experience

- How easy is it to use?
- Can Claude Code understand it?
- Documentation needs?

### Step 5: Include Tapestry-Specific Sections

Add these sections for all Tapestry RFCs:

```markdown
## Integration with Existing Tools

How this affects current tools:

- Tool A: {impact}
- Tool B: {impact}
- Registry: {changes needed}

## Claude Code Considerations

How AI assistants will interact with this:

- API clarity for AI understanding
- Error messages that guide AI
- Self-documenting interfaces

## Monolith to Microservices Path

If we split later:

- How does this design support extraction?
- What would be the service boundaries?
- Data separation strategy?
```

### Step 6: Quality Checklist

Before finalizing, ensure:

- [ ] Problem is clear to a non-technical reader
- [ ] Solution directly addresses the problem
- [ ] Code examples compile (test them!)
- [ ] Diagrams are clear and labeled
- [ ] Trade-offs are honest
- [ ] Timeline is realistic
- [ ] Success metrics are measurable
- [ ] Follows Stripe's principle: "simple enough for junior engineers"

### Step 7: Common Patterns for Tapestry RFCs

#### For New Tools

Focus on:

- Tool's purpose and use cases
- Integration with registry
- Domain model design
- MCP protocol mapping

#### For Infrastructure Changes

Focus on:

- Impact on all tools
- Migration strategy
- Backward compatibility
- Performance implications

#### For Process Changes

Focus on:

- Current pain points
- Expected improvements
- Team impact
- Measurement strategy

### Template Variables to Replace

When using the template, replace:

- `{NUMBER}`: Next RFC number (e.g., 001, 002)
- `{TITLE}`: Clear, descriptive title
- `{DATE}`: Today's date (YYYY-MM-DD)
- `{AUTHOR}`: Your name or "Tapestry Team"
- `{ISSUE_NUMBER}`: GitHub issue if exists

### Example Opening for Tapestry RFC

```markdown
# RFC-001: Unified Tool Registry for MCP Tools

**Date**: 2024-01-15  
**Author**: Tapestry Team  
**Status**: Draft  
**Implementation**: TBD  
**Tracking Issue**: #1

## Executive Summary

We need a centralized registry to discover, manage, and invoke MCP tools within
Tapestry. This RFC proposes a registry system that allows tools to
self-register, provides metadata about capabilities, and enables dynamic tool
discovery at runtime.

## Context and Problem Statement

As we build more MCP tools within Tapestry, we face several challenges:

- No standard way to discover available tools
- Each tool has different initialization requirements
- Claude Code needs to understand what tools are available
- No central place to manage tool lifecycle

Without a registry, users must manually track available tools, and our system
cannot dynamically adapt to new tools being added.
```

### Output Location

Save the RFC to: `docs/design/features/RFC-{NUMBER}-{kebab-case-title}.md`

Example: `docs/design/features/RFC-001-tool-registry.md`

### After Writing

1. Create a tracking issue on GitHub
2. Share with team for feedback
3. Update status as it progresses through review
4. Link implementation PR when started

Remember: RFCs are living documents. Update them as you learn more!
