# .claude/ Directory Guide

## Purpose

This directory contains all context, instructions, and tooling for AI-assisted
development with Claude Code. It encodes our engineering principles, patterns,
and accumulated knowledge to help Claude Code be an effective member of the
development team.

## Directory Structure

```
.claude/
├── README.md                # This file
├── instructions.md          # Core principles (rarely changes)
├── context/                 # Current project state
│   ├── architecture.md      # System design
│   ├── project-state.md     # Sprint progress
│   └── team-conventions.md  # Coding standards
├── commands/                # Reusable Claude prompts
├── templates/               # Code and doc templates
├── knowledge/               # Accumulated learnings
│   ├── decisions/           # ADRs
│   ├── patterns/            # What works
│   └── anti-patterns/       # What doesn't
└── sessions/                # Daily working context
    └── current.md           # Today's focus
```

## Quick Start

### For New Team Members

1. **Read Core Principles**: Start with `instructions.md`
2. **Understand Architecture**: Review `context/architecture.md`
3. **Learn Conventions**: Study `context/team-conventions.md`
4. **Check Current State**: See `context/project-state.md`

### For Daily Development

1. **Morning Setup**:

   ```bash
   # Update your session file
   cp .claude/sessions/template.md .claude/sessions/current.md
   # Edit with today's goals
   ```

2. **Use Commands**:

   ```
   /create-mcp-tool "tool-name" "description"
   /write-rfc "feature" "problem to solve"
   ```

3. **Evening Wrap-up**:
   - Update `project-state.md` with progress
   - Move learnings to `knowledge/`
   - Archive session notes

## File Purposes

### Core Files (Stable)

**instructions.md**

- S-tier company principles
- Architectural guidelines
- Security requirements
- Never changes without RFC

### Context Files (Weekly Updates)

**architecture.md**

- Current system design
- Technology choices
- Open questions

**project-state.md**

- Sprint progress
- Known issues
- Technical debt

**team-conventions.md**

- Coding standards
- Git workflow
- Review process

### Working Files (Daily Updates)

**sessions/current.md**

- Today's focus
- Active decisions
- Blockers

### Knowledge Base (Continuous Growth)

**decisions/**

- Architecture Decision Records
- Major technical choices
- Historical context

**patterns/**

- Successful approaches
- Reusable solutions
- Best practices

**anti-patterns/**

- Failed approaches
- Common mistakes
- What to avoid

## How Claude Code Uses This

### Priority Order

Claude Code reads files in this priority:

1. `instructions.md` - Always applies
2. `context/*.md` - Current project context
3. `sessions/current.md` - Today's specific focus
4. `commands/` - When you use slash commands
5. `knowledge/` - For reference and learning

### Command Usage

Commands are triggered with slash notation:

```
/create-mcp-tool "auth-handler" "handles OAuth authentication"
```

Claude Code will:

1. Find the command file
2. Read the instructions
3. Apply parameters
4. Execute the workflow
5. Follow all conventions

### Template Application

Templates ensure consistency:

- RFC template for all proposals
- Tool template for new MCP tools
- Test template for test files

## Maintenance Schedule

### Daily

- [ ] Update `sessions/current.md`
- [ ] Note any new patterns discovered
- [ ] Flag any anti-patterns encountered

### Weekly

- [ ] Update `project-state.md`
- [ ] Review and archive session notes
- [ ] Update technical debt list
- [ ] Add new decisions to ADRs

### Per Feature

- [ ] Write RFC using template
- [ ] Update architecture if needed
- [ ] Document patterns learned
- [ ] Update commands if needed

### Monthly

- [ ] Review instructions.md for updates
- [ ] Clean up old session files
- [ ] Consolidate patterns
- [ ] Update templates with improvements

## Best Practices

### 1. Keep Instructions Stable

Core principles shouldn't change often. Project-specific details go in context.

### 2. Document Learnings

Every bug, every success, every pattern should be captured in `knowledge/`.

### 3. Use Commands for Repetitive Tasks

If you do something twice, create a command for it.

### 4. Templates Enforce Standards

Always start from templates for consistency.

### 5. Session Notes are Temporary

Important decisions move to context or knowledge. Sessions are for working
memory.

## Common Workflows

### Starting a New Feature

1. Write RFC:

   ```
   /write-rfc "Feature Name" "Problem Statement"
   ```

2. Update context:

   - Add to `project-state.md`
   - Update `architecture.md` if needed

3. Create tools:
   ```
   /create-mcp-tool "tool-name" "description"
   ```

### Debugging an Issue

1. Check anti-patterns:

   ```bash
   grep -r "similar-issue" .claude/knowledge/anti-patterns/
   ```

2. Update session:

   - Document issue in `current.md`
   - Note solution approach

3. Capture learning:
   - Add pattern or anti-pattern
   - Update relevant documentation

### Code Review

1. Check conventions:

   - Reference `team-conventions.md`
   - Verify against patterns

2. Update based on feedback:
   - Add new patterns discovered
   - Document any exceptions

## Integration with Git

### What to Commit

✅ **Always Commit**:

- All files in `.claude/` except `sessions/`
- Templates and commands
- Knowledge and decisions

⚠️ **Sometimes Commit**:

- `sessions/current.md` if it contains important context

❌ **Never Commit**:

- Old session files
- Personal notes
- Credentials or secrets

### Git Workflow

```bash
# After updating .claude/ files
git add .claude/
git commit -m "docs(claude): update project state and patterns"

# Exclude sessions from commits
echo ".claude/sessions/archive/" >> .gitignore
```

## Tips for Claude Code

### Make Instructions Clear

Claude Code takes instructions literally. Be specific and include examples.

### Provide Context

The more context in these files, the better Claude Code performs.

### Update Regularly

Stale context leads to incorrect assumptions. Keep files current.

### Use Examples

Claude Code learns from examples. Include them in patterns and commands.

## Troubleshooting

### Claude Code isn't following conventions

- Check `instructions.md` is up to date
- Verify context files are current
- Explicitly reference the convention file

### Commands aren't working

- Ensure command file follows correct format
- Check parameter names match
- Verify command file is in correct location

### Inconsistent results

- Update `sessions/current.md` with clear focus
- Check for conflicting instructions
- Consolidate overlapping patterns

## Evolution

This structure grows with your project:

**Week 1**: Basic structure **Month 1**: Patterns emerging **Month 3**: Rich
knowledge base **Month 6**: Comprehensive system

The key is consistent maintenance and capturing learnings as you go.

---

Questions? Check the knowledge base or ask the team!
