use git_workflow::domain::{
    CommitOptions, CommitType, FileChange, FileStatus, GitWorkflowError, GitWorkflowService,
};
use std::path::PathBuf;

#[test]
fn test_commit_plan_creation() {
    let service = GitWorkflowService::new();

    let options = CommitOptions {
        commit_type: Some(CommitType::Feat),
        scope: Some("auth".to_string()),
        subject: "add OAuth support".to_string(),
        body: Some("Implements OAuth 2.0 flow".to_string()),
        breaking: false,
        issues: vec!["123".to_string()],
        staged_files: vec![],
    };

    let result = service.prepare_commit(options);
    assert!(result.is_ok());

    let plan = result.unwrap();
    assert_eq!(plan.commit_type, CommitType::Feat);
    assert_eq!(plan.scope, Some("auth".to_string()));
    assert!(plan
        .formatted_message
        .starts_with("feat(auth): add OAuth support"));
    assert!(plan.formatted_message.contains("Closes #123"));
}

#[test]
fn test_breaking_change_formatting() {
    let service = GitWorkflowService::new();

    let options = CommitOptions {
        commit_type: Some(CommitType::Feat),
        scope: None,
        subject: "change API".to_string(),
        body: Some("API endpoints have changed".to_string()),
        breaking: true,
        issues: vec![],
        staged_files: vec![],
    };

    let result = service.prepare_commit(options);
    assert!(result.is_ok());

    let plan = result.unwrap();
    assert!(plan.formatted_message.contains("BREAKING CHANGE:"));
}

#[test]
fn test_commit_validation() {
    let service = GitWorkflowService::new();

    // Valid conventional commit
    match service.validate_commit_message("feat: add new feature") {
        Ok(valid) => assert!(valid),
        Err(_) => panic!("Expected Ok"),
    }

    // Invalid commit
    match service.validate_commit_message("Added new feature") {
        Ok(valid) => assert!(!valid),
        Err(_) => panic!("Expected Ok"),
    }

    // Empty message
    match service.validate_commit_message("") {
        Ok(valid) => assert!(!valid),
        Err(_) => panic!("Expected Ok"),
    }
}

#[test]
fn test_change_analysis() {
    let service = GitWorkflowService::new();

    let files = vec![
        FileChange {
            path: PathBuf::from("README.md"),
            status: FileStatus::Modified,
            additions: 10,
            deletions: 5,
        },
        FileChange {
            path: PathBuf::from("tests/test_auth.rs"),
            status: FileStatus::Added,
            additions: 100,
            deletions: 0,
        },
        FileChange {
            path: PathBuf::from(".github/workflows/ci.yml"),
            status: FileStatus::Modified,
            additions: 20,
            deletions: 10,
        },
    ];

    let result = service.analyze_changes(files);
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.groups.len(), 3); // Docs, Tests, CI/CD

    // Check that groups have correct types
    let doc_group = analysis.groups.iter().find(|g| g.name == "Documentation");
    assert!(doc_group.is_some());
    assert_eq!(doc_group.unwrap().suggested_type, CommitType::Docs);
}

#[test]
fn test_empty_changes_error() {
    let service = GitWorkflowService::new();

    let result = service.analyze_changes(vec![]);
    assert!(result.is_err());

    match result.unwrap_err() {
        GitWorkflowError::NoChanges => {}
        _ => panic!("Expected NoChanges error"),
    }
}
