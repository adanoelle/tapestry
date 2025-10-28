---
title: RFD Search Command
authors:
  - adanoelle <ada@tapestrylabs.org>
state: implemented
created: 2025-10-21T00:00:00Z
updated: 2025-10-27T20:26:11.790055090Z
tags:
  - rfd-cli
  - search
  - agent-friendly
  - cli
---

# RFD 0004: RFD Search Command

## Summary

Add a `search` command to the RFD CLI that enables fast, flexible searching across RFD documents. Supports field-specific search, case-sensitive matching, filter combination, and JSON output for AI agents.

## Status

**Status**: IMPLEMENTED (merged from main branch on 2025-10-27)

## Motivation

### Current State

With the RFD CLI in production, users will accumulate 10s-100s of RFD documents. Currently, the only way to find specific content is:

1. **Manual scanning** - Run `rfd list` and visually search titles
2. **grep/ripgrep** - Use external tools to search file contents
3. **File system browsing** - Open files one by one

These approaches are:
- **Slow**: Require multiple steps
- **Agent-unfriendly**: External tools don't integrate with JSON output
- **Inconsistent**: No structured way to search metadata vs content
- **Error-prone**: grep doesn't understand RFD structure

### Why This Matters

1. **Dogfooding**: Once we convert Tapestry docs to RFD format, we'll need search immediately
2. **AI-first**: Claude needs structured search to answer questions about RFDs
3. **Productivity**: Fast search is essential for 20+ documents
4. **Foundation**: Search infrastructure enables future features (dependencies, cross-references)

### User Stories

**As a developer:**
- I want to search RFDs by keyword to quickly find relevant design documents
- I want to search specific fields (title, content, tags) to narrow results
- I want to combine search with filters (status, author) for precise queries

**As an AI agent:**
- I want structured search via CLI with predictable JSON output
- I want to programmatically find RFDs to answer user questions
- I want to search without external dependencies or complex parsing

## Detailed Design

### Command Interface

```bash
# Basic search (searches title + content by default)
rfd search "authentication"
rfd search "oauth api"           # Multiple terms (AND logic)

# Field-specific search
rfd search "auth" --in title     # Title only
rfd search "auth" --in content   # Content only
rfd search "auth" --in tags      # Tags only
rfd search "auth" --in metadata  # Title + tags + authors
rfd search "auth" --in all       # Title + content (default)

# Case sensitivity
rfd search "OAuth" --case-sensitive

# Combine with existing filters
rfd search "api" --status accepted
rfd search "api" --author ada --limit 5
rfd search "security" --status draft --in tags

# JSON output for agents
rfd search "authentication" --format json
```

### Search Behavior

**Default behavior:**
- Case-insensitive substring matching
- Searches title + content (not metadata by default)
- Multiple terms use AND logic (all must match)

**Search scopes (`--in` flag):**
- `title` - Search title field only
- `content` - Search markdown body only
- `tags` - Search tags array only
- `metadata` - Search title + tags + authors
- `all` - Search title + content (default)

**Multiple search terms:**
```bash
# All terms must match (AND logic)
rfd search "oauth api"
# Finds RFDs containing both "oauth" AND "api"
```

**Case sensitivity:**
```bash
# Default: case-insensitive
rfd search "oauth"     # Matches "OAuth", "oauth", "OAUTH"

# Explicit case-sensitive
rfd search "OAuth" --case-sensitive  # Only matches "OAuth"
```

### Output Format

**Pretty mode (default):**
```
Found 3 RFDs matching "authentication":

  [0001] draft    - Authentication System
         By: Alice <alice@example.com>
         Tags: api, security
         Updated: 2025-10-17

  [0005] accepted - OAuth Integration
         By: Bob <bob@example.com>
         Tags: auth, oauth
         Updated: 2025-10-19

  [0012] review   - Multi-Factor Auth
         By: Alice <alice@example.com>
         Tags: security, mfa
         Updated: 2025-10-20
```

**JSON mode:**
```json
{
  "query": "authentication",
  "scope": "all",
  "case_sensitive": false,
  "total": 3,
  "rfds": [
    {
      "id": "0001",
      "title": "Authentication System",
      "state": "draft",
      "authors": ["Alice <alice@example.com>"],
      "created": "2025-10-17T00:00:00Z",
      "updated": "2025-10-20T00:00:00Z",
      "tags": ["api", "security"],
      "path": "rfds/0001-authentication-system.md"
    }
  ]
}
```

**Quiet mode:**
```
# No output unless error
```

### Technical Implementation

**Architecture:**
```
cli/rfd/src/
├── commands/
│   ├── search.rs          # Search implementation
│   └── mod.rs             # Export search
├── main.rs                # Add Search subcommand
```

**Core types:**
```rust
// SearchScope enum
pub enum SearchScope {
    Title,       // Search title only
    Content,     // Search markdown body only
    Tags,        // Search tags only
    Metadata,    // Search title + tags + authors
    All,         // Search title + content (default)
}

// Main search function
pub fn execute(
    query: String,
    search_in: Option<String>,
    case_sensitive: bool,
    status: Option<String>,
    author: Option<String>,
    limit: Option<usize>,
    output: &Output,
) -> Result<(), RfdError>
```

**Search algorithm:**
```rust
fn matches_search(
    doc: &RfdDocument,
    terms: &[String],
    scope: &SearchScope,
    case_sensitive: bool,
) -> bool {
    // Split query into terms
    // Extract searchable text based on scope
    // Check if all terms match (AND logic)
    // Apply case sensitivity
}
```

**Execution flow:**
1. Parse search query into terms (split on whitespace)
2. Parse scope from `--in` flag
3. Load all RFDs via `find_all_rfds()`
4. Apply status/author filters first (early exit)
5. Apply search matching per document
6. Sort results by RFD number (descending)
7. Apply limit if specified
8. Output via `Output::list()` (reuses existing code)

### Performance

**Expected performance:**
| Documents | Load Time | Search Time | Total Time |
|-----------|-----------|-------------|------------|
| 10        | ~10ms     | ~1ms        | ~11ms      |
| 100       | ~100ms    | ~10ms       | ~110ms     |
| 1,000     | ~1s       | ~100ms      | ~1.1s      |

**Optimization strategy:**
- MVP: Sequential search (acceptable for < 1000 docs)
- Future: Build `.rfd/index.json` for instant search

**Binary size impact:**
- Expected increase: < 50KB (no new dependencies)
- Search logic is pure Rust, minimal overhead

## Implementation

**Status**: ✅ IMPLEMENTED (as of 2025-10-27)

The search feature was implemented and merged from the main branch. All planned phases are complete.

### Phase 1: Core Search ✅
- [x] CLI command structure
- [x] Search module created
- [x] Basic search logic implemented
- [x] Manual testing complete

### Phase 2: Field-Specific Search ✅
- [x] `--in` flag added
- [x] SearchScope logic implemented
- [x] All scopes tested

### Phase 3: Filter Integration ✅
- [x] Filter flags (--status, --author, --limit) added
- [x] Integrated with existing filter logic
- [x] Tested combinations

### Phase 4: Testing & Polish ✅
- [x] Unit tests (80% coverage achieved)
- [x] Integration tests passing
- [x] Error handling complete
- [x] Documentation updated

### Phase 5: Documentation ✅
- [x] README.md updated
- [x] Examples added
- [x] ARCHITECTURE.md updated
- [x] CHANGELOG.md updated

**Total implementation time: 2-3 days (14-16 hours)**

## Alternative Approaches

### Alternative 1: External grep Integration

**Pros:**
- Leverage battle-tested search tools
- Very fast (ripgrep is optimized)
- Regex support out of the box

**Cons:**
- Requires external dependency (breaks portability)
- Doesn't understand RFD structure (YAML frontmatter)
- Hard to integrate with JSON output
- Can't combine with metadata filters easily

**Decision**: Rejected - Native implementation better fits our agent-friendly design

### Alternative 2: Regex-Based Search

**Pros:**
- More powerful queries
- Familiar to developers

**Cons:**
- Adds complexity to MVP
- Not needed for common use cases
- Can add later without breaking changes

**Decision**: Deferred to future enhancement

### Alternative 3: Fuzzy Search

**Pros:**
- Better UX for typos
- More forgiving searches

**Cons:**
- Slower performance
- Unexpected results for users
- Adds dependency (fuzzy-matcher crate)

**Decision**: Deferred to future enhancement

### Alternative 4: Full-Text Index

**Pros:**
- Instant search (< 1ms)
- Scales to 10,000+ documents

**Cons:**
- Adds complexity (index maintenance)
- Overkill for MVP (< 100 docs expected)
- Can add later transparently

**Decision**: Deferred - implement when we see 500+ docs

## Success Criteria

### Functional Requirements ✅
- [x] Basic text search works
- [x] Multiple terms use AND logic
- [x] Field-specific search works (--in flag)
- [x] Case sensitivity option works
- [x] Combines with existing filters (status, author, limit)
- [x] JSON output works
- [x] Pretty output is readable

### Quality Requirements ✅
- [x] Unit test coverage > 80%
- [x] Integration tests pass
- [x] Zero clippy warnings
- [x] Documentation complete with examples
- [x] Help text is clear and accurate

### Performance Requirements ✅
- [x] Search 100 RFDs in < 150ms
- [x] Startup time still < 10ms
- [x] Binary size increase < 100KB
- [x] Memory usage < 15MB peak

### User Experience Requirements ✅
- [x] Intuitive command syntax
- [x] Helpful error messages
- [x] Consistent with existing commands
- [x] Agent-friendly JSON output

## Future Enhancements

These are explicitly **not** in scope for this RFC but documented for future consideration:

### Search Result Snippets
```bash
rfd search "authentication" --snippet
# Shows matching text context:
# [0001] Authentication System
#        ...implementing OAuth 2.0 authentication for...
```

### OR Logic
```bash
rfd search "oauth OR saml" --or
# Matches RFDs with either "oauth" OR "saml"
```

### Regex Support
```bash
rfd search "auth(entication)?" --regex
# Matches "auth" or "authentication"
```

### Result Highlighting
```bash
rfd search "oauth" --highlight
# Highlights matched terms in color output
```

### Search Index
- Build `.rfd/index.json` on create/update
- Instant search (< 1ms for 10,000 docs)
- Automatically rebuilds on changes

### Relevance Scoring
- Rank results by match quality
- Title matches rank higher than content
- Multiple matches rank higher

## Open Questions

1. **Empty query handling**: Error or list all?
   - **Decision**: Error with helpful message

2. **Whitespace in queries**: How to search for exact phrases?
   - **Decision**: MVP doesn't support phrases, add later with quotes

3. **Special characters**: How to escape?
   - **Decision**: No escaping in MVP (literal search only)

4. **Performance threshold**: When to add indexing?
   - **Decision**: Monitor usage, add when > 500 RFDs or user feedback

## References

- [RFD-002](./0002-rfd-cli-tool.md) - RFD CLI Tool
- [CLAUDE.md](../CLAUDE.md) - Tapestry architecture

---

**Authors**: adanoelle <ada@tapestrylabs.org>
**Created**: 2025-10-21
**Updated**: 2025-10-27
**State**: implemented
