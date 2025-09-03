# MCP Tools Design Principles for Rust

## Quick Reference for Claude Code

When building MCP tools in Rust, follow these battle-tested principles from
Anthropic and industry leaders.

---

## 🏗️ Architecture: Use Hexagonal Pattern

```
mcp-tool/
├── domain/           # Core business logic (no MCP dependencies)
│   ├── entities/     # Domain models
│   ├── services/     # Business rules
│   └── ports/        # Interface traits
├── adapters/         # External interfaces
│   ├── mcp/         # MCP protocol adapter (rmcp implementation)
│   ├── api/         # External API adapters
│   └── storage/     # Persistence adapters
└── infrastructure/   # Technical concerns
    ├── transport/    # STDIO/SSE/HTTP
    └── config/       # Configuration
```

**Key Rule**: Dependencies flow inward. Domain knows nothing about adapters or
infrastructure.

---

## 🦀 Rust Implementation Patterns

### Tool Definition

```rust
use rmcp::tool;

#[tool(
    name = "analyze_data",
    description = "Analyzes data and returns insights. Specify format: 'summary' for overview, 'detailed' for full analysis"
)]
async fn analyze_data(
    #[param(description = "Data source URL or identifier")]
    source: String,
    #[param(description = "Analysis depth: 'summary' or 'detailed'")]
    depth: AnalysisDepth,
) -> McpResult<Analysis> {
    // Implementation
}
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Configuration required: {0}")]
    Configuration(String),  // Guides agent to fix config
    #[error("Retry with: {suggestion}")]
    Recoverable { suggestion: String },
    #[error("Internal error")]
    Internal,  // Never expose implementation details
}

// Always provide actionable errors for agents
fn handle_auth_failure() -> McpError {
    McpError::Configuration(
        "Set API_TOKEN environment variable with valid credentials"
    )
}
```

### Async-First Design

```rust
// Always use async for I/O operations
use tokio::time::{sleep, Duration};

async fn with_retry<T, F, Fut>(
    operation: F,
    max_attempts: u32,
) -> Result<T, McpError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, McpError>>,
{
    for attempt in 0..max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts - 1 => return Err(e),
            _ => sleep(Duration::from_secs(2_u64.pow(attempt))).await,
        }
    }
    unreachable!()
}
```

---

## 🔒 Security Requirements (Non-Negotiable)

### Authentication

```rust
// NEVER use static tokens
// ALWAYS validate token audience
impl McpServer {
    async fn validate_request(&self, req: &Request) -> McpResult<()> {
        // 1. Validate OAuth token
        let claims = self.validate_oauth_token(&req.token)?;

        // 2. Check token audience matches this server
        if claims.audience != self.server_id {
            return Err(McpError::AuthenticationFailed);
        }

        // 3. Enforce rate limits
        self.rate_limiter.check(&claims.subject).await?;

        // 4. Validate and sanitize inputs
        self.validate_inputs(&req.params)?;

        Ok(())
    }
}
```

### Principle of Least Privilege

```rust
// Each tool gets minimal permissions
#[tool]
async fn read_user_data(user_id: String) -> McpResult<UserData> {
    // ❌ WRONG: Full database access
    // db.query("SELECT * FROM users")?

    // ✅ CORRECT: Scoped access only
    db.get_user_by_id(&user_id, &["name", "email"]).await?
}
```

---

## 🎯 Tool Design Principles

### 1. Tool Budget Optimization

```rust
// ❌ AVOID: Multiple single-purpose tools
#[tool] async fn get_user(id: String) -> User { }
#[tool] async fn get_user_posts(id: String) -> Vec<Post> { }
#[tool] async fn get_user_comments(id: String) -> Vec<Comment> { }

// ✅ BETTER: Consolidated tool with options
#[tool(description = "Retrieve user information with optional related data")]
async fn get_user_info(
    id: String,
    #[param(description = "Include related: 'posts', 'comments', 'all'")]
    include: Option<Vec<String>>,
) -> McpResult<UserInfo> { }
```

### 2. Agent-Friendly Responses

```rust
// Tool descriptions must be precise and actionable
#[tool(
    description = "Searches code repository for patterns. Returns file paths and line numbers. Use 'regex' for complex patterns, 'text' for literal search"
)]
async fn search_code() { }

// Errors must guide the agent
match external_api.call().await {
    Err(ApiError::RateLimit) => {
        Err(McpError::Recoverable {
            suggestion: "Wait 60 seconds before retrying"
        })
    }
    Err(ApiError::InvalidKey) => {
        Err(McpError::Configuration(
            "Configure valid API_KEY in environment"
        ))
    }
}
```

### 3. Token Efficiency

```rust
// Implement pagination for large results
#[tool]
async fn list_items(
    #[param(description = "Maximum items to return (default: 10, max: 100)")]
    limit: Option<usize>,
    #[param(description = "Pagination cursor from previous response")]
    cursor: Option<String>,
) -> McpResult<ItemsPage> {
    let limit = limit.unwrap_or(10).min(100);
    // Return only essential fields + continuation cursor
}
```

---

## 🚀 Performance Patterns

### Zero-Copy Operations

```rust
use bytes::Bytes;

// Use Bytes for network data
async fn fetch_content(url: &str) -> McpResult<Bytes> {
    // Avoid String allocation for binary data
    let response = client.get(url).await?;
    Ok(response.bytes().await?)
}
```

### Connection Pooling

```rust
use deadpool::managed::Pool;

struct McpServer {
    db_pool: Pool<DbConnection>,
    http_client: reqwest::Client,  // Reuses connections
}
```

### Efficient Serialization

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // Match MCP conventions
struct ToolResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,  // Omit null fields to save tokens
    #[serde(flatten)]
    metadata: HashMap<String, Value>,
}
```

---

## 🧪 Testing Checklist

```rust
#[cfg(test)]
mod tests {
    // 1. Unit test business logic
    #[test]
    fn test_domain_logic() {
        // Test without MCP protocol
    }

    // 2. Integration test MCP protocol
    #[tokio::test]
    async fn test_tool_invocation() {
        // Test with mock MCP client
    }

    // 3. Security test
    #[tokio::test]
    async fn test_rejects_invalid_token() {
        // Ensure auth works correctly
    }

    // 4. Error handling test
    #[tokio::test]
    async fn test_actionable_errors() {
        // Verify errors guide the agent
    }
}
```

---

## 📦 Deployment

### Docker (11MB optimized image)

```dockerfile
FROM rust:1-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static:nonroot
COPY --from=builder /app/target/release/mcp-server /usr/local/bin/
ENTRYPOINT ["mcp-server"]
```

### Environment Variables

```rust
// Use structured config with validation
#[derive(Deserialize)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
    api_token: String,  // Required, will fail fast if missing
    #[serde(default = "default_timeout")]
    timeout_seconds: u64,
}

fn default_port() -> u16 { 8080 }
fn default_timeout() -> u64 { 30 }
```

---

## ⚠️ Critical Anti-Patterns to Avoid

```rust
// ❌ NEVER: Expose internal errors
Err(format!("Database error: {}", db_error))

// ❌ NEVER: Use static/long-lived tokens
const API_KEY: &str = "sk-1234567890";

// ❌ NEVER: Write to stdout in STDIO transport
println!("Debug info");  // Breaks JSON-RPC protocol

// ❌ NEVER: Block async runtime
std::thread::sleep(Duration::from_secs(1));  // Use tokio::time::sleep

// ❌ NEVER: Unlimited resource consumption
let all_data = db.fetch_all().await?;  // Implement pagination

// ❌ NEVER: Accept unvalidated input
sql_query(&format!("SELECT * FROM users WHERE id = {}", user_input))
```

---

## 📊 Monitoring Essentials

```rust
// Track these metrics
metrics! {
    counter!("mcp.tool.invocations", "tool" => tool_name);
    histogram!("mcp.tool.duration", "tool" => tool_name);
    counter!("mcp.errors", "type" => error_type);
    gauge!("mcp.token.usage", tokens_used);
}

// Structured logging
use tracing::{info, warn, error};

info!(
    tool = %tool_name,
    user_id = %user_id,
    duration_ms = %duration.as_millis(),
    "Tool execution completed"
);
```

---

## 🎯 Quick Decision Guide

**When to create a new tool:**

- Single, clear responsibility
- Can be described in one sentence
- Doesn't duplicate existing tool functionality

**When to add parameters to existing tool:**

- Related functionality
- Shares same authentication/resources
- Would reduce agent's tool selection complexity

**When to chain operations internally:**

- Multiple steps always occur together
- Intermediate results aren't useful to agent
- Reduces token usage significantly

---

## 📚 Resources

- Official rmcp SDK: `rmcp = "0.3.2"`
- Test with: `npx @modelcontextprotocol/inspector`
- Security: Follow OWASP Top 10 for LLMs
- Performance: Target < 100ms P50, < 500ms P99

---

Remember: **You're building for AI agents, not humans.** Every decision should
optimize for agent comprehension and execution.
