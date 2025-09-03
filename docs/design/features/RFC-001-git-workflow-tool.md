# RFC-001: Git Workflow Automation Tool

## Summary

Create an MCP tool that augments Claude Code's git capabilities with intelligent
workflow automation, conventional commits enforcement, and comprehensive change
tracking - providing a commitizen-like experience enhanced by AI understanding.

## Motivation

While Claude Code can make basic git commits, it lacks sophisticated git
workflow capabilities:

**Current Pain Points:**

- Claude often creates non-standard commit messages
- No automatic conventional commits format enforcement
- Limited understanding of what files should be staged together
- No semantic grouping of changes
- Can't generate detailed commit bodies with breaking changes, issues, etc.
- Lacks awareness of project-specific commit conventions

**What We Need:**

- Intelligent change detection and grouping
- Conventional commits with type, scope, and breaking change detection
- Interactive commit flow similar to commitizen
- AI-powered commit message generation that understands the actual changes
- Automatic staging of related files
- Pre-commit validation and checks

## Detailed Design

### Tool Interface

```rust
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait GitWorkflowTool {
    // Track and analyze current changes
    async fn analyze_changes(&self) -> Result<ChangeAnalysis>;

    // Smart staging - group related changes
    async fn suggest_staging(&self) -> Result<Vec<StagingGroup>>;

    // Interactive commit with conventional format
    async fn prepare_commit(&self, options: CommitOptions) -> Result<CommitPlan>;

    // Execute the commit with validation
    async fn execute_commit(&self, plan: CommitPlan) -> Result<CommitResult>;

    // Generate changelog entries
    async fn generate_changelog(&self, since: Option<String>) -> Result<Vec<ChangelogEntry>>;

    // Learn project conventions
    async fn analyze_commit_history(&self) -> Result<ProjectConventions>;
}
```

### Core Features

#### 1. Intelligent Change Analysis

Beyond simple `git status`:

- **Semantic grouping**: Identify related changes (e.g., component + tests +
  docs)
- **Change categorization**: Features vs fixes vs refactors
- **Dependency detection**: Find files that should be committed together
- **Breaking change detection**: Identify API changes, removed functions, etc.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAnalysis {
    pub groups: Vec<ChangeGroup>,
    pub suggested_commits: Vec<CommitSuggestion>,
    pub breaking_changes: Vec<BreakingChange>,
    pub dependencies: Vec<FileDependency>,
}

#[derive(Debug, Clone)]
pub struct ChangeGroup {
    pub name: String,
    pub files: Vec<PathBuf>,
    pub change_type: ChangeType,
    pub description: String,
}
```

#### 2. Conventional Commits Automation

Full commitizen-style workflow:

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CommitType {
    Feat,     // New feature
    Fix,      // Bug fix
    Docs,     // Documentation only
    Style,    // Formatting, semicolons, etc
    Refactor, // Code change that neither fixes nor adds
    Perf,     // Performance improvement
    Test,     // Adding tests
    Build,    // Build system changes
    Ci,       // CI configuration
    Chore,    // Other changes
    Revert,   // Revert previous commit
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitPlan {
    pub commit_type: CommitType,
    pub scope: Option<String>,        // e.g., "auth", "api", "ui"
    pub subject: String,               // Short description
    pub body: Option<String>,          // Detailed explanation
    pub breaking: Option<String>,      // BREAKING CHANGE: description
    pub issues: Vec<String>,           // Closes #123, Fixes #456
    pub co_authors: Vec<String>,       // Co-authored-by: Name <email>
}

impl CommitPlan {
    pub fn format_message(&self) -> String {
        // Format as conventional commit
        let mut message = format!(
            "{}{}: {}",
            self.commit_type.as_str(),
            self.scope.as_ref().map(|s| format!("({})", s)).unwrap_or_default(),
            self.subject
        );

        if let Some(body) = &self.body {
            message.push_str(&format!("\n\n{}", body));
        }

        if let Some(breaking) = &self.breaking {
            message.push_str(&format!("\n\nBREAKING CHANGE: {}", breaking));
        }

        // Add issue references and co-authors
        message
    }
}
```

#### 3. Interactive Commit Flow

Guide AI and humans through proper commits:

1. **Analyze changes**: Group and categorize all changes
2. **Select commit type**: Based on actual changes
3. **Determine scope**: From file paths and previous commits
4. **Generate subject**: AI-powered, following conventions
5. **Create body**: Explain why, not what
6. **Detect breaking**: Scan for API/interface changes
7. **Link issues**: Extract from code comments or branch names
8. **Validate**: Check message format and length
9. **Execute**: Make the commit with all metadata

#### 4. Smart Staging

Intelligent file grouping:

```rust
#[derive(Debug, Clone)]
pub struct StagingGroup {
    pub name: String,
    pub files: Vec<PathBuf>,
    pub rationale: String,
    pub suggested_commit_type: CommitType,
    pub suggested_scope: Option<String>,
}

impl StagingGroup {
    pub fn from_analysis(changes: &[FileChange]) -> Vec<Self> {
        // Group related files intelligently
        // - Test files with their implementation
        // - Documentation with related code
        // - Config changes together
        // - Separate features from fixes
        vec![]
    }
}
```

#### 5. Project Convention Learning

Analyze existing commits to learn patterns:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConventions {
    pub common_scopes: Vec<String>,
    pub commit_style: CommitStyle,
    pub average_commit_size: usize,
    pub branch_patterns: Vec<String>,
    pub custom_types: Vec<String>,
}

impl ProjectConventions {
    pub async fn learn_from_history(repo: &GitRepository) -> Result<Self> {
        // Analyze commit history to detect patterns
        // Extract common scopes, message formats, etc.
        Ok(Self::default())
    }
}
```

### Implementation Architecture

Following hexagonal architecture:

```rust
// Domain Layer - Pure git logic
pub struct GitWorkflow {
    analyzer: ChangeAnalyzer,
    commit_builder: CommitBuilder,
    convention_learner: ConventionLearner,
}

impl GitWorkflow {
    pub fn analyze_changes(&self, changes: Vec<FileChange>) -> ChangeAnalysis {
        // Pure logic for analyzing changes
        self.analyzer.analyze(changes)
    }

    pub fn build_commit(&self, analysis: ChangeAnalysis) -> CommitPlan {
        // Pure logic for building commit plan
        self.commit_builder.build(analysis)
    }
}

// Port - Interface
#[async_trait]
pub trait GitWorkflowPort {
    async fn get_changes(&self) -> Result<Vec<FileChange>>;
    async fn stage_files(&self, files: Vec<PathBuf>) -> Result<()>;
    async fn commit(&self, plan: CommitPlan) -> Result<CommitId>;
}

// MCP Adapter
pub struct GitWorkflowMcpAdapter {
    workflow: GitWorkflow,
    git_repo: GitRepository,
}

#[rmcp::tool(
    name = "git-workflow",
    description = "Intelligent git workflow automation with conventional commits"
)]
impl Tool for GitWorkflowMcpAdapter {
    async fn execute(&self, params: Value) -> Result<Value> {
        // Parse params and route to appropriate method
        // Return JSON response for MCP protocol
        Ok(json!({}))
    }
}
```

### Example Interactions

#### Scenario 1: Feature Addition

```
User: "I've just added a new authentication system"
Tool analyzes:
  - New files: auth/*.rs, tests/auth_test.rs
  - Modified: main.rs, Cargo.toml
  - Suggests: feat(auth): add JWT-based authentication system

Tool generates:
  feat(auth): add JWT-based authentication system

  - Implement JWT token generation and validation
  - Add login/logout endpoints
  - Include refresh token mechanism
  - Add comprehensive test coverage

  Closes #45
```

#### Scenario 2: Breaking Change

```
Tool detects: Public API function signature changed
Tool suggests:
  feat(api)!: change response format to follow JSON:API spec

  BREAKING CHANGE: API responses now follow JSON:API specification.
  Previous format: { data: [...] }
  New format: { data: [...], meta: {...}, links: {...} }

  Migration: Update clients to handle new response structure
```

## Alternatives Considered

### Alternative 1: Simple Commit Message Formatter

Just format messages without intelligence.

**Rejected because**: Doesn't add value beyond existing tools.

### Alternative 2: Full Git GUI

Build comprehensive git GUI with MCP.

**Rejected because**: Too complex, better tools exist (GitKraken, etc).

### Alternative 3: Basic Git Wrapper

Wrap git commands without workflow intelligence.

**Rejected because**: Claude Code already does this.

## Success Criteria

### Immediate (Launch)

- [ ] 100% conventional commits compliance
- [ ] AI understands which files to stage together
- [ ] Commit messages accurately describe changes
- [ ] No more "update files" commits

### Short-term (1 month)

- [ ] 50% reduction in time spent on commits
- [ ] Automatic changelog generation working
- [ ] Project conventions detected and followed
- [ ] Breaking changes never missed

### Long-term (3 months)

- [ ] Becomes essential part of workflow
- [ ] Other developers adopt the tool
- [ ] Integration with PR workflows
- [ ] Predictive staging suggestions

## Implementation Plan

### Week 1: Core Analysis

- Change detection and grouping
- File dependency analysis
- Basic conventional commit structure

### Week 2: Commit Building

- Interactive commit flow
- AI-powered message generation
- Validation and formatting

### Week 3: Intelligence Layer

- Convention learning from history
- Breaking change detection
- Smart staging suggestions

### Week 4: Polish & Integration

- MCP protocol implementation
- Claude Code integration
- Performance optimization
- Comprehensive testing

## Open Questions

1. Should we support custom commit types beyond conventional?
2. How to handle monorepo with multiple scopes?
3. Should the tool make commits directly or generate commands?
4. How to integrate with existing git hooks?
5. Should we support commit signing (GPG)?

## Future Extensions

- **PR Description Generation**: From commit history
- **Release Notes Automation**: From conventional commits
- **Git Hook Integration**: Pre-commit, commit-msg validation
- **Team Analytics**: Commit patterns and productivity metrics
- **Merge Conflict Prediction**: Based on staging patterns
- **Automatic Issue Linking**: From code TODOs and FIXMEs

---

**RFC Status**: PROPOSED  
**Created**: 2024-01-15  
**Author**: Ada  
**Discussion**: [Link to discussion]
