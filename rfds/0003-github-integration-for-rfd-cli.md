---
title: GitHub Integration for RFD CLI
authors:
  - adanoelle <ada@tapestrylabs.org>
state: draft
created: 2025-10-23T00:00:00Z
updated: 2025-10-27T20:20:00Z
tags:
  - rfd-cli
  - github
  - integration
  - hexagonal-architecture
---

# RFD 003: GitHub Integration for RFD CLI

## Summary

Add GitHub integration to the RFD CLI tool, enabling automated issue creation,
state synchronization, and discussion linking between RFD documents and GitHub
issues. This integration uses hexagonal architecture with a port interface and
direct GitHub API adapter, designed for both human and agent workflows.

**Key Features:**

- Create GitHub issues for RFD discussions
- Sync RFD state changes to issue labels/comments
- Link existing issues to RFDs
- Support both manual and automated workflows
- Agent-friendly with JSON output

**Architecture Decision:** Use direct GitHub REST API (not `gh` CLI) with port
interface for testability.

## Motivation

### Current Pain Points

1. **Manual Discussion Management**: Creating GitHub issues for RFD discussions
   requires manual steps:

   - Create RFD document
   - Manually create GitHub issue
   - Manually copy RFD link to issue
   - Manually update RFD with issue link
   - No synchronization when RFD state changes

2. **Inconsistent Tracking**: RFD metadata has a `discussion` field, but it's
   rarely populated consistently:

   - Hard to find which RFDs have active discussions
   - No connection between RFD state and issue state
   - Labels don't reflect RFD lifecycle

3. **Agent Workflow Gaps**: Current process is not agent-friendly:

   - Multiple manual steps
   - No structured commands
   - Can't automate with Skills
   - No idempotent operations

4. **Missing Collaboration**: GitHub is where teams collaborate, but RFDs live
   in markdown:
   - Comments on RFDs require PR reviews
   - No notification when RFD state changes
   - Hard to track who's interested in which RFDs

### Why GitHub Integration?

- **Where Developers Are**: GitHub is the primary collaboration platform
- **Notifications**: Issue watchers get updates automatically
- **Discussion Threading**: Better than markdown comments
- **Labels/Milestones**: Project management integration
- **Search**: GitHub's search across issues is excellent
- **Agent Friendly**: Structured API for automation

### Why Now?

1. **RFD CLI is production-ready**: Foundation is stable (RFC-002 implemented)
2. **Skills paradigm validated**: Ready to build orchestration workflows
3. **Real usage patterns**: Dogfooding Tapestry docs has shown gaps
4. **Natural extension**: `discussion` field already exists in metadata

### Success Scenario

```bash
# Alice creates an RFD
rfd create --title "Add metrics to API" --author "Alice <alice@example.com>"
# → Created RFD 004

# Alice creates GitHub issue for discussion (one command!)
rfd github create-issue 004
# → Created issue #42: RFD 004: Add metrics to API
# → Updated RFD 004 with discussion link
# → Added labels: rfd:draft

# Alice updates RFD status after review
rfd status 004 --set accepted
# → Updated RFD 004 status: draft → accepted
# → Synced to GitHub issue #42 (updated labels, added comment)

# Team members watch issue #42, get notified of changes
# All discussion happens on GitHub, RFD is source of truth
```

## Goals

1. **Automate Issue Creation**: Single command to create GitHub issue for RFD
2. **Bidirectional Linking**: RFD ↔ Issue connections in both directions
3. **State Synchronization**: RFD state changes reflected in GitHub
   labels/comments
4. **Agent-Friendly**: All operations work with Skills, JSON output, idempotent
5. **Testable**: Architecture supports mocking for tests (no real GitHub calls)
6. **Simple to Use**: Sensible defaults, optional auto-sync, clear errors

## Non-Goals

1. **Not Building `gh` CLI Alternative**: We're integrating, not replacing
   GitHub's CLI
2. **Not Syncing Issue → RFD**: One-way sync (RFD is source of truth)
3. **Not Supporting Multiple Repos**: Single repo per project (for now)
4. **Not Building Full GitHub Client**: Only issue operations we need
5. **Not Supporting Pull Requests**: Future extension (RFD-004)
6. **Not Real-time Webhooks**: Polling or manual sync only (MCP tool later)

## Proposed Solution

### Architecture Overview

**Three-Layer Design:**

```
┌─────────────────────────────────────────────────────────┐
│              Application Layer (Commands)               │
│                                                         │
│  rfd github create-issue    rfd github sync-status     │
│  rfd github link-issue      rfd status --sync-github   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│           Domain Layer (Port Interface)                 │
│                                                         │
│  trait GitHubPort {                                     │
│    async fn create_issue(...) -> Result<...>           │
│    async fn add_comment(...) -> Result<...>            │
│    async fn set_labels(...) -> Result<...>             │
│    async fn get_issue(...) -> Result<...>              │
│  }                                                      │
└───────────────┬─────────────────────┬───────────────────┘
                │                     │
                ▼                     ▼
    ┌───────────────────┐ ┌───────────────────────┐
    │  GitHubAdapter    │ │  MockGitHubAdapter    │
    │                   │ │                       │
    │  • REST API       │ │  • For tests          │
    │  • GITHUB_TOKEN   │ │  • No real API calls  │
    │  • Rate limiting  │ │  • Predictable data   │
    └───────────────────┘ └───────────────────────┘
```

### Why Direct API (Not `gh` CLI)?

**Decision: Use GitHub REST API directly**

**Reasoning:**

1. **gh CLI unavailable in Claude Code**: Confirmed not available in many agent
   environments
2. **More control**: Better error handling, rate limit visibility, parallel
   requests
3. **Simpler**: One less external dependency to install/manage
4. **Faster**: No shell process overhead (~10ms saved per call)
5. **Predictable**: API responses are structured JSON, CLI output can change
6. **YAGNI**: No clear benefit to supporting both CLI and API

**Port interface still valuable for:**

- Testing (mock adapter, no real API calls)
- Future adapters if needed (GitHub Enterprise, GraphQL)
- Clean architecture (domain doesn't depend on GitHub)

### Port Interface

**File: `cli/rfd/src/ports/github.rs`**

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Port for GitHub operations (hexagonal architecture boundary)
#[async_trait]
pub trait GitHubPort: Send + Sync {
    /// Create a GitHub issue
    async fn create_issue(
        &self,
        request: CreateIssueRequest,
    ) -> Result<GitHubIssue, GitHubError>;

    /// Get issue details
    async fn get_issue(&self, number: u32) -> Result<GitHubIssue, GitHubError>;

    /// Add comment to issue
    async fn add_comment(
        &self,
        number: u32,
        body: String,
    ) -> Result<GitHubComment, GitHubError>;

    /// Set issue labels (replaces all labels)
    async fn set_labels(
        &self,
        number: u32,
        labels: Vec<String>,
    ) -> Result<(), GitHubError>;

    /// Check rate limit status
    async fn rate_limit(&self) -> Result<RateLimit, GitHubError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueRequest {
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub number: u32,
    pub url: String,
    pub html_url: String,
    pub state: String,
    pub title: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubComment {
    pub id: u64,
    pub body: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub remaining: u32,
    pub reset_at: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GitHubError {
    #[error("Authentication failed: {message}\nSuggestion: {suggestion}")]
    Auth { message: String, suggestion: String },

    #[error("Rate limit exceeded. Resets at {reset_at}")]
    RateLimit { reset_at: String },

    #[error("Not found: {resource}")]
    NotFound { resource: String },

    #[error("GitHub API error: {message}")]
    Api { message: String, status_code: Option<u16> },
}
```

### GitHub API Adapter

**File: `cli/rfd/src/adapters/github_api.rs`**

```rust
use crate::ports::github::*;
use reqwest::{Client, StatusCode};

pub struct GitHubAdapter {
    client: Client,
    token: String,
    repo: String,
    base_url: String,
}

impl GitHubAdapter {
    pub fn new(repo: String) -> Result<Self, GitHubError> {
        let token = std::env::var("GITHUB_TOKEN").map_err(|_| {
            GitHubError::Auth {
                message: "GITHUB_TOKEN not set".to_string(),
                suggestion: "export GITHUB_TOKEN=ghp_xxx or create at https://github.com/settings/tokens".to_string(),
            }
        })?;

        let client = Client::builder()
            .user_agent("tapestry-rfd-cli")
            .build()
            .map_err(|e| GitHubError::Api {
                message: format!("Failed to create client: {}", e),
                status_code: None,
            })?;

        Ok(Self {
            client,
            token,
            repo,
            base_url: "https://api.github.com".to_string(),
        })
    }

    async fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, GitHubError> {
        // Implementation: Make HTTP request, handle errors, check rate limits
        // ...
    }
}

#[async_trait]
impl GitHubPort for GitHubAdapter {
    async fn create_issue(
        &self,
        request: CreateIssueRequest,
    ) -> Result<GitHubIssue, GitHubError> {
        let body = json!({
            "title": request.title,
            "body": request.body,
            "labels": request.labels,
        });

        let result = self.request(
            reqwest::Method::POST,
            &format!("/repos/{}/issues", self.repo),
            Some(body),
        ).await?;

        // Parse response into GitHubIssue
        // ...
    }

    // ... implement other methods
}
```

### Configuration

**File: `.rfd/config.toml` (additions)**

```toml
[github]
# Repository in "owner/repo" format
repo = "adanoelle/tapestry"

# Auto-create issue when creating RFD (default: false)
auto_create_issue = false

# Auto-sync status changes to GitHub (default: false)
auto_sync_status = false

# Label mapping for RFD states
[github.label_mapping]
draft = ["rfd:draft"]
review = ["rfd:review", "needs-review"]
accepted = ["rfd:accepted"]
implemented = ["rfd:implemented", "done"]
rejected = ["rfd:rejected"]
archived = ["rfd:archived"]
```

### Commands

**1. Create Issue for RFD**

```bash
rfd github create-issue <rfd-id> [--format json]

# Example
$ rfd github create-issue 003
✓ Created GitHub issue #42 for RFD 003
  URL: https://github.com/adanoelle/tapestry/issues/42
✓ Updated RFD 003 metadata with discussion link

# JSON output
$ rfd github create-issue 003 --format json
{
  "success": true,
  "rfd_id": "003",
  "issue": {
    "number": 42,
    "url": "https://github.com/adanoelle/tapestry/issues/42"
  }
}
```

**2. Sync Status to GitHub**

```bash
rfd github sync-status <rfd-id> [--format json]

# Example
$ rfd github sync-status 003
✓ Synced RFD 003 status to GitHub issue #42
  - Updated labels: rfd:draft → rfd:review
  - Added comment about status change

# Auto-sync on status change (if configured)
$ rfd status 003 --set accepted --sync-github
✓ Updated RFD 003 status: review → accepted
✓ Synced to GitHub issue #42
```

**3. Link Existing Issue**

```bash
rfd github link-issue <rfd-id> --issue <number> [--format json]

# Example
$ rfd github link-issue 003 --issue 42
✓ Linked RFD 003 to GitHub issue #42
✓ Updated RFD metadata
✓ Added comment to issue #42
```

### Issue Template

When creating a GitHub issue for an RFD:

```markdown
# RFD 003: GitHub Integration for RFD CLI

Discussion for
[RFD 003](https://github.com/adanoelle/tapestry/blob/main/rfds/0003-github-integration.md)

## Summary

Add GitHub integration to the RFD CLI tool, enabling automated issue creation...

## Quick Links

- **RFD Document**: [rfds/0003-github-integration.md](...)
- **State**: draft
- **Authors**: Ada <ada@tapestry.dev>

## Discussion

Please comment below with:

- Questions about the proposal
- Concerns or risks
- Alternative approaches
- Implementation suggestions

---

_This issue was created automatically by the RFD CLI tool._ _Updates to RFD
state will be synced here automatically._
```

### Error Handling

**Structured, Actionable Errors:**

```json
// Authentication error
{
  "error": "GITHUB_AUTH_FAILED",
  "message": "GITHUB_TOKEN environment variable not set",
  "suggestion": "Create a token at https://github.com/settings/tokens with 'repo' scope",
  "docs": "https://docs.tapestry.dev/rfd-cli/github-integration#authentication"
}

// Rate limit error
{
  "error": "GITHUB_RATE_LIMIT",
  "message": "API rate limit exceeded",
  "rate_limit": {
    "remaining": 0,
    "reset_at": "2025-10-23T15:30:00Z"
  },
  "suggestion": "Wait until 15:30 or use a different token"
}

// Issue already exists
{
  "error": "ISSUE_ALREADY_EXISTS",
  "message": "RFD 003 already has a discussion link",
  "current_issue": "https://github.com/org/repo/issues/42",
  "suggestion": "Use 'rfd github sync-status 003' to update existing issue"
}
```

## Implementation Plan

### Phase 1: Foundation (Week 1)

**Goals**: Port interface, API adapter, basic tests

- [ ] Create `ports/github.rs` with trait definition
- [ ] Implement `adapters/github_api.rs` (REST API)
- [ ] Implement `adapters/github_mock.rs` (testing)
- [ ] Add configuration to `.rfd/config.toml`
- [ ] Unit tests with mock adapter
- [ ] Integration tests with real GitHub (optional, gated)

**Dependencies:**

```toml
async-trait = "0.1"
tokio = { version = "1", features = ["rt", "macros"] }
reqwest = { version = "0.12", features = ["json"] }
thiserror = "1.0"
```

**Success Criteria:**

- Port trait compiles and is well-documented
- API adapter can create/get issues (tested against real GitHub)
- Mock adapter works in unit tests
- Error handling covers auth, rate limit, not found, API errors

### Phase 2: Commands (Week 2)

**Goals**: User-facing commands, RFD integration

- [ ] Add `rfd github create-issue` command
- [ ] Add `rfd github sync-status` command
- [ ] Add `rfd github link-issue` command
- [ ] Update RFD metadata on operations
- [ ] Add `--sync-github` flag to `rfd status` command
- [ ] JSON output for all commands

**Success Criteria:**

- Can create issue for RFD (updates metadata)
- Can sync RFD status to GitHub (labels + comment)
- Can link existing issue to RFD
- All operations idempotent
- Clear error messages with suggestions

### Phase 3: Polish (Week 3)

**Goals**: Documentation, examples, edge cases

- [ ] Comprehensive documentation
- [ ] Example workflows in `cli/rfd/examples/`
- [ ] Handle edge cases (deleted issues, permissions, etc.)
- [ ] Rate limiting with exponential backoff
- [ ] Add `rfd github status` command (check auth, rate limit)
- [ ] Update CHANGELOG.md

**Success Criteria:**

- Documentation complete with examples
- Handles all error scenarios gracefully
- Rate limiting prevents hitting API limits
- Users can check GitHub integration status

### Phase 4: Automation (Week 4 - Optional)

**Goals**: Auto-sync, Skills integration

- [ ] Auto-create issue on `rfd create` (if configured)
- [ ] Auto-sync on status changes (if configured)
- [ ] Create `rfd-github` Skill for orchestration
- [ ] Consider MCP tool for webhooks (future)

**Success Criteria:**

- Config option for auto-create works
- Config option for auto-sync works
- Skill demonstrates full workflow
- Clear path to MCP tool if needed

## Testing Strategy

### Unit Tests

```rust
#[tokio::test]
async fn test_create_issue_updates_rfd_metadata() {
    let mock_github = MockGitHubAdapter::new();

    // Create issue
    let result = mock_github.create_issue(CreateIssueRequest {
        title: "RFD 003: Test".to_string(),
        body: "Test body".to_string(),
        labels: vec!["rfd:draft".to_string()],
    }).await;

    assert!(result.is_ok());
    let issue = result.unwrap();
    assert_eq!(issue.number, 1);

    // Verify RFD metadata was updated
    // ...
}

#[tokio::test]
async fn test_rate_limit_error() {
    let mock_github = MockGitHubAdapter::with_rate_limit_exceeded();
    let result = mock_github.create_issue(...).await;

    assert!(matches!(result, Err(GitHubError::RateLimit { .. })));
}
```

### Integration Tests

```rust
#[tokio::test]
#[ignore] // Only run when GITHUB_TOKEN is set
async fn test_real_github_create_issue() {
    if std::env::var("GITHUB_TOKEN").is_err() {
        return;
    }

    let adapter = GitHubAdapter::new("test/repo".to_string()).unwrap();

    let result = adapter.create_issue(CreateIssueRequest {
        title: "[Test] RFD Integration Test".to_string(),
        body: "Automated test - safe to close".to_string(),
        labels: vec!["test".to_string()],
    }).await;

    assert!(result.is_ok());

    // Clean up: close the test issue
    // ...
}
```

### Manual Testing Checklist

- [ ] Create issue for RFD (fresh RFD, no existing issue)
- [ ] Create issue fails gracefully if issue already exists
- [ ] Sync status updates labels correctly
- [ ] Sync status adds comment with details
- [ ] Link existing issue works
- [ ] Auth errors are clear and actionable
- [ ] Rate limit handling works
- [ ] JSON output is valid and complete
- [ ] Works in Claude Code environment (API only)

## Alternatives Considered

### Alternative 1: Support Both `gh` CLI and API

**Pros:**

- Flexibility for users who prefer `gh`
- `gh` handles auth automatically
- Battle-tested by GitHub

**Cons:**

- 2x the code to maintain
- 2x the testing surface
- Adapter selection complexity
- `gh` not available in Claude Code
- Inconsistent behavior between adapters

**Decision: Rejected**

- YAGNI - no clear benefit to supporting both
- API-only is simpler, faster to implement
- Port interface allows adding `gh` adapter later if needed

### Alternative 2: Use GitHub GraphQL API

**Pros:**

- More efficient (fetch exactly what we need)
- Better for complex queries
- Single request for multiple resources

**Cons:**

- More complex to implement
- Steeper learning curve
- REST API is sufficient for our needs
- GraphQL queries harder to debug

**Decision: Rejected for MVP**

- REST API simpler and well-documented
- Can add GraphQL adapter later via port interface
- Our queries are simple (single issue operations)

### Alternative 3: Build Separate `gh` CLI Tool

**Pros:**

- Reusable across all Tapestry tools
- Single GitHub client for ecosystem
- Could support more operations

**Cons:**

- Premature abstraction
- Only RFD tool needs GitHub now
- Can extract later if needed

**Decision: Rejected**

- YAGNI - extract when second tool needs GitHub
- Keep GitHub code in RFD tool for now
- Port interface makes extraction easy later

### Alternative 4: MCP Tool for GitHub Integration

**Pros:**

- Stateful (could watch for changes)
- Deep integration potential
- Could support webhooks

**Cons:**

- Overkill for simple operations
- Slower startup (100ms+ vs 10ms)
- More complex architecture
- Higher token cost

**Decision: Rejected for MVP**

- CLI tool sufficient for one-shot operations
- Can add MCP tool later for stateful workflows
- Skills can orchestrate CLI commands

## Security Considerations

### Authentication

**Token Storage:**

- `GITHUB_TOKEN` from environment variable only
- Never hardcoded or in config files
- Documented in error messages

**Token Scopes:**

- Require minimal scopes: `repo` (for private repos) or `public_repo`
- Document required scopes in setup guide
- Validate token has required scopes on first use

### Rate Limiting

**Protection:**

- Check rate limit headers on every response
- Exponential backoff on rate limit errors
- Warn when remaining < 100 requests

**Implementation:**

```rust
if remaining < 100 {
    eprintln!("Warning: Only {} API requests remaining", remaining);
    eprintln!("Rate limit resets at {}", reset_at);
}
```

### Data Exposure

**RFD Content:**

- Only sync summary/title to issue (not full content)
- Link to full RFD in issue body
- User controls what goes into issue template

**Error Messages:**

- Don't expose token in error messages
- Sanitize API responses before displaying
- Log at appropriate levels (no sensitive data in logs)

## Dependencies

### New Rust Crates

```toml
[dependencies]
# Existing dependencies
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# New for GitHub integration
async-trait = "0.1"           # Async trait methods
tokio = { version = "1", features = ["rt", "macros"] }  # Async runtime
reqwest = { version = "0.12", features = ["json"] }     # HTTP client
thiserror = "1.0"             # Error handling
```

**Size Impact:**

- Current binary: ~2.4MB
- Estimated with new deps: ~3.5MB
- Still well under 5MB target

**Startup Impact:**

- Async runtime adds ~5ms
- Still under 10ms target for CLI

## Open Questions

1. **Auto-sync Timing**: When status is updated, should we sync immediately or
   batch?

   - **Proposal**: Immediate (simpler, more intuitive)
   - **Alternative**: Batch with `rfd github sync-all` command

2. **Issue Closing**: Should we close GitHub issue when RFD is archived?

   - **Proposal**: No, keep issue open for historical discussion
   - **Alternative**: Auto-close with comment

3. **Label Management**: Should we create labels if they don't exist?

   - **Proposal**: No, fail with clear error message
   - **Alternative**: Auto-create with default colors

4. **Multiple Repos**: How to handle RFDs that span multiple repos?
   - **Proposal**: Out of scope for MVP
   - **Alternative**: Support multiple repo configs

## Success Metrics

### Immediate (MVP Launch)

- [ ] Can create GitHub issue for RFD
- [ ] Can sync RFD status to issue
- [ ] Can link existing issue to RFD
- [ ] All operations have JSON output
- [ ] Error messages are actionable
- [ ] Tests pass (unit + integration)

### Short-term (1 Month)

- [ ] 10+ RFDs with linked GitHub issues
- [ ] Zero manual issue creation for new RFDs
- [ ] Labels consistently reflect RFD state
- [ ] Skills successfully orchestrate workflows
- [ ] No GitHub rate limit issues

### Long-term (3 Months)

- [ ] 50+ RFDs with GitHub integration
- [ ] Community feedback incorporated
- [ ] Documented patterns for other tools
- [ ] Decision on MCP tool need
- [ ] External users adopt feature

## Future Enhancements

**Phase 5: Pull Request Integration** (RFD-004)

- Create PR when RFD accepted
- Link PRs to RFD implementation
- Track implementation progress

**Phase 6: MCP Tool** (if needed)

- Watch for issue changes via webhooks
- Auto-update RFD when issue closed
- Event-driven workflows

**Phase 7: Advanced Features**

- GitHub Projects integration
- Milestone tracking
- Release notes generation from RFDs
- Metrics dashboard

## References

- [RFC-002: RFD CLI Tool](./RFC-002-rfd-cli.md) - Foundation for this work
- [GitHub REST API Docs](https://docs.github.com/en/rest)
- [Tapestry Architecture](../../.claude/context/architecture.md)
- [Hexagonal Architecture](../../.claude/context/architecture.md#hexagonal-architecture)

## Appendix A: Complete Command Reference

```bash
# GitHub Integration Commands

# Create issue for RFD discussion
rfd github create-issue <rfd-id> [--format json]

# Sync RFD status to GitHub issue
rfd github sync-status <rfd-id> [--format json]

# Link existing GitHub issue to RFD
rfd github link-issue <rfd-id> --issue <number> [--format json]

# Check GitHub integration status
rfd github status [--format json]

# Modified commands

# Status command with GitHub sync
rfd status <rfd-id> --set <state> [--sync-github] [--format json]

# Create with auto-issue (if configured)
rfd create --title "..." --author "..." [--create-issue]
```

## Appendix B: Configuration Examples

**Minimal Configuration:**

```toml
[github]
repo = "adanoelle/tapestry"
```

**Full Configuration:**

```toml
[github]
repo = "adanoelle/tapestry"
auto_create_issue = true
auto_sync_status = true

[github.label_mapping]
draft = ["rfd:draft", "status:draft"]
review = ["rfd:review", "needs-review", "status:in-review"]
accepted = ["rfd:accepted", "status:accepted"]
implemented = ["rfd:implemented", "status:done"]
rejected = ["rfd:rejected", "status:rejected"]
archived = ["rfd:archived", "status:archived"]
```

## Appendix C: Error Code Reference

| Code                       | Message                       | Action                        |
| -------------------------- | ----------------------------- | ----------------------------- |
| `GITHUB_AUTH_FAILED`       | GITHUB_TOKEN not set          | Set environment variable      |
| `GITHUB_RATE_LIMIT`        | Rate limit exceeded           | Wait or use different token   |
| `GITHUB_NOT_FOUND`         | Issue/repo not found          | Check repo name, issue number |
| `GITHUB_PERMISSION_DENIED` | Insufficient permissions      | Check token scopes            |
| `ISSUE_ALREADY_EXISTS`     | RFD has discussion link       | Use sync-status instead       |
| `INVALID_REPO_FORMAT`      | Repo not in owner/repo format | Fix config                    |

---

**RFD Status**: DRAFT **Next Steps**: Review and feedback, then begin Phase 1
implementation **Related Work**: Depends on RFD CLI (RFC-002), enables Skills
orchestration
