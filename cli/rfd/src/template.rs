use minijinja::{context, Environment};
use std::collections::HashMap;
use std::path::Path;

use crate::config::RfdConfig;
use crate::document::RfdMetadata;
use crate::error::RfdError;

/// Template data for rendering RFD documents
pub struct TemplateContext {
    pub rfd_number: String,
    pub title: String,
    pub authors: Vec<String>,
    pub state: String,
    pub created: String,
    pub discussion: Option<String>,
    pub tags: Vec<String>,
}

impl From<(u32, &RfdMetadata)> for TemplateContext {
    fn from((number, metadata): (u32, &RfdMetadata)) -> Self {
        Self {
            rfd_number: format!("{:04}", number),
            title: metadata.title.clone(),
            authors: metadata.authors.clone(),
            state: metadata.state.to_string(),
            created: metadata.created.format("%Y-%m-%d").to_string(),
            discussion: metadata.discussion.clone(),
            tags: metadata.tags.clone(),
        }
    }
}

/// Template engine for rendering RFD documents.
///
/// # Memory Management
///
/// Previously, this used `Box::leak` to achieve 'static lifetime for templates,
/// which caused a memory leak on each instantiation. This has been refactored to
/// use a simpler approach: store templates as owned Strings and render directly
/// from them, avoiding the Environment's 'static requirement.
///
/// For junior developers: When you see lifetime issues like this, consider whether
/// you really need the 'static lifetime. Often, rearchitecting to avoid it is better
/// than using unsafe patterns like Box::leak.
pub struct TemplateEngine {
    /// Template contents stored as owned Strings
    /// Key: template name (e.g., "default")
    /// Value: template content (Jinja2 format)
    templates: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine with default and custom templates.
    ///
    /// This loads templates in priority order:
    /// 1. Built-in default template (embedded in binary)
    /// 2. User templates from ~/.config/rfd/templates/
    /// 3. Project templates from .rfd/templates/ (overrides previous)
    ///
    /// # Arguments
    ///
    /// * `config` - RFD configuration specifying template search paths
    ///
    /// # Returns
    ///
    /// Returns a template engine ready to render RFD documents, or an error if
    /// template loading fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rfd::template::TemplateEngine;
    /// use rfd::config::RfdConfig;
    ///
    /// let config = RfdConfig::default();
    /// let engine = TemplateEngine::new(&config)?;
    /// ```
    ///
    /// # For Junior Developers
    ///
    /// This function loads all available templates into memory. Templates are stored
    /// as Strings in a HashMap, which we can then use to render documents. This is
    /// simpler and safer than trying to manage complex lifetimes with Environment<'static>.
    pub fn new(config: &RfdConfig) -> Result<Self, RfdError> {
        let mut templates = HashMap::new();

        // Add built-in default template
        templates.insert(
            "default".to_string(),
            include_str!("../templates/default.md.jinja").to_string(),
        );

        // Load custom templates from filesystem (project, then user)
        for template_dir in config.template_paths() {
            if template_dir.exists() {
                Self::load_templates_from_dir(&mut templates, &template_dir)?;
            }
        }

        Ok(Self { templates })
    }

    /// Load all templates from a directory into the HashMap
    fn load_templates_from_dir(
        templates: &mut std::collections::HashMap<String, String>,
        dir: &Path,
    ) -> Result<(), RfdError> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries = std::fs::read_dir(dir).map_err(|e| RfdError::TemplateError {
            message: format!("Failed to read template directory: {}", e),
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| RfdError::TemplateError {
                message: format!("Failed to read directory entry: {}", e),
            })?;

            let entry_path = entry.path();

            // Only process .jinja or .md.jinja files
            if entry_path.is_file() {
                if let Some(name) = entry_path.file_name().and_then(|s| s.to_str()) {
                    if name.ends_with(".jinja") {
                        // Template name without .jinja extension
                        let template_name = name
                            .trim_end_matches(".md.jinja")
                            .trim_end_matches(".jinja");

                        let content = std::fs::read_to_string(&entry_path).map_err(|e| {
                            RfdError::TemplateError {
                                message: format!("Failed to read template file: {}", e),
                            }
                        })?;

                        // Insert (will override if duplicate name)
                        templates.insert(template_name.to_string(), content);
                    }
                }
            }
        }

        Ok(())
    }

    /// Render a template with the given context.
    ///
    /// # Arguments
    ///
    /// * `template_name` - Name of the template to use (e.g., "default")
    /// * `number` - RFD number (e.g., 42 will be formatted as "0042")
    /// * `metadata` - RFD metadata containing title, authors, state, etc.
    ///
    /// # Returns
    ///
    /// Returns the rendered template as a String, or an error if:
    /// - The template doesn't exist
    /// - Template syntax is invalid
    /// - Rendering fails (e.g., missing variable)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rfd::template::TemplateEngine;
    /// use rfd::document::RfdMetadata;
    /// use rfd::config::RfdConfig;
    ///
    /// let config = RfdConfig::default();
    /// let engine = TemplateEngine::new(&config)?;
    /// let metadata = RfdMetadata::new(
    ///     "My Proposal".to_string(),
    ///     vec!["Alice <alice@example.com>".to_string()],
    /// );
    ///
    /// let content = engine.render("default", 1, &metadata)?;
    /// assert!(content.contains("RFD 0001"));
    /// ```
    ///
    /// # For Junior Developers
    ///
    /// This method does three things:
    /// 1. Finds the template by name in our HashMap
    /// 2. Creates a fresh Environment (no 'static lifetime needed!)
    /// 3. Renders the template with the RFD data
    ///
    /// The key insight: we create a new Environment each time, which is fine for
    /// CLI performance and avoids complex lifetime management.
    pub fn render(
        &self,
        template_name: &str,
        number: u32,
        metadata: &RfdMetadata,
    ) -> Result<String, RfdError> {
        // Get template content from our HashMap
        let template_content =
            self.templates
                .get(template_name)
                .ok_or_else(|| RfdError::TemplateError {
                    message: format!(
                        "Template '{}' not found. Available templates: {}",
                        template_name,
                        self.templates
                            .keys()
                            .map(|k| k.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                })?;

        // Create a fresh Environment for this render
        // For junior developers: This avoids lifetime issues! Each render gets its own
        // Environment, which is dropped when we're done. For a CLI tool, the performance
        // cost is negligible (< 1ms), and it makes the code much simpler.
        let mut env = Environment::new();
        env.add_template(template_name, template_content)
            .map_err(|e| RfdError::TemplateError {
                message: format!("Failed to parse template '{}': {}", template_name, e),
            })?;

        let template = env
            .get_template(template_name)
            .map_err(|e| RfdError::TemplateError {
                message: format!("Failed to get template '{}': {}", template_name, e),
            })?;

        // Build context from metadata
        let ctx = TemplateContext::from((number, metadata));

        // Render template with context
        let rendered = template
            .render(context! {
                rfd_number => ctx.rfd_number,
                title => ctx.title,
                authors => ctx.authors,
                state => ctx.state,
                created => ctx.created,
                discussion => ctx.discussion,
                tags => ctx.tags,
            })
            .map_err(|e| RfdError::TemplateError {
                message: format!("Failed to render template: {}", e),
            })?;

        Ok(rendered)
    }

    /// List available templates.
    ///
    /// Returns a list of all template names that can be used with `render()`.
    /// This is primarily useful for introspection and debugging.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rfd::template::TemplateEngine;
    /// use rfd::config::RfdConfig;
    ///
    /// let config = RfdConfig::default();
    /// let engine = TemplateEngine::new(&config)?;
    ///
    /// let templates = engine.list_templates();
    /// assert!(templates.contains(&"default".to_string()));
    /// ```
    ///
    /// # For Junior Developers
    ///
    /// This method is part of the public API but not currently used by commands.
    /// It's kept for future features (e.g., `rfd templates` command) and testing.
    #[allow(dead_code)]
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Check if a template exists.
    ///
    /// # Arguments
    ///
    /// * `name` - Template name to check (e.g., "default")
    ///
    /// # Returns
    ///
    /// Returns `true` if the template is loaded, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rfd::template::TemplateEngine;
    /// use rfd::config::RfdConfig;
    ///
    /// let config = RfdConfig::default();
    /// let engine = TemplateEngine::new(&config)?;
    ///
    /// assert!(engine.has_template("default"));
    /// assert!(!engine.has_template("nonexistent"));
    /// ```
    ///
    /// # For Junior Developers
    ///
    /// This is a utility method for checking template existence before rendering.
    /// Kept for future validation logic and programmatic use.
    #[allow(dead_code)]
    pub fn has_template(&self, name: &str) -> bool {
        self.templates.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_rendering() {
        let config = RfdConfig::default();
        let engine = TemplateEngine::new(&config).unwrap();

        let metadata = RfdMetadata::new(
            "Test Feature".to_string(),
            vec!["Alice <alice@example.com>".to_string()],
        );

        let rendered = engine.render("default", 1, &metadata).unwrap();

        assert!(rendered.contains("RFD 0001: Test Feature"));
        assert!(rendered.contains("Alice <alice@example.com>"));
        assert!(rendered.contains("## Summary"));
        assert!(rendered.contains("## Motivation"));
        assert!(rendered.contains("## Proposal"));
    }

    #[test]
    fn test_template_context() {
        let metadata = RfdMetadata::new(
            "Test".to_string(),
            vec!["Alice <alice@example.com>".to_string()],
        );

        let ctx = TemplateContext::from((42, &metadata));

        assert_eq!(ctx.rfd_number, "0042");
        assert_eq!(ctx.title, "Test");
        assert_eq!(ctx.state, "draft");
        assert_eq!(ctx.authors.len(), 1);
    }

    #[test]
    fn test_list_templates() {
        let config = RfdConfig::default();
        let engine = TemplateEngine::new(&config).unwrap();

        let templates = engine.list_templates();
        assert!(templates.contains(&"default".to_string()));
    }
}
