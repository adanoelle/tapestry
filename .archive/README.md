# Archived Content

This directory contains archived configurations and files that are temporarily set aside while we focus on core development.

## Contents

### github/
- **GitHub Actions workflows**: CI/CD pipelines for automated testing and validation
- **Issue templates**: Templates for bug reports, feature requests, and design docs
- **Community automation**: Welcome messages and auto-labeling

## Why Archived?

We're focusing on building the core MCP tools and AI-assisted development workflow first. These GitHub configurations will be revisited and refined once we have:

1. A working implementation of our first MCP tool (Git Workflow)
2. Validated our agent-driven development process
3. Established clear patterns for tool development

## To Restore

When ready to reintegrate GitHub Actions:

```bash
# Move back to root
mv .archive/github .github

# Review and update workflows based on lessons learned
```

## Notes

The workflows were updated for AI-assisted development but need real-world testing with actual tool implementations before being finalized.