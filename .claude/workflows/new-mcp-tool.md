# Workflow: Create New MCP Tool

## Workflow Metadata

**Workflow ID**: new-mcp-tool  
**Version**: 1.0  
**Purpose**: Systematic process for creating a new MCP tool from RFC to
deployment  
**Duration**: 3-5 days  
**Agents Required**: Design Reviewer, Rust Expert, Test Writer, Security Auditor

## Prerequisites

- [ ] RFC written and in `docs/design/features/RFC-XXX-{tool-name}.md`
- [ ] Project structure initialized (`cargo init` completed)
- [ ] Dependencies updated in `Cargo.toml`
- [ ] Development environment ready

## Workflow Phases

### Phase 1: Design & Architecture (Day 1)

**Lead Agent**: Design Reviewer  
**Duration**: 4-6 hours

#### Step 1.1: RFC Review

```yaml
agent: Design Reviewer
input:
  - RFC document
  - Architecture guidelines (.claude/context/architecture.md)
output:
  - Design review report
  - Approved/Rejected status
  - Required changes list
artifacts:
  - .claude/workflows/artifacts/{tool-name}/design-review.md
```

**Prompt**:

```markdown
Act as the Design Reviewer agent. Review the RFC for {TOOL_NAME} located at
{RFC_PATH}. Validate against our hexagonal architecture and S-tier practices.
Save your review to .claude/workflows/artifacts/{tool-name}/design-review.md
```

#### Step 1.2: Architecture Design

```yaml
agent: Design Reviewer
input:
  - Approved RFC
  - Domain requirements
output:
  - Domain entities specification
  - Port interfaces (traits)
  - Adapter requirements
  - Data flow diagram
artifacts:
  - .claude/workflows/artifacts/{tool-name}/architecture.md
```

**Prompt**:

```markdown
Act as the Design Reviewer agent. Create detailed architecture for {TOOL_NAME}.
Define all domain entities, port interfaces, and adapter responsibilities.
Ensure hexagonal architecture with inward dependencies.
```

#### Step 1.3: API Contract Definition

```yaml
agent: Design Reviewer
input:
  - Architecture design
  - MCP specification
output:
  - Complete API specification
  - Error types and conditions
  - Usage examples
artifacts:
  - .claude/workflows/artifacts/{tool-name}/api-contract.md
```

**Gate**: Design must be approved before proceeding to Phase 2

---

### Phase 2: Implementation (Day 2)

**Lead Agent**: Rust Expert  
**Duration**: 6-8 hours

#### Step 2.1: Domain Implementation

```yaml
agent: Rust Expert
input:
  - Architecture design from Phase 1
  - Domain entity specifications
output:
  - mcp/{tool_name}/src/domain.rs
  - Pure business logic implementation
artifacts:
  - Source code file
```

**Prompt**:

```markdown
Act as the Rust Expert agent. Implement domain logic for {TOOL_NAME}. Use
architecture from .claude/workflows/artifacts/{tool-name}/architecture.md Create
mcp/{tool_name}/src/domain.rs with zero external dependencies.
```

#### Step 2.2: Port Implementation

```yaml
agent: Rust Expert
input:
  - Domain implementation
  - Port interface specifications
output:
  - mcp/{tool_name}/src/port.rs
  - Trait definitions with async-trait
artifacts:
  - Source code file
```

#### Step 2.3: Adapter Implementation

```yaml
agent: Rust Expert
input:
  - Port definitions
  - MCP protocol requirements
output:
  - mcp/{tool_name}/src/adapter.rs
  - MCP Tool trait implementation
artifacts:
  - Source code file
```

#### Step 2.4: Module Integration

```yaml
agent: Rust Expert
input:
  - All implementation files
output:
  - mcp/{tool_name}/src/lib.rs
  - Updated workspace Cargo.toml (auto-discovered via mcp/*)
artifacts:
  - Source code files
```

**Validation**: Run `cargo build` - must compile without errors

---

### Phase 3: Code Review (Day 2-3)

**Lead Agent**: Rust Expert  
**Duration**: 2-3 hours

#### Step 3.1: Safety Review

```yaml
agent: Rust Expert
input:
  - All source code from Phase 2
output:
  - Safety review report
  - Issues to fix (Critical/Important/Suggestions)
artifacts:
  - .claude/workflows/artifacts/{tool-name}/safety-review.md
```

#### Step 3.2: Performance Review

```yaml
agent: Rust Expert
input:
  - Source code
  - Performance targets (<100ms P50)
output:
  - Performance analysis
  - Optimization recommendations
artifacts:
  - .claude/workflows/artifacts/{tool-name}/performance-review.md
```

#### Step 3.3: Fix Critical Issues

```yaml
agent: Rust Expert
input:
  - Review reports
output:
  - Updated source code
  - Fixed issues checklist
```

**Gate**: All critical issues must be resolved

---

### Phase 4: Testing (Day 3)

**Lead Agent**: Test Writer  
**Duration**: 4-5 hours

#### Step 4.1: Test Planning

```yaml
agent: Test Writer
input:
  - Implementation code
  - Domain logic documentation
output:
  - Test plan document
  - Test case specifications
artifacts:
  - .claude/workflows/artifacts/{tool-name}/test-plan.md
```

#### Step 4.2: Unit Tests

```yaml
agent: Test Writer
input:
  - Domain implementation
  - Test plan
output:
  - Unit tests in domain.rs
  - Coverage report
artifacts:
  - Test code in source files
```

#### Step 4.3: Integration Tests

```yaml
agent: Test Writer
input:
  - Full implementation
  - MCP protocol specs
output:
  - tests/{tool_name}_integration.rs
  - Test helpers/fixtures
artifacts:
  - Integration test file
```

**Validation**: Run `cargo test` - all tests must pass

---

### Phase 5: Security Audit (Day 3-4)

**Lead Agent**: Security Auditor  
**Duration**: 2-3 hours

#### Step 5.1: Vulnerability Assessment

```yaml
agent: Security Auditor
input:
  - Complete source code
  - Dependencies list
output:
  - Security audit report
  - Vulnerability list with severity
artifacts:
  - .claude/workflows/artifacts/{tool-name}/security-audit.md
```

#### Step 5.2: Threat Modeling

```yaml
agent: Security Auditor
input:
  - Tool functionality
  - External interfaces
output:
  - Threat model document
  - Risk matrix
  - Mitigation strategies
artifacts:
  - .claude/workflows/artifacts/{tool-name}/threat-model.md
```

**Gate**: No high-severity vulnerabilities

---

### Phase 6: Final Integration (Day 4)

**Lead Agent**: Design Reviewer  
**Duration**: 2-3 hours

#### Step 6.1: Architecture Compliance

```yaml
agent: Design Reviewer
input:
  - Original RFC
  - Implementation code
  - Test results
output:
  - Compliance report
  - Final approval/rejection
artifacts:
  - .claude/workflows/artifacts/{tool-name}/final-review.md
```

#### Step 6.2: Documentation

```yaml
agent: Documentation Writer
input:
  - Source code
  - API contracts
  - Usage examples
output:
  - Updated README.md
  - API documentation
  - CHANGELOG.md entry
```

#### Step 6.3: Release Preparation

```yaml
agent: Primary
tasks:
  - Run final test suite
  - Run cargo fmt and cargo clippy
  - Create git commit
  - Update tool registry
```

---

## Workflow Artifacts

All artifacts are stored in:

```
.claude/workflows/artifacts/{tool-name}/
├── design-review.md
├── architecture.md
├── api-contract.md
├── safety-review.md
├── performance-review.md
├── test-plan.md
├── security-audit.md
├── threat-model.md
└── final-review.md
```

## Success Criteria

### Phase Gates

- [ ] Design approved by Design Reviewer
- [ ] Code compiles without warnings
- [ ] All critical issues fixed
- [ ] Test coverage > 80% for domain logic
- [ ] No high-severity vulnerabilities
- [ ] Architecture compliance verified
- [ ] Documentation complete

### Quality Metrics

- **Performance**: P50 < 100ms, P99 < 500ms
- **Memory**: < 10MB per tool instance
- **Test Coverage**: > 80% domain, > 60% overall
- **Code Quality**: Zero clippy warnings
- **Documentation**: All public APIs documented

## Rollback Plan

If issues discovered after integration:

1. Revert commits to last known good state
2. Document issue in `.claude/workflows/artifacts/{tool-name}/postmortem.md`
3. Create fix RFC if architectural changes needed
4. Re-enter workflow at appropriate phase

## Workflow Commands

### Start Workflow

```bash
# Initialize workflow for new tool
mkdir -p .claude/workflows/artifacts/{tool-name}
echo "Workflow started: $(date)" > .claude/workflows/artifacts/{tool-name}/workflow.log
```

### Check Status

```bash
# See current phase and completed steps
cat .claude/workflows/artifacts/{tool-name}/workflow.log
```

### Agent Handoff

```markdown
"Current phase complete. Handing off to [NEXT_AGENT]. Input artifacts available
at: .claude/workflows/artifacts/{tool-name}/ Next steps: [PHASE_NAME] -
[SPECIFIC_TASKS]"
```

## Parallel Workflows

Some phases can run in parallel:

```
Phase 2.1 (Domain) ──┐
                     ├──> Phase 2.3 (Adapter)
Phase 2.2 (Port) ────┘

Phase 4.2 (Unit Tests) ────┐
                           ├──> Phase 5 (Security)
Phase 4.3 (Integration) ───┘
```

## Troubleshooting

### Common Issues

**Design not approved**:

- Revise RFC based on feedback
- Re-enter at Phase 1.1

**Compilation errors**:

- Rust Expert fixes in Phase 2
- May need Design Reviewer input for architectural issues

**Test failures**:

- Return to Phase 2 for fixes
- Update tests if requirements changed

**Security vulnerabilities**:

- Critical: Must fix before proceeding
- Medium: Fix in next iteration
- Low: Document for future work

## Workflow Variations

### Fast Track (1-2 days)

- Skip formal RFC review if pre-approved
- Combine review phases
- Minimal documentation

### Extended (5-7 days)

- Add performance benchmarking phase
- Include user acceptance testing
- Create video documentation
- Add integration with other tools

## Notes

- This workflow assumes a single developer with AI assistance
- For team development, add code review gates between phases
- Adjust timelines based on tool complexity
- Keep all artifacts for future reference and learning

---

**Workflow Version**: 1.0  
**Last Updated**: 2024-01-15  
**Next Review**: After 5 tool implementations
