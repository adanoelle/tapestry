#!/usr/bin/env bash
#
# Custom Templates Example
#
# This script demonstrates:
# - Creating custom template directories
# - Writing custom templates
# - Using custom templates
# - Template variables and syntax
#
# For junior developers: Shows how to customize RFD format
# for your team or organization.

set -euo pipefail

echo "=== Custom Templates Example ==="
echo

# Step 1: Create template directory
echo "1. Creating custom template directory..."
mkdir -p .rfd/templates

echo "✓ Created .rfd/templates/"
echo

# Step 2: Create a minimal template
echo "2. Creating minimal template (minimal.md.jinja)..."
cat > .rfd/templates/minimal.md.jinja <<'TEMPLATE'
---
title: "{{ metadata.title }}"
authors: {{ metadata.authors | tojson }}
state: {{ metadata.state }}
created: {{ metadata.created | date(format="%Y-%m-%d") }}
---

# {{ metadata.title }}

_Status: {{ metadata.state }}_
_Authors: {{ metadata.authors | join(", ") }}_

## Overview

<!-- Brief summary here -->

## Details

<!-- Details here -->

## References

<!-- Links and references -->

---
_RFD {{ number }} · Last updated: {{ metadata.updated | date(format="%Y-%m-%d") }}_
TEMPLATE

echo "✓ Created minimal template"
echo

# Step 3: Create a detailed template
echo "3. Creating detailed template (detailed.md.jinja)..."
cat > .rfd/templates/detailed.md.jinja <<'TEMPLATE'
---
title: "{{ metadata.title }}"
authors: {{ metadata.authors | tojson }}
state: {{ metadata.state }}
created: {{ metadata.created | date(format="%Y-%m-%dT%H:%M:%SZ") }}
updated: {{ metadata.updated | date(format="%Y-%m-%dT%H:%M:%SZ") }}
{% if metadata.tags %}tags: {{ metadata.tags | tojson }}{% endif %}
{% if metadata.discussion %}discussion: "{{ metadata.discussion }}"{% endif %}
---

<div align="center">

# RFD {{ "%04d"|format(number) }} - {{ metadata.title }}

**Status**: `{{ metadata.state }}`
**Authors**: {{ metadata.authors | join(", ") }}
**Created**: {{ metadata.created | date(format="%B %d, %Y") }}
**Last Updated**: {{ metadata.updated | date(format="%B %d, %Y") }}

{% if metadata.discussion %}
📝 [**Discussion**]({{ metadata.discussion }})
{% endif %}

</div>

---

## 📋 Table of Contents

- [Summary](#summary)
- [Motivation](#motivation)
- [Proposal](#proposal)
  - [Overview](#overview)
  - [Technical Design](#technical-design)
  - [Security Considerations](#security-considerations)
- [Implementation](#implementation)
  - [Phase 1](#phase-1)
  - [Phase 2](#phase-2)
  - [Testing Strategy](#testing-strategy)
- [Alternatives Considered](#alternatives-considered)
- [Open Questions](#open-questions)
- [References](#references)
- [Appendix](#appendix)

---

## Summary

<!-- High-level overview in 2-3 sentences -->

## Motivation

### Problem Statement

<!-- What problem are we solving? -->

### Goals

- <!-- Goal 1 -->
- <!-- Goal 2 -->

### Non-Goals

- <!-- What we're explicitly not doing -->

## Proposal

### Overview

<!-- Technical approach -->

### Technical Design

<!-- Detailed design with diagrams if needed -->

### Security Considerations

<!-- Security implications and mitigations -->

## Implementation

### Phase 1

- [ ] Task 1
- [ ] Task 2

### Phase 2

- [ ] Task 3
- [ ] Task 4

### Testing Strategy

<!-- How will we verify this works? -->

## Alternatives Considered

### Alternative 1

**Pros**:
**Cons**:
**Decision**:

## Open Questions

- [ ] Question 1
- [ ] Question 2

## References

- Link 1
- Link 2

## Appendix

### Change Log

- {{ metadata.created | date(format="%Y-%m-%d") }}: Initial draft
- {{ metadata.updated | date(format="%Y-%m-%d") }}: Last update

---

<div align="center">

_RFD {{ "%04d"|format(number) }} · {{ metadata.title }}_
_Generated with [RFD CLI](https://github.com/yourusername/tapestry/tree/main/cli/rfd)_

</div>
TEMPLATE

echo "✓ Created detailed template"
echo

# Step 4: Create a team-specific template
echo "4. Creating team-specific template (team.md.jinja)..."
cat > .rfd/templates/team.md.jinja <<'TEMPLATE'
---
title: "{{ metadata.title }}"
authors: {{ metadata.authors | tojson }}
state: {{ metadata.state }}
created: {{ metadata.created | date(format="%Y-%m-%d") }}
team: "Platform Engineering"
priority: "Medium"
---

# [{{ metadata.state | upper }}] {{ metadata.title }}

**RFD Number**: {{ "%04d"|format(number) }}
**Team**: Platform Engineering
**Priority**: Medium
**Authors**: {{ metadata.authors | join(", ") }}
**Date**: {{ metadata.created | date(format="%Y-%m-%d") }}

## Executive Summary

<!-- TL;DR for leadership -->

## Context

<!-- Background and current situation -->

## Proposed Solution

<!-- Your proposal -->

## Success Criteria

- [ ] Criterion 1
- [ ] Criterion 2

## Resource Requirements

**Engineering**:
**Infrastructure**:
**Timeline**:

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
|      |        |            |

## Next Steps

1. Step 1
2. Step 2

---
**Reviewers**: <!-- @ mention reviewers -->
**Status**: {{ metadata.state }}
TEMPLATE

echo "✓ Created team template"
echo

# Step 5: List available templates
echo "5. Listing available templates..."
ls -la .rfd/templates/

echo
echo "✓ Templates created"
echo

# Step 6: Use minimal template
echo "6. Creating RFD with minimal template..."
rfd create \
    --title "Quick Update Proposal" \
    --author "Alice <alice@example.com>" \
    --template minimal

echo
echo "✓ Created RFD with minimal template"
echo

# Step 7: Use detailed template
echo "7. Creating RFD with detailed template..."
rfd create \
    --title "Major Feature Proposal" \
    --author "Bob <bob@example.com>" \
    --template detailed

echo
echo "✓ Created RFD with detailed template"
echo

# Step 8: Use team template
echo "8. Creating RFD with team template..."
rfd create \
    --title "Infrastructure Upgrade" \
    --author "Charlie <charlie@example.com>" \
    --template team

echo
echo "✓ Created RFD with team template"
echo

# Step 9: Show how to preview templates
echo "9. Showing template variables available..."
cat <<'EOF'

Template Variables Available:
=============================

{{ number }}                     - RFD number (e.g., 42)
{{ "%04d"|format(number) }}     - Zero-padded (e.g., 0042)

{{ metadata.title }}             - RFD title
{{ metadata.authors }}           - List of authors
{{ metadata.authors | join(", ") }} - Authors as string
{{ metadata.state }}             - Current state
{{ metadata.created }}           - Creation timestamp
{{ metadata.updated }}           - Last update timestamp
{{ metadata.tags }}              - List of tags (optional)
{{ metadata.discussion }}        - Discussion URL (optional)

Date Formatting:
{{ metadata.created | date(format="%Y-%m-%d") }}        - 2025-10-19
{{ metadata.created | date(format="%B %d, %Y") }}       - October 19, 2025
{{ metadata.created | date(format="%Y-%m-%dT%H:%M:%SZ") }} - ISO 8601

Conditionals:
{% if metadata.tags %}
Tags: {{ metadata.tags | tojson }}
{% endif %}

Loops:
{% for author in metadata.authors %}
- {{ author }}
{% endfor %}

EOF

echo "✓ Template guide shown"
echo

# Step 10: Show the created RFDs
echo "10. Showing RFDs created with different templates..."
rfd list

echo
echo "=== Custom Templates Complete! ==="
echo
echo "Summary:"
echo "  - Created .rfd/templates/ directory"
echo "  - Defined 3 custom templates:"
echo "    • minimal.md.jinja - Simple format"
echo "    • detailed.md.jinja - Comprehensive with TOC"
echo "    • team.md.jinja - Team-specific fields"
echo "  - Created RFDs using each template"
echo
echo "Template Syntax (Jinja2):"
echo "  - Variables: {{ variable }}"
echo "  - Filters: {{ variable | filter }}"
echo "  - Conditionals: {% if condition %}...{% endif %}"
echo "  - Loops: {% for item in list %}...{% endfor %}"
echo
echo "Next steps:"
echo "  - Customize templates for your team"
echo "  - Add organization-specific sections"
echo "  - Share templates across projects"
echo "  - Version control templates in .rfd/templates/"
echo
echo "Cleanup:"
echo "  rm -rf .rfd/  # Remove custom templates"
