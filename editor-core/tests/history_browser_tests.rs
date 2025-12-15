use editor_core::{CommitInfo, DiffViewMode, HistoryBrowser};

fn create_test_commits(count: usize) -> Vec<CommitInfo> {
    (0..count)
        .map(|i| CommitInfo {
            id: format!("commit_{}", i),
            author_name: "Test Author".to_string(),
            author_email: "test@example.com".to_string(),
            timestamp: 1000000 + i as i64,
            message: format!("Commit message {}", i),
        })
        .collect()
}

#[test]
fn test_history_browser_new() {
    let browser = HistoryBrowser::new();
    assert!(browser.is_empty());
    assert_eq!(browser.len(), 0);
    assert_eq!(browser.selected_index(), 0);
    assert_eq!(browser.current_page(), 0);
    assert_eq!(browser.page_size(), 20);
    assert!(browser.selected_file().is_none());
    assert_eq!(browser.diff_view_mode(), &DiffViewMode::FullDiff);
    assert!(browser.base_commit_index().is_none());
}

#[test]
fn test_history_browser_default() {
    let browser = HistoryBrowser::default();
    assert!(browser.is_empty());
}

#[test]
fn test_history_browser_with_commits() {
    let commits = create_test_commits(5);
    let browser = HistoryBrowser::with_commits(commits.clone());

    assert_eq!(browser.len(), 5);
    assert!(!browser.is_empty());
    assert_eq!(browser.commits().len(), 5);
    assert_eq!(browser.selected_index(), 0);
}

#[test]
fn test_set_commits() {
    let mut browser = HistoryBrowser::new();
    assert!(browser.is_empty());

    let commits = create_test_commits(10);
    browser.set_commits(commits);

    assert_eq!(browser.len(), 10);
    assert_eq!(browser.selected_index(), 0);
    assert_eq!(browser.current_page(), 0);
}

#[test]
fn test_selected_commit() {
    let commits = create_test_commits(5);
    let browser = HistoryBrowser::with_commits(commits);

    let selected = browser.selected_commit().unwrap();
    assert_eq!(selected.id, "commit_0");
}

#[test]
fn test_selected_commit_empty() {
    let browser = HistoryBrowser::new();
    assert!(browser.selected_commit().is_none());
}

#[test]
fn test_select_next() {
    let commits = create_test_commits(5);
    let mut browser = HistoryBrowser::with_commits(commits);

    assert_eq!(browser.selected_index(), 0);

    assert!(browser.select_next());
    assert_eq!(browser.selected_index(), 1);

    assert!(browser.select_next());
    assert_eq!(browser.selected_index(), 2);
}

#[test]
fn test_select_next_at_end() {
    let commits = create_test_commits(3);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(2);
    assert!(!browser.select_next());
    assert_eq!(browser.selected_index(), 2);
}

#[test]
fn test_select_previous() {
    let commits = create_test_commits(5);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(3);
    assert_eq!(browser.selected_index(), 3);

    assert!(browser.select_previous());
    assert_eq!(browser.selected_index(), 2);

    assert!(browser.select_previous());
    assert_eq!(browser.selected_index(), 1);
}

#[test]
fn test_select_previous_at_start() {
    let commits = create_test_commits(3);
    let mut browser = HistoryBrowser::with_commits(commits);

    assert!(!browser.select_previous());
    assert_eq!(browser.selected_index(), 0);
}

#[test]
fn test_select_commit() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    assert!(browser.select_commit(5));
    assert_eq!(browser.selected_index(), 5);
    assert_eq!(browser.selected_commit().unwrap().id, "commit_5");
}

#[test]
fn test_select_commit_invalid() {
    let commits = create_test_commits(5);
    let mut browser = HistoryBrowser::with_commits(commits);

    assert!(!browser.select_commit(10));
    assert_eq!(browser.selected_index(), 0);
}

#[test]
fn test_pagination_page_size() {
    let mut browser = HistoryBrowser::new();
    assert_eq!(browser.page_size(), 20);

    browser.set_page_size(10);
    assert_eq!(browser.page_size(), 10);
}

#[test]
fn test_pagination_page_size_zero_ignored() {
    let mut browser = HistoryBrowser::new();
    browser.set_page_size(10);
    browser.set_page_size(0);
    assert_eq!(browser.page_size(), 10);
}

#[test]
fn test_pagination_total_pages() {
    let commits = create_test_commits(25);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_page_size(10);
    assert_eq!(browser.total_pages(), 3);
}

#[test]
fn test_pagination_total_pages_exact() {
    let commits = create_test_commits(30);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_page_size(10);
    assert_eq!(browser.total_pages(), 3);
}

#[test]
fn test_pagination_total_pages_empty() {
    let browser = HistoryBrowser::new();
    assert_eq!(browser.total_pages(), 0);
}

#[test]
fn test_pagination_page_commits() {
    let commits = create_test_commits(25);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    let page_commits = browser.page_commits();
    assert_eq!(page_commits.len(), 10);
    assert_eq!(page_commits[0].id, "commit_0");
    assert_eq!(page_commits[9].id, "commit_9");
}

#[test]
fn test_pagination_next_page() {
    let commits = create_test_commits(25);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    assert_eq!(browser.current_page(), 0);

    assert!(browser.next_page());
    assert_eq!(browser.current_page(), 1);
    assert_eq!(browser.selected_index(), 10);

    let page_commits = browser.page_commits();
    assert_eq!(page_commits[0].id, "commit_10");
}

#[test]
fn test_pagination_next_page_at_end() {
    let commits = create_test_commits(25);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    browser.next_page();
    browser.next_page();

    assert!(!browser.next_page());
    assert_eq!(browser.current_page(), 2);
}

#[test]
fn test_pagination_previous_page() {
    let commits = create_test_commits(25);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    browser.next_page();
    browser.next_page();
    assert_eq!(browser.current_page(), 2);

    assert!(browser.previous_page());
    assert_eq!(browser.current_page(), 1);
    assert_eq!(browser.selected_index(), 10);
}

#[test]
fn test_pagination_previous_page_at_start() {
    let commits = create_test_commits(25);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    assert!(!browser.previous_page());
    assert_eq!(browser.current_page(), 0);
}

#[test]
fn test_pagination_auto_update_on_select() {
    let commits = create_test_commits(50);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    browser.select_commit(25);
    assert_eq!(browser.current_page(), 2);
}

#[test]
fn test_selected_file() {
    let mut browser = HistoryBrowser::new();
    assert!(browser.selected_file().is_none());

    browser.set_selected_file(Some("test.txt".to_string()));
    assert_eq!(browser.selected_file(), Some("test.txt"));
}

#[test]
fn test_clear_selected_file() {
    let mut browser = HistoryBrowser::new();
    browser.set_selected_file(Some("test.txt".to_string()));

    browser.clear_selected_file();
    assert!(browser.selected_file().is_none());
}

#[test]
fn test_diff_view_mode() {
    let mut browser = HistoryBrowser::new();
    assert_eq!(browser.diff_view_mode(), &DiffViewMode::FullDiff);

    browser.set_diff_view_mode(DiffViewMode::FileDiff("test.txt".to_string()));
    assert_eq!(
        browser.diff_view_mode(),
        &DiffViewMode::FileDiff("test.txt".to_string())
    );
}

#[test]
fn test_diff_view_mode_updates_selected_file() {
    let mut browser = HistoryBrowser::new();

    browser.set_diff_view_mode(DiffViewMode::FileDiff("test.txt".to_string()));
    assert_eq!(browser.selected_file(), Some("test.txt"));

    browser.set_diff_view_mode(DiffViewMode::FullDiff);
    assert!(browser.selected_file().is_none());
}

#[test]
fn test_set_selected_file_updates_diff_mode() {
    let mut browser = HistoryBrowser::new();

    browser.set_selected_file(Some("test.txt".to_string()));
    assert_eq!(
        browser.diff_view_mode(),
        &DiffViewMode::FileDiff("test.txt".to_string())
    );

    browser.set_selected_file(None);
    assert_eq!(browser.diff_view_mode(), &DiffViewMode::FullDiff);
}

#[test]
fn test_base_commit() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    assert!(browser.base_commit_index().is_none());
    assert!(browser.base_commit().is_none());

    assert!(browser.set_base_commit(Some(5)));
    assert_eq!(browser.base_commit_index(), Some(5));
    assert_eq!(browser.base_commit().unwrap().id, "commit_5");
}

#[test]
fn test_base_commit_invalid() {
    let commits = create_test_commits(5);
    let mut browser = HistoryBrowser::with_commits(commits);

    assert!(!browser.set_base_commit(Some(10)));
    assert!(browser.base_commit_index().is_none());
}

#[test]
fn test_clear_base_commit() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_base_commit(Some(5));
    assert!(browser.base_commit_index().is_some());

    assert!(browser.set_base_commit(None));
    assert!(browser.base_commit_index().is_none());
}

#[test]
fn test_get_diff_commits_with_base() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(2);
    browser.set_base_commit(Some(5));

    let (from, to) = browser.get_diff_commits().unwrap();
    assert_eq!(from.id, "commit_5");
    assert_eq!(to.id, "commit_2");
}

#[test]
fn test_get_diff_commits_without_base() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(2);

    let (from, to) = browser.get_diff_commits().unwrap();
    assert_eq!(from.id, "commit_3");
    assert_eq!(to.id, "commit_2");
}

#[test]
fn test_get_diff_commits_at_end() {
    let commits = create_test_commits(5);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(4);

    assert!(browser.get_diff_commits().is_none());
}

#[test]
fn test_get_diff_commits_empty() {
    let browser = HistoryBrowser::new();
    assert!(browser.get_diff_commits().is_none());
}

#[test]
fn test_clear() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(5);
    browser.set_selected_file(Some("test.txt".to_string()));
    browser.set_base_commit(Some(3));

    browser.clear();

    assert!(browser.is_empty());
    assert_eq!(browser.selected_index(), 0);
    assert_eq!(browser.current_page(), 0);
    assert!(browser.selected_file().is_none());
    assert_eq!(browser.diff_view_mode(), &DiffViewMode::FullDiff);
    assert!(browser.base_commit_index().is_none());
}

#[test]
fn test_set_commits_resets_state() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(5);
    browser.set_selected_file(Some("test.txt".to_string()));
    browser.set_base_commit(Some(3));
    browser.set_page_size(5);
    browser.next_page();

    let new_commits = create_test_commits(20);
    browser.set_commits(new_commits);

    assert_eq!(browser.len(), 20);
    assert_eq!(browser.selected_index(), 0);
    assert_eq!(browser.current_page(), 0);
    assert!(browser.selected_file().is_none());
    assert_eq!(browser.diff_view_mode(), &DiffViewMode::FullDiff);
    assert!(browser.base_commit_index().is_none());
    assert_eq!(browser.page_size(), 5);
}

#[test]
fn test_page_commits_empty_browser() {
    let browser = HistoryBrowser::new();
    assert_eq!(browser.page_commits().len(), 0);
}

#[test]
fn test_page_commits_partial_last_page() {
    let commits = create_test_commits(25);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    browser.next_page();
    browser.next_page();

    let page_commits = browser.page_commits();
    assert_eq!(page_commits.len(), 5);
    assert_eq!(page_commits[0].id, "commit_20");
    assert_eq!(page_commits[4].id, "commit_24");
}
