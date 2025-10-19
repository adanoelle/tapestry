# git-workflow

> **Status**: ⏸️ Paused during Skills exploration
>
> This MCP tool development is temporarily on hold while we validate the Skills-first approach with lighter-weight CLI tools. The architecture and domain logic are solid and will serve as a reference for future MCP tool development.

Automates git workflow with conventional commits, smart staging, and change analysis based on RFC-001

## Usage

```rust
use tapestry::tools::git_workflow;

let tool = git_workflow::create_tool();
let input = git_workflow::GitworkflowInput {
    // Set input fields
};

let output = tool.execute(input).await?;
```

## Configuration

The tool can be configured via environment variables:

- `GIT-WORKFLOW_DEBUG`: Enable debug logging (default: false)
- `GIT-WORKFLOW_TIMEOUT`: Timeout in seconds (default: 30)

## Architecture

This tool follows Tapestry's hexagonal architecture:

- **Domain**: Core business logic in `domain.rs`
- **Port**: Interface definition in `port.rs`
- **Adapter**: MCP implementation in `adapter.rs`
- **Config**: Configuration in `config.rs`

## Testing

Run tests with:
```bash
# Unit tests
cargo test --lib tools::git_workflow

# Integration tests
cargo test --test git_workflow_test
```

## Performance

Target metrics:
- P50 latency: < 100ms
- P99 latency: < 500ms
- Memory usage: < 10MB

## Security Considerations

- Input validation implemented in domain layer
- Resource limits enforced via configuration
- No external command execution
- All errors sanitized before returning

## Contributing

See the RFC in `docs/design/features/RFC-002-git-workflow.md`
