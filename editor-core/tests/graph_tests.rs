use editor_core::{format_graph_line, generate_commit_graph, CommitInfo};

fn create_test_commit(id: &str, message: &str, annotation: Option<String>) -> CommitInfo {
    CommitInfo {
        id: id.to_string(),
        author_name: "Test Author".to_string(),
        author_email: "test@example.com".to_string(),
        timestamp: 1234567890,
        message: message.to_string(),
        annotation,
    }
}

#[test]
fn test_generate_commit_graph_empty() {
    let commits = vec![];
    let graph = generate_commit_graph(&commits);
    assert_eq!(graph.len(), 0);
}

#[test]
fn test_generate_commit_graph_single_commit() {
    let commits = vec![create_test_commit("abc123", "Initial commit", None)];
    let graph = generate_commit_graph(&commits);
    assert_eq!(graph.len(), 1);
    assert_eq!(graph[0].graph_line, "* ");
    assert_eq!(graph[0].level, 0);
    assert_eq!(graph[0].commit.message, "Initial commit");
}

#[test]
fn test_generate_commit_graph_multiple_commits() {
    let commits = vec![
        create_test_commit("abc123", "Third commit", None),
        create_test_commit("def456", "Second commit", None),
        create_test_commit("ghi789", "First commit", None),
    ];
    let graph = generate_commit_graph(&commits);
    assert_eq!(graph.len(), 3);
    assert_eq!(graph[0].graph_line, "* ");
    assert_eq!(graph[1].graph_line, "* ");
    assert_eq!(graph[2].graph_line, "* ");
}

#[test]
fn test_format_graph_line_simple() {
    let commit = create_test_commit("abcdef1234567890", "Test commit message", None);
    let graph = generate_commit_graph(&[commit]);
    let lines = format_graph_line(&graph[0], false);

    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "* abcdef1 Test commit message");
}

#[test]
fn test_format_graph_line_with_annotation() {
    let commit = create_test_commit(
        "abcdef1234567890",
        "Test commit message",
        Some("This is a note".to_string()),
    );
    let graph = generate_commit_graph(&[commit]);
    let lines = format_graph_line(&graph[0], true);

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "* abcdef1 Test commit message");
    assert_eq!(lines[1], "    📝 This is a note");
}

#[test]
fn test_format_graph_line_without_showing_annotation() {
    let commit = create_test_commit(
        "abcdef1234567890",
        "Test commit message",
        Some("This is a note".to_string()),
    );
    let graph = generate_commit_graph(&[commit]);
    let lines = format_graph_line(&graph[0], false);

    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "* abcdef1 Test commit message");
}

#[test]
fn test_format_graph_line_multiline_message() {
    let commit = create_test_commit(
        "abcdef1234567890",
        "First line\nSecond line\nThird line",
        None,
    );
    let graph = generate_commit_graph(&[commit]);
    let lines = format_graph_line(&graph[0], false);

    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "* abcdef1 First line");
}

#[test]
fn test_format_graph_line_short_commit_id() {
    let commit = create_test_commit("abc", "Test", None);
    let graph = generate_commit_graph(&[commit]);
    let lines = format_graph_line(&graph[0], false);

    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "* abc Test");
}

#[test]
fn test_graph_preserves_commit_info() {
    let commits = vec![
        create_test_commit("abc123", "Commit 1", Some("Note 1".to_string())),
        create_test_commit("def456", "Commit 2", None),
        create_test_commit("ghi789", "Commit 3", Some("Note 3".to_string())),
    ];

    let graph = generate_commit_graph(&commits);

    assert_eq!(graph[0].commit.id, "abc123");
    assert_eq!(graph[0].commit.message, "Commit 1");
    assert_eq!(graph[0].commit.annotation, Some("Note 1".to_string()));

    assert_eq!(graph[1].commit.id, "def456");
    assert_eq!(graph[1].commit.message, "Commit 2");
    assert_eq!(graph[1].commit.annotation, None);

    assert_eq!(graph[2].commit.id, "ghi789");
    assert_eq!(graph[2].commit.message, "Commit 3");
    assert_eq!(graph[2].commit.annotation, Some("Note 3".to_string()));
}
