# RFC-003: RFD Search Command

## Summary

Add a `search` command to the RFD CLI that enables fast, flexible searching across RFD documents. Supports field-specific search, case-sensitive matching, filter combination, and JSON output for AI agents.

## Status

**Status**: PROPOSED
**Created**: 2025-10-21
**Author**: Claude (via Claude Code)

## Motivation

### Current Problem

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
│   ├── search.rs          # NEW: Search implementation
│   └── mod.rs             # Update: Export search
├── main.rs                # Update: Add Search subcommand
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

### CLI Integration

**Updated help output:**
```bash
$ rfd --help

Commands:
  create     Create a new RFD document
  list       List all RFDs
  search     Search RFDs by content                    # NEW
  show       Show details of an RFD
  status     Update RFD status
  update     Update RFD metadata
  validate   Validate an RFD
  help       Print this message or the help of the given subcommand(s)
```

**Search command help:**
```bash
$ rfd search --help

Search RFDs by content

Usage: rfd search [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Search query (multiple terms use AND logic)

Options:
      --in <SCOPE>          Search scope [possible values: title, content, tags, metadata, all]
      --case-sensitive      Enable case-sensitive search
  -s, --status <STATUS>     Filter by status
  -a, --author <AUTHOR>     Filter by author
  -l, --limit <LIMIT>       Limit number of results
  -f, --format <FORMAT>     Output format [default: pretty] [possible values: pretty, json, quiet]
  -h, --help                Print help
```

## Alternative Approaches Considered

### Alternative 1: External grep Integration

**Approach**: Shell out to `grep` or `ripgrep` for searching

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

**Approach**: Add `--regex` flag for pattern matching

**Pros:**
- More powerful queries
- Familiar to developers

**Cons:**
- Adds complexity to MVP
- Not needed for common use cases
- Can add later without breaking changes

**Decision**: Deferred to future enhancement

### Alternative 3: Fuzzy Search

**Approach**: Use Levenshtein distance for typo tolerance

**Pros:**
- Better UX for typos
- More forgiving searches

**Cons:**
- Slower performance
- Unexpected results for users
- Adds dependency (fuzzy-matcher crate)

**Decision**: Deferred to future enhancement

### Alternative 4: Full-Text Index

**Approach**: Build search index on create/update

**Pros:**
- Instant search (< 1ms)
- Scales to 10,000+ documents

**Cons:**
- Adds complexity (index maintenance)
- Overkill for MVP (< 100 docs expected)
- Can add later transparently

**Decision**: Deferred - implement when we see 500+ docs

## Implementation Plan

### Phase 1: Core Search (Day 1 - 4 hours)
- Add CLI command structure
- Create search module
- Implement basic search logic
- Manual testing

### Phase 2: Field-Specific Search (Day 2 - 3 hours)
- Add `--in` flag
- Implement SearchScope logic
- Test each scope

### Phase 3: Filter Integration (Day 2 - 1 hour)
- Add filter flags (--status, --author, --limit)
- Integrate with existing filter logic
- Test combinations

### Phase 4: Testing & Polish (Day 3 - 4 hours)
- Unit tests (80% coverage target)
- Integration tests
- Error handling
- Documentation updates

### Phase 5: Documentation (Day 3 - 2 hours)
- Update README.md
- Add examples/search.sh
- Update ARCHITECTURE.md
- Update CHANGELOG.md

**Total estimate: 2-3 days (14-16 hours)**

## Success Criteria

### Functional Requirements
- ✅ Basic text search works
- ✅ Multiple terms use AND logic
- ✅ Field-specific search works (--in flag)
- ✅ Case sensitivity option works
- ✅ Combines with existing filters (status, author, limit)
- ✅ JSON output works
- ✅ Pretty output is readable

### Quality Requirements
- ✅ Unit test coverage > 80%
- ✅ Integration tests pass
- ✅ Zero clippy warnings
- ✅ Documentation complete with examples
- ✅ Help text is clear and accurate

### Performance Requirements
- ✅ Search 100 RFDs in < 150ms
- ✅ Startup time still < 10ms
- ✅ Binary size increase < 100KB
- ✅ Memory usage < 15MB peak

### User Experience Requirements
- ✅ Intuitive command syntax
- ✅ Helpful error messages
- ✅ Consistent with existing commands
- ✅ Agent-friendly JSON output

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_search_finds_matching_title()
fn test_search_finds_matching_content()
fn test_search_multiple_terms_and_logic()
fn test_search_case_insensitive_default()
fn test_search_case_sensitive_flag()
fn test_search_scope_title_only()
fn test_search_scope_content_only()
fn test_search_scope_tags_only()
fn test_search_scope_metadata()
fn test_search_with_status_filter()
fn test_search_with_author_filter()
fn test_search_with_limit()
fn test_search_no_results()
fn test_search_empty_query()
```

### Integration Tests
- Create temp directory with test RFDs
- Test end-to-end search command
- Test JSON output structure
- Test filter combinations
- Test error cases

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

## Dependencies

**No new dependencies required!**

The search implementation uses only standard library and existing dependencies:
- String matching: `std::string`
- Case conversion: `std::ascii::AsciiExt`
- File I/O: Existing `fs.rs` module
- Output: Existing `Output` module

## Migration & Compatibility

**No breaking changes:**
- This is a new command, doesn't modify existing commands
- All existing CLI flags work as before
- JSON output extends existing format (adds search metadata)

**Backward compatibility:**
- RFD file format unchanged
- Configuration unchanged
- Existing commands unchanged

## Documentation Updates

### Files to Update
- `cli/rfd/README.md` - Add search command examples
- `cli/rfd/ARCHITECTURE.md` - Document search design
- `CHANGELOG.md` - Document new feature
- `cli/rfd/examples/search.sh` - Add usage examples

### Example Content
```bash
# examples/search.sh
#!/bin/bash

echo "RFD Search Examples"
echo "==================="

# Basic search
rfd search "authentication"

# Field-specific
rfd search "api" --in tags

# Combine filters
rfd search "security" --status accepted --author alice

# JSON for agents
rfd search "oauth" --format json | jq '.rfds[] | .title'
```

## Open Questions

1. **Empty query handling**: Error or list all?
   - **Decision**: Error with helpful message

2. **Whitespace in queries**: How to search for exact phrases?
   - **Decision**: MVP doesn't support phrases, add later with quotes

3. **Special characters**: How to escape?
   - **Decision**: No escaping in MVP (literal search only)

4. **Performance threshold**: When to add indexing?
   - **Decision**: Monitor usage, add when > 500 RFDs or user feedback

## Risk Assessment

**Low Risk:**
- Pure additive feature (no breaking changes)
- No new dependencies
- Reuses existing infrastructure
- Simple algorithm (substring matching)

**Mitigations:**
- Comprehensive testing before merge
- Manual testing with real RFDs
- Performance benchmarking
- Clear documentation

## Conclusion

The search command is a natural next step for the RFD CLI. It's well-scoped, builds on existing infrastructure, and provides immediate value for both humans and AI agents. The implementation is straightforward, the performance is acceptable, and it sets the foundation for future enhancements like indexing and advanced search features.

**Recommendation**: Approve and implement in Phase 2 of Tapestry roadmap.

---

**Next Steps:**
1. Review and approve this RFC
2. Create implementation tasks
3. Begin Phase 1 development
4. Dogfood on Tapestry RFDs
