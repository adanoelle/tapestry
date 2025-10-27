//! Integration tests for the RFD CLI.
//!
//! These tests verify end-to-end functionality by invoking the actual
//! RFD binary and checking its behavior in realistic scenarios.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test environment with a temporary directory
fn setup_test_env() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

/// Helper to run RFD command in a test directory
fn rfd_cmd(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("rfd").expect("Failed to find rfd binary");
    cmd.current_dir(dir.path());
    cmd
}

#[test]
fn test_help_displays_usage() {
    let mut cmd = Command::cargo_bin("rfd").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CLI tool for managing RFD"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"));
}

#[test]
fn test_create_rfd_success() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Test RFD",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created RFD 0001"));

    // Verify file was created
    let rfd_path = temp_dir.path().join("rfds/0001-test-rfd.md");
    assert!(rfd_path.exists(), "RFD file should exist");

    // Verify content
    let content = fs::read_to_string(&rfd_path).expect("Failed to read RFD file");
    assert!(content.contains("title: Test RFD"));
    assert!(content.contains("Alice <alice@test.com>"));
    assert!(content.contains("state: draft"));
}

#[test]
fn test_create_with_json_output() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "JSON Test",
            "--author",
            "Bob <bob@test.com>",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{").and(predicate::str::ends_with("}\n")))
        .stdout(predicate::str::contains("\"id\""))
        .stdout(predicate::str::contains("\"title\""))
        .stdout(predicate::str::contains("\"path\""));
}

#[test]
fn test_list_empty_directory() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No RFDs found").or(predicate::str::contains("Found 0")));
}

#[test]
fn test_list_shows_created_rfds() {
    let temp_dir = setup_test_env();

    // Create two RFDs
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "First RFD",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Second RFD",
            "--author",
            "Bob <bob@test.com>",
        ])
        .assert()
        .success();

    // List should show both
    rfd_cmd(&temp_dir)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First RFD"))
        .stdout(predicate::str::contains("Second RFD"))
        .stdout(predicate::str::contains("Found 2"));
}

#[test]
fn test_list_with_json_output() {
    let temp_dir = setup_test_env();

    // Create an RFD
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // List as JSON
    rfd_cmd(&temp_dir)
        .args(["list", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains("\"rfds\""))
        .stdout(predicate::str::contains("\"total\""));
}

#[test]
fn test_show_rfd() {
    let temp_dir = setup_test_env();

    // Create an RFD
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Show Test",
            "--author",
            "Charlie <charlie@test.com>",
        ])
        .assert()
        .success();

    // Show the RFD
    rfd_cmd(&temp_dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("RFD 0001"))
        .stdout(predicate::str::contains("Show Test"))
        .stdout(predicate::str::contains("Charlie"));
}

#[test]
fn test_show_nonexistent_rfd() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args(["show", "999"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("NOT_FOUND")));
}

#[test]
fn test_status_transition_draft_to_review() {
    let temp_dir = setup_test_env();

    // Create an RFD (starts in draft)
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Status Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Transition to review
    rfd_cmd(&temp_dir)
        .args(["status", "1", "--set", "review"])
        .assert()
        .success()
        .stdout(predicate::str::contains("review").or(predicate::str::contains("Updated")));

    // Verify state changed (state is nested in metadata)
    rfd_cmd(&temp_dir)
        .args(["show", "1", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("review"));
}

#[test]
fn test_status_idempotent() {
    let temp_dir = setup_test_env();

    // Create an RFD
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Idempotent Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Set to review
    rfd_cmd(&temp_dir)
        .args(["status", "1", "--set", "review"])
        .assert()
        .success();

    // Set to review again (should succeed)
    rfd_cmd(&temp_dir)
        .args(["status", "1", "--set", "review"])
        .assert()
        .success()
        .stdout(predicate::str::contains("already").or(predicate::str::contains("review")));
}

#[test]
fn test_status_invalid_transition() {
    let temp_dir = setup_test_env();

    // Create an RFD (starts in draft)
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Invalid Transition Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Try invalid transition: draft -> archived (should fail)
    rfd_cmd(&temp_dir)
        .args(["status", "1", "--set", "archived"])
        .assert()
        .failure()
        .code(3) // State transition error
        .stderr(predicate::str::contains("transition").or(predicate::str::contains("Cannot")));
}

#[test]
fn test_validate_valid_rfd() {
    let temp_dir = setup_test_env();

    // Create an RFD
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Validate Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Validate should pass
    rfd_cmd(&temp_dir)
        .args(["validate", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn test_validate_with_json() {
    let temp_dir = setup_test_env();

    // Create an RFD
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Validate JSON Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Validate with JSON output
    rfd_cmd(&temp_dir)
        .args(["validate", "1", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains("\"valid\""))
        .stdout(predicate::str::contains("\"issues\""));
}

#[test]
fn test_update_metadata() {
    let temp_dir = setup_test_env();

    // Create an RFD
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Update Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Update title
    rfd_cmd(&temp_dir)
        .args(["update", "1", "--field", "title", "--value", "New Title"])
        .assert()
        .success();

    // Verify update
    rfd_cmd(&temp_dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("New Title"));
}

#[test]
fn test_filter_by_status() {
    let temp_dir = setup_test_env();

    // Create RFDs in different states
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Draft RFD",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Review RFD",
            "--author",
            "Bob <bob@test.com>",
        ])
        .assert()
        .success();

    // Move second to review
    rfd_cmd(&temp_dir)
        .args(["status", "2", "--set", "review"])
        .assert()
        .success();

    // Filter by draft
    rfd_cmd(&temp_dir)
        .args(["list", "--status", "draft"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Draft RFD"))
        .stdout(predicate::str::contains("Review RFD").not());

    // Filter by review
    rfd_cmd(&temp_dir)
        .args(["list", "--status", "review"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Review RFD"))
        .stdout(predicate::str::contains("Draft RFD").not());
}

#[test]
fn test_workflow_draft_to_implemented() {
    let temp_dir = setup_test_env();

    // Create RFD (draft)
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Workflow Test",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // draft -> review
    rfd_cmd(&temp_dir)
        .args(["status", "1", "--set", "review"])
        .assert()
        .success();

    // review -> accepted
    rfd_cmd(&temp_dir)
        .args(["status", "1", "--set", "accepted"])
        .assert()
        .success();

    // accepted -> implemented
    rfd_cmd(&temp_dir)
        .args(["status", "1", "--set", "implemented"])
        .assert()
        .success();

    // Verify final state (state is nested in metadata)
    rfd_cmd(&temp_dir)
        .args(["show", "1", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("implemented"));
}

// ============================================================================
// Search Command Integration Tests
// ============================================================================

#[test]
fn test_search_help() {
    let mut cmd = Command::cargo_bin("rfd").unwrap();
    cmd.args(["search", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Search RFDs by content"))
        .stdout(predicate::str::contains("--in"))
        .stdout(predicate::str::contains("--case-sensitive"));
}

#[test]
fn test_search_empty_query_error() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args(["search", ""])
        .assert()
        .failure()
        .stderr(predicate::str::contains("empty"));
}

#[test]
fn test_search_basic_finds_matching_title() {
    let temp_dir = setup_test_env();

    // Create RFDs with different titles
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Authentication System",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Database Design",
            "--author",
            "Bob <bob@test.com>",
        ])
        .assert()
        .success();

    // Search for "authentication"
    rfd_cmd(&temp_dir)
        .args(["search", "authentication"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Authentication System"))
        .stdout(predicate::str::contains("Database Design").not());
}

#[test]
fn test_search_case_insensitive_default() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "OAuth Integration",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Search with lowercase - should match "OAuth"
    rfd_cmd(&temp_dir)
        .args(["search", "oauth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OAuth Integration"));
}

#[test]
fn test_search_case_sensitive_flag() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "OAuth Integration",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Case-sensitive search with wrong case - should NOT match
    rfd_cmd(&temp_dir)
        .args(["search", "oauth", "--case-sensitive"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OAuth Integration").not());

    // Case-sensitive search with correct case - should match
    rfd_cmd(&temp_dir)
        .args(["search", "OAuth", "--case-sensitive"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OAuth Integration"));
}

#[test]
fn test_search_multiple_terms_and_logic() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "OAuth API Integration",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "OAuth System",
            "--author",
            "Bob <bob@test.com>",
        ])
        .assert()
        .success();

    // Search for "oauth api" - should only match first RFD
    rfd_cmd(&temp_dir)
        .args(["search", "oauth api"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OAuth API Integration"))
        .stdout(predicate::str::contains("OAuth System").not());
}

#[test]
fn test_search_scope_title() {
    let temp_dir = setup_test_env();

    // Create RFD and manually edit content to add "database" to body
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Authentication System",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Add content with "database" in body
    let rfd_path = temp_dir.path().join("rfds/0001-authentication-system.md");
    let content = fs::read_to_string(&rfd_path).unwrap();
    let new_content = content.replace(
        "# Summary",
        "# Summary\nThis system integrates with the database layer.",
    );
    fs::write(&rfd_path, new_content).unwrap();

    // Search only in title - should NOT find "database"
    rfd_cmd(&temp_dir)
        .args(["search", "database", "--in", "title"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Authentication System").not());

    // Search in content - should find "database"
    rfd_cmd(&temp_dir)
        .args(["search", "database", "--in", "content"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Authentication System"));
}

#[test]
fn test_search_with_status_filter() {
    let temp_dir = setup_test_env();

    // Create two RFDs
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Authentication API",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Authorization API",
            "--author",
            "Bob <bob@test.com>",
        ])
        .assert()
        .success();

    // Move second to review
    rfd_cmd(&temp_dir)
        .args(["status", "2", "--set", "review"])
        .assert()
        .success();

    // Search for "api" with status=draft filter
    rfd_cmd(&temp_dir)
        .args(["search", "api", "--status", "draft"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Authentication API"))
        .stdout(predicate::str::contains("Authorization API").not());

    // Search for "api" with status=review filter
    rfd_cmd(&temp_dir)
        .args(["search", "api", "--status", "review"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Authorization API"))
        .stdout(predicate::str::contains("Authentication API").not());
}

#[test]
fn test_search_with_author_filter() {
    let temp_dir = setup_test_env();

    // Create RFDs with different authors
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Security System",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Security Design",
            "--author",
            "Bob <bob@test.com>",
        ])
        .assert()
        .success();

    // Search for "security" with author filter
    rfd_cmd(&temp_dir)
        .args(["search", "security", "--author", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Security System"))
        .stdout(predicate::str::contains("Security Design").not());
}

#[test]
fn test_search_with_limit() {
    let temp_dir = setup_test_env();

    // Create three RFDs with "API" in title
    for i in 1..=3 {
        rfd_cmd(&temp_dir)
            .args([
                "create",
                "--title",
                &format!("API Design {}", i),
                "--author",
                "Alice <alice@test.com>",
            ])
            .assert()
            .success();
    }

    // Search with limit=2
    rfd_cmd(&temp_dir)
        .args(["search", "api", "--limit", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Found 2").or(predicate::str::contains("API Design")));
}

#[test]
fn test_search_json_output() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Test Search",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Search with JSON output
    rfd_cmd(&temp_dir)
        .args(["search", "test", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains("\"rfds\""))
        .stdout(predicate::str::contains("\"total\""))
        .stdout(predicate::str::contains("Test Search"));
}

#[test]
fn test_search_no_results() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Test RFD",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Search for term that doesn't exist
    rfd_cmd(&temp_dir)
        .args(["search", "nonexistent"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Found 0")
                .or(predicate::str::contains("No RFDs found"))
                .or(predicate::str::contains("Test RFD").not()),
        );
}

#[test]
fn test_search_invalid_scope() {
    let temp_dir = setup_test_env();

    rfd_cmd(&temp_dir)
        .args(["search", "test", "--in", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid search scope"));
}

#[test]
fn test_search_combined_filters() {
    let temp_dir = setup_test_env();

    // Create multiple RFDs
    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "API Security",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "API Design",
            "--author",
            "Bob <bob@test.com>",
        ])
        .assert()
        .success();

    rfd_cmd(&temp_dir)
        .args([
            "create",
            "--title",
            "Database API",
            "--author",
            "Alice <alice@test.com>",
        ])
        .assert()
        .success();

    // Move one to review
    rfd_cmd(&temp_dir)
        .args(["status", "3", "--set", "review"])
        .assert()
        .success();

    // Search: query="api" + author="alice" + status="review"
    rfd_cmd(&temp_dir)
        .args([
            "search", "api", "--author", "alice", "--status", "review", "--limit", "5",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Database API"))
        .stdout(predicate::str::contains("API Security").not())
        .stdout(predicate::str::contains("API Design").not());
}
