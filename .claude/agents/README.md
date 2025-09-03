# Claude Code Agents and Sub-Agents Guide

## Understanding Claude Code Agents

### What Are Agents?

In Claude Code, agents are specialized AI personas with specific expertise,
context, and responsibilities. They can:

- Focus on specific domains (design, security, performance)
- Maintain their own context and memory
- Collaborate with other agents
- Be invoked for specific tasks

### Agent vs Sub-Agent

- **Primary Agent**: The main Claude Code instance you interact with
- **Sub-Agents**: Specialized agents called by the primary agent for specific
  tasks
- **Parallel Agents**: Multiple Claude instances working simultaneously
  (multiple terminals)

## How Agents Work in Claude Code

### 1. Delegation Pattern

```bash
# Terminal 1: Primary Agent (Orchestrator)
claude-code --role orchestrator
# "Implement the authentication tool"
# This agent breaks down the task and delegates

# Terminal 2: Design Agent
claude-code --role design-reviewer
# Reviews architecture and design decisions

# Terminal 3: Code Agent
claude-code --role rust-expert
# Implements the actual code
```

### 2. Sequential Pattern

```bash
# Same terminal, different phases
claude-code --role designer    # First: Design the solution
claude-code --role implementer  # Then: Write the code
claude-code --role reviewer     # Finally: Review everything
```

### 3. Collaborative Pattern (Recommended for Tapestry)

```bash
# Create task for multiple agents
echo "Implement OAuth tool" > .claude/tasks/current-task.md

# Each agent reads the task and contributes
claude-code --agent design-reviewer --task review-design
claude-code --agent rust-expert --task implement
claude-code --agent security-auditor --task audit
```

## Tapestry Agent Architecture

### Agent Hierarchy

```
Primary Agent (You interact with this)
├── Design Reviewer Agent
│   ├── RFC Validator
│   ├── Architecture Checker
│   └── API Design Reviewer
├── Rust Code Reviewer Agent
│   ├── Safety Checker
│   ├── Performance Analyzer
│   └── Idiom Enforcer
├── Test Writer Agent
│   ├── Unit Test Generator
│   ├── Integration Test Designer
│   └── Property Test Creator
└── Documentation Agent
    ├── API Doc Writer
    ├── README Updater
    └── RFC Writer
```

## Creating Specialized Agents for Tapestry

### Method 1: Agent Configuration Files

Create `.claude/agents/` directory with agent definitions:

```markdown
# .claude/agents/design-reviewer.md

Role: Expert Design Reviewer Expertise: Software architecture, API design,
system design Focus: Reviewing designs for Tapestry MCP tools

Your responsibilities:

1. Validate hexagonal architecture compliance
2. Ensure proper separation of concerns
3. Review API contracts and interfaces
4. Check for design patterns and anti-patterns
5. Verify scalability considerations

When reviewing, always check:

- [ ] Dependencies flow inward only
- [ ] Domain logic has no external dependencies
- [ ] Ports clearly define contracts
- [ ] Design supports testing
- [ ] Migration path to microservices is clear
```

### Method 2: Agent Invocation Commands

```markdown
# .claude/commands/invoke-design-reviewer.md

Role: Design Review Agent

Review the current design with these criteria:

## Architecture Compliance

- Hexagonal architecture properly implemented?
- Clear separation between layers?
- Dependencies flowing correctly?

## API Design (Stripe Standards)

- Would this work with "seven lines of code"?
- Backward compatibility maintained?
- Clear and predictable behavior?

## Scalability (Uber/Netflix Patterns)

- Can this handle 10x growth?
- Clear service boundaries?
- Performance implications understood?

## Simplicity (Anthropic Principle)

- Is this the simplest solution?
- Are we building a bicycle or a spaceship?

Provide feedback as:

1. ✅ What's good
2. ⚠️ Concerns
3. ❌ Must fix
4. 💡 Suggestions
```

### Method 3: Inline Agent Switching

```markdown
# In your conversation with Claude Code

You: @design-reviewer Please review this architecture: [Your design]

You: @rust-expert Now implement this with best practices

You: @test-writer Create comprehensive tests for this
```

## Implementing Agent Workflow for Tapestry

### Step 1: Create Agent Directory Structure

```bash
.claude/
├── agents/
│   ├── README.md           # This file
│   ├── design-reviewer.md  # Design review agent
│   ├── rust-expert.md      # Rust code reviewer
│   ├── test-writer.md      # Test specialist
│   └── security-auditor.md # Security reviewer
├── workflows/
│   ├── new-tool.md        # Multi-agent workflow for new tools
│   └── code-review.md     # Review workflow
└── tasks/
    └── current/           # Current tasks for agents
```

### Step 2: Define the Design Reviewer Agent

```markdown
# .claude/agents/design-reviewer.md

## Identity

- Name: Tapestry Design Reviewer
- Role: Expert system architect and API designer
- Experience: 10+ years with distributed systems, microservices, and API design

## Expertise

- Hexagonal/Clean/DDD architectures
- API design (REST, GraphQL, gRPC)
- System design and scalability
- Design patterns and anti-patterns
- S-tier company practices (Stripe, Google, Uber, Netflix)

## Knowledge Base

- Study: .claude/context/architecture.md
- Reference: .claude/knowledge/decisions/
- Follow: .claude/instructions.md principles

## Review Process

### 1. Architecture Review
```

- [ ] Hexagonal boundaries respected?
- [ ] Domain logic pure?
- [ ] Ports well-defined?
- [ ] Adapters properly isolated?

```

### 2. API Design Review
```

- [ ] Follows REST/RPC principles?
- [ ] Versioning strategy clear?
- [ ] Error responses actionable?
- [ ] Documentation complete?

```

### 3. Scalability Review
```

- [ ] Performance implications considered?
- [ ] Resource usage acceptable?
- [ ] Concurrency handled properly?
- [ ] Migration path clear?

````

## Output Format

Provide review as:

```markdown
# Design Review for [Component]

## Summary
[One paragraph overview]

## Strengths ✅
- [What's done well]

## Critical Issues ❌
- [Must fix before proceeding]

## Recommendations ⚠️
- [Should consider changing]

## Suggestions 💡
- [Nice to have improvements]

## Verdict
[ ] Approved
[ ] Approved with changes
[ ] Needs revision
````

````

### Step 3: Define the Rust Expert Agent

```markdown
# .claude/agents/rust-expert.md

## Identity
- Name: Tapestry Rust Expert
- Role: Senior Rust developer and code reviewer
- Experience: Deep knowledge of Rust idioms, async programming, and best practices

## Expertise
- Rust ownership and borrowing
- Async/await with Tokio
- Error handling patterns
- Performance optimization
- Safe abstractions
- Testing strategies

## Knowledge Base
- Study: .claude/context/team-conventions.md
- Follow: Rust API Guidelines
- Reference: .claude/knowledge/patterns/

## Code Review Checklist

### Safety
- [ ] No `unsafe` without justification
- [ ] No `unwrap()` in production code
- [ ] All errors handled with `?` or `match`
- [ ] No data races possible

### Idioms
- [ ] Uses iterators instead of loops where appropriate
- [ ] Leverages pattern matching
- [ ] Proper use of `Option` and `Result`
- [ ] Follows Rust naming conventions

### Performance
- [ ] No unnecessary allocations
- [ ] Uses `&str` vs `String` appropriately
- [ ] Async used for I/O operations
- [ ] Zero-copy where possible

### Architecture
- [ ] Follows hexagonal pattern
- [ ] Dependencies injected properly
- [ ] Traits used for abstraction
- [ ] Modules properly organized

## Review Output

```rust
// SAFETY: ✅ No unsafe code used
// IDIOMS: ⚠️ Consider using iterator here
// PERF: ✅ Efficient implementation
// ARCH: ✅ Follows hexagonal pattern

// Specific feedback on code...
````

````

### Step 4: Create Multi-Agent Workflow

```markdown
# .claude/workflows/new-tool.md

## Workflow: Create New MCP Tool

### Phase 1: Design (Design Reviewer Agent)
1. Review requirements
2. Create architecture design
3. Define API contracts
4. Document in RFC

### Phase 2: Implementation (Rust Expert Agent)
1. Implement domain logic
2. Create ports
3. Build adapters
4. Follow conventions

### Phase 3: Testing (Test Writer Agent)
1. Write unit tests for domain
2. Create integration tests
3. Add property-based tests
4. Verify coverage

### Phase 4: Review (All Agents)
- Design Reviewer: Check architecture
- Rust Expert: Review code quality
- Security Auditor: Check vulnerabilities
- Documentation Agent: Verify docs

### Phase 5: Integration (Primary Agent)
1. Integrate all feedback
2. Run final tests
3. Update documentation
4. Prepare for merge
````

## Practical Usage Examples

### Example 1: Two-Terminal Review

```bash
# Terminal 1: Implementation
You: "Implement the OAuth tool following our patterns"
Claude: [Implements the tool]

# Terminal 2: Review
You: "As a Rust expert, review the OAuth tool implementation in Terminal 1"
Claude: [Provides detailed code review]
```

### Example 2: Sequential Agents

```bash
You: "Act as a design reviewer. Review this design: [paste design]"
Claude: [Provides design review]

You: "Now act as a Rust expert. How would you implement this?"
Claude: [Switches context, implements with Rust best practices]

You: "Finally, as a test specialist, what tests would this need?"
Claude: [Switches again, creates comprehensive test plan]
```

### Example 3: Collaborative Document

```markdown
# .claude/tasks/implement-oauth.md

## Task: Implement OAuth Tool

### Design Review Notes (by Design Agent)

- Architecture looks good
- Consider adding refresh token support
- API should follow OAuth 2.0 spec

### Implementation Notes (by Rust Agent)

- Used `oauth2` crate for standard compliance
- Implemented with async/await
- Added proper error handling

### Test Plan (by Test Agent)

- Unit tests for token validation
- Integration tests with mock provider
- Property tests for state machine

### Security Notes (by Security Agent)

- Tokens properly encrypted
- No secrets in logs
- PKCE implemented
```

## Best Practices for Using Agents

### 1. Clear Role Definition

Each agent should have a single, clear focus. Don't make agents too broad.

### 2. Shared Context

Use `.claude/agents/shared-context.md` for information all agents need.

### 3. Agent Handoff

```markdown
## Handoff from Design to Implementation

Design Review Complete ✅

- Architecture approved
- API contracts defined
- RFC documented

Ready for Implementation:

- [ ] Domain logic
- [ ] Ports
- [ ] Adapters
- [ ] Tests
```

### 4. Feedback Loops

```markdown
Implementation → Review → Revision → Review → Approval
```

### 5. Documentation

Each agent should document their decisions and reasoning.

## Advanced Techniques

### Parallel Processing

```bash
# Three agents work simultaneously on different aspects
Agent 1: Works on domain logic
Agent 2: Designs the API
Agent 3: Writes documentation

# Then synchronize
Primary Agent: Integrates all work
```

### Agent Specialization by Layer

```markdown
Domain Agent: Only works in src/domain/ Application Agent: Only works in
src/application/ Infrastructure Agent: Only works in src/infrastructure/
```

### Agent Memory

```markdown
# .claude/agents/memory/design-reviewer-memory.md

## Previous Reviews

- OAuth Tool: Approved with minor changes
- File Parser: Needed architecture revision
- Logger Tool: Good separation of concerns

## Common Issues Found

- Dependencies flowing wrong direction: 3 times
- Missing error handling: 5 times
- API not following REST principles: 2 times

## Patterns That Work

- Builder pattern for complex configs
- Factory pattern for tool creation
- Strategy pattern for multiple implementations
```

## Integration with Tapestry Development

### For Every New Tool

1. **Design Phase**: Design Reviewer agent creates/reviews RFC
2. **Implementation Phase**: Rust Expert implements following patterns
3. **Test Phase**: Test Writer creates comprehensive tests
4. **Review Phase**: All agents review their domains
5. **Documentation Phase**: Doc agent ensures everything is documented

### Commands to Add

```markdown
# .claude/commands/review-design.md

Invoke the design reviewer agent to review the current design

# .claude/commands/review-code.md

Invoke the Rust expert agent to review the current code

# .claude/commands/review-security.md

Invoke the security auditor to check for vulnerabilities
```

## Measuring Agent Effectiveness

Track metrics for each agent:

- Design Reviewer: Issues caught before implementation
- Rust Expert: Code quality improvements
- Test Writer: Coverage achieved
- Security Auditor: Vulnerabilities prevented

## Next Steps

1. Create the agent definition files in `.claude/agents/`
2. Test with a simple tool implementation
3. Refine agent definitions based on results
4. Document successful patterns
5. Share learnings with team

Remember: Agents are tools to improve quality and consistency. Start simple,
iterate based on what works!
