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

#[test]
fn test_single_commit_no_diff_available() {
    let commits = create_test_commits(1);
    let browser = HistoryBrowser::with_commits(commits);

    assert_eq!(browser.len(), 1);
    assert!(!browser.is_empty());
    assert!(browser.selected_commit().is_some());
    assert!(browser.get_diff_commits().is_none());
}

#[test]
fn test_search_query() {
    let mut commits = create_test_commits(5);
    commits[1].message = "Fix bug in parser".to_string();
    commits[2].message = "Add new feature".to_string();
    commits[3].author_name = "Alice Developer".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    assert!(browser.search_query().is_none());
    assert!(!browser.is_searching());

    browser.set_search_query(Some("bug".to_string()));
    assert_eq!(browser.search_query(), Some("bug"));
    assert!(browser.is_searching());
    assert_eq!(browser.match_count(), 1);
    assert!(browser.is_commit_visible(1));
    assert!(!browser.is_commit_visible(0));
}

#[test]
fn test_search_multiple_matches() {
    let mut commits = create_test_commits(10);
    commits[2].message = "Fix authentication bug".to_string();
    commits[5].message = "Fix rendering bug".to_string();
    commits[7].author_name = "Bug Hunter".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("bug".to_string()));
    assert_eq!(browser.match_count(), 3);
    assert!(browser.is_commit_visible(2));
    assert!(browser.is_commit_visible(5));
    assert!(browser.is_commit_visible(7));
    assert!(!browser.is_commit_visible(0));
}

#[test]
fn test_search_case_insensitive() {
    let mut commits = create_test_commits(3);
    commits[0].message = "Fix Bug in Parser".to_string();
    commits[1].message = "BUG: memory leak".to_string();
    commits[2].message = "Update documentation".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("bug".to_string()));
    assert_eq!(browser.match_count(), 2);
    assert!(browser.is_commit_visible(0));
    assert!(browser.is_commit_visible(1));
    assert!(!browser.is_commit_visible(2));
}

#[test]
fn test_clear_search() {
    let mut commits = create_test_commits(5);
    commits[1].message = "Fix bug".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("bug".to_string()));
    assert_eq!(browser.match_count(), 1);

    browser.clear_search();
    assert!(browser.search_query().is_none());
    assert!(!browser.is_searching());
    assert_eq!(browser.match_count(), 5);
    assert!(browser.is_commit_visible(0));
    assert!(browser.is_commit_visible(4));
}

#[test]
fn test_search_by_author() {
    let mut commits = create_test_commits(3);
    commits[1].author_name = "Alice Smith".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("alice".to_string()));
    assert_eq!(browser.match_count(), 1);
    assert!(browser.is_commit_visible(1));
}

#[test]
fn test_search_by_email() {
    let mut commits = create_test_commits(3);
    commits[2].author_email = "special@example.com".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("special".to_string()));
    assert_eq!(browser.match_count(), 1);
    assert!(browser.is_commit_visible(2));
}

#[test]
fn test_search_by_commit_id() {
    let commits = create_test_commits(5);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("commit_2".to_string()));
    assert_eq!(browser.match_count(), 1);
    assert!(browser.is_commit_visible(2));
}

#[test]
fn test_file_filter() {
    let mut commits = create_test_commits(5);
    commits[1].message = "Update main.rs".to_string();
    commits[3].message = "Fix bug in main.rs".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    assert!(browser.file_filter().is_none());
    assert!(!browser.is_file_filtering());

    browser.set_file_filter(Some("main.rs".to_string()));
    assert_eq!(browser.file_filter(), Some("main.rs"));
    assert!(browser.is_file_filtering());
    assert_eq!(browser.match_count(), 2);
    assert!(browser.is_commit_visible(1));
    assert!(browser.is_commit_visible(3));
    assert!(!browser.is_commit_visible(0));
}

#[test]
fn test_file_filter_case_insensitive() {
    let mut commits = create_test_commits(3);
    commits[0].message = "Update Main.rs".to_string();
    commits[1].message = "Fix MAIN.RS".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_file_filter(Some("main.rs".to_string()));
    assert_eq!(browser.match_count(), 2);
    assert!(browser.is_commit_visible(0));
    assert!(browser.is_commit_visible(1));
}

#[test]
fn test_clear_file_filter() {
    let mut commits = create_test_commits(5);
    commits[1].message = "Update main.rs".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_file_filter(Some("main.rs".to_string()));
    assert_eq!(browser.match_count(), 1);

    browser.clear_file_filter();
    assert!(browser.file_filter().is_none());
    assert!(!browser.is_file_filtering());
    assert_eq!(browser.match_count(), 5);
}

#[test]
fn test_combined_search_and_file_filter() {
    let mut commits = create_test_commits(6);
    commits[1].message = "Fix bug in main.rs".to_string();
    commits[2].message = "Update main.rs".to_string();
    commits[3].message = "Fix bug in utils.rs".to_string();
    commits[4].message = "Add feature to main.rs".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("fix".to_string()));
    browser.set_file_filter(Some("main.rs".to_string()));

    assert_eq!(browser.match_count(), 1);
    assert!(browser.is_commit_visible(1));
    assert!(!browser.is_commit_visible(2));
    assert!(!browser.is_commit_visible(3));
}

#[test]
fn test_toggle_side_by_side() {
    let mut browser = HistoryBrowser::new();

    assert_eq!(browser.diff_view_mode(), &DiffViewMode::FullDiff);
    assert!(!browser.is_side_by_side());

    browser.toggle_side_by_side();
    assert_eq!(browser.diff_view_mode(), &DiffViewMode::SideBySide);
    assert!(browser.is_side_by_side());

    browser.toggle_side_by_side();
    assert_eq!(browser.diff_view_mode(), &DiffViewMode::FullDiff);
    assert!(!browser.is_side_by_side());
}

#[test]
fn test_toggle_side_by_side_with_file() {
    let mut browser = HistoryBrowser::new();

    browser.set_diff_view_mode(DiffViewMode::FileDiff("test.rs".to_string()));
    assert!(!browser.is_side_by_side());

    browser.toggle_side_by_side();
    assert_eq!(
        browser.diff_view_mode(),
        &DiffViewMode::SideBySideFile("test.rs".to_string())
    );
    assert!(browser.is_side_by_side());

    browser.toggle_side_by_side();
    assert_eq!(
        browser.diff_view_mode(),
        &DiffViewMode::FileDiff("test.rs".to_string())
    );
    assert!(!browser.is_side_by_side());
}

#[test]
fn test_select_first() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.select_commit(5);
    assert_eq!(browser.selected_index(), 5);

    assert!(browser.select_first());
    assert_eq!(browser.selected_index(), 0);
}

#[test]
fn test_select_first_empty() {
    let mut browser = HistoryBrowser::new();
    assert!(!browser.select_first());
}

#[test]
fn test_select_last() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    assert_eq!(browser.selected_index(), 0);

    assert!(browser.select_last());
    assert_eq!(browser.selected_index(), 9);
}

#[test]
fn test_select_last_empty() {
    let mut browser = HistoryBrowser::new();
    assert!(!browser.select_last());
}

#[test]
fn test_page_up() {
    let commits = create_test_commits(50);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    browser.select_commit(25);
    assert_eq!(browser.selected_index(), 25);

    assert!(browser.page_up());
    assert_eq!(browser.selected_index(), 15);

    assert!(browser.page_up());
    assert_eq!(browser.selected_index(), 5);
}

#[test]
fn test_page_up_at_top() {
    let commits = create_test_commits(50);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    browser.select_commit(5);
    assert!(browser.page_up());
    assert_eq!(browser.selected_index(), 0);

    assert!(!browser.page_up());
    assert_eq!(browser.selected_index(), 0);
}

#[test]
fn test_page_down() {
    let commits = create_test_commits(50);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    assert_eq!(browser.selected_index(), 0);

    assert!(browser.page_down());
    assert_eq!(browser.selected_index(), 10);

    assert!(browser.page_down());
    assert_eq!(browser.selected_index(), 20);
}

#[test]
fn test_page_down_at_bottom() {
    let commits = create_test_commits(50);
    let mut browser = HistoryBrowser::with_commits(commits);
    browser.set_page_size(10);

    browser.select_commit(45);
    assert!(browser.page_down());
    assert_eq!(browser.selected_index(), 49);

    assert!(!browser.page_down());
    assert_eq!(browser.selected_index(), 49);
}

#[test]
fn test_filtered_commits() {
    let mut commits = create_test_commits(5);
    commits[1].message = "Fix bug".to_string();
    commits[3].message = "Fix error".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("fix".to_string()));

    let filtered = browser.filtered_commits();
    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].message, "Fix bug");
    assert_eq!(filtered[1].message, "Fix error");
}

#[test]
fn test_search_updates_selected_index() {
    let mut commits = create_test_commits(10);
    commits[5].message = "Important commit".to_string();

    let mut browser = HistoryBrowser::with_commits(commits);
    browser.select_commit(2);

    browser.set_search_query(Some("important".to_string()));
    assert_eq!(browser.selected_index(), 5);
}

#[test]
fn test_clear_resets_filters() {
    let commits = create_test_commits(10);
    let mut browser = HistoryBrowser::with_commits(commits);

    browser.set_search_query(Some("test".to_string()));
    browser.set_file_filter(Some("main.rs".to_string()));
    browser.set_base_commit(Some(5));

    browser.clear();

    assert_eq!(browser.len(), 0);
    assert!(browser.search_query().is_none());
    assert!(browser.file_filter().is_none());
    assert!(browser.base_commit_index().is_none());
}

#[test]
fn test_set_commits_resets_filters() {
    let commits1 = create_test_commits(5);
    let commits2 = create_test_commits(10);

    let mut browser = HistoryBrowser::with_commits(commits1);

    browser.set_search_query(Some("test".to_string()));
    browser.set_file_filter(Some("main.rs".to_string()));

    browser.set_commits(commits2);

    assert_eq!(browser.len(), 10);
    assert!(browser.search_query().is_none());
    assert!(browser.file_filter().is_none());
}
