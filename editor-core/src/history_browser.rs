use crate::git_history::{format_graph_line, generate_commit_graph, CommitGraphNode};
use crate::CommitInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffViewMode {
    FullDiff,
    FileDiff(String),
    SideBySide,
    SideBySideFile(String),
}

#[derive(Debug, Clone)]
pub struct HistoryBrowser {
    commits: Vec<CommitInfo>,
    selected_index: usize,
    page_size: usize,
    current_page: usize,
    selected_file: Option<String>,
    diff_view_mode: DiffViewMode,
    base_commit_index: Option<usize>,
    search_query: Option<String>,
    filtered_indices: Option<Vec<usize>>,
    file_filter: Option<String>,
}

impl HistoryBrowser {
    pub fn new() -> Self {
        Self {
            commits: Vec::new(),
            selected_index: 0,
            page_size: 20,
            current_page: 0,
            selected_file: None,
            diff_view_mode: DiffViewMode::FullDiff,
            base_commit_index: None,
            search_query: None,
            filtered_indices: None,
            file_filter: None,
        }
    }

    pub fn with_commits(commits: Vec<CommitInfo>) -> Self {
        Self {
            commits,
            selected_index: 0,
            page_size: 20,
            current_page: 0,
            selected_file: None,
            diff_view_mode: DiffViewMode::FullDiff,
            base_commit_index: None,
            search_query: None,
            filtered_indices: None,
            file_filter: None,
        }
    }

    pub fn set_commits(&mut self, commits: Vec<CommitInfo>) {
        self.commits = commits;
        self.selected_index = 0;
        self.current_page = 0;
        self.selected_file = None;
        self.diff_view_mode = DiffViewMode::FullDiff;
        self.base_commit_index = None;
        self.search_query = None;
        self.filtered_indices = None;
        self.file_filter = None;
    }

    pub fn commits(&self) -> &[CommitInfo] {
        &self.commits
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn selected_commit(&self) -> Option<&CommitInfo> {
        self.commits.get(self.selected_index)
    }

    pub fn select_next(&mut self) -> bool {
        if self.selected_index + 1 < self.commits.len() {
            self.selected_index += 1;
            self.update_current_page();
            true
        } else {
            false
        }
    }

    pub fn select_previous(&mut self) -> bool {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.update_current_page();
            true
        } else {
            false
        }
    }

    pub fn select_commit(&mut self, index: usize) -> bool {
        if index < self.commits.len() {
            self.selected_index = index;
            self.update_current_page();
            true
        } else {
            false
        }
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn set_page_size(&mut self, page_size: usize) {
        if page_size > 0 {
            self.page_size = page_size;
            self.update_current_page();
        }
    }

    pub fn current_page(&self) -> usize {
        self.current_page
    }

    pub fn total_pages(&self) -> usize {
        if self.commits.is_empty() || self.page_size == 0 {
            0
        } else {
            self.commits.len().div_ceil(self.page_size)
        }
    }

    pub fn page_commits(&self) -> &[CommitInfo] {
        if self.commits.is_empty() {
            return &[];
        }

        let start = self.current_page * self.page_size;
        let end = (start + self.page_size).min(self.commits.len());

        &self.commits[start..end]
    }

    pub fn next_page(&mut self) -> bool {
        let total_pages = self.total_pages();
        if total_pages > 0 && self.current_page + 1 < total_pages {
            self.current_page += 1;
            self.selected_index = self.current_page * self.page_size;
            true
        } else {
            false
        }
    }

    pub fn previous_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.selected_index = self.current_page * self.page_size;
            true
        } else {
            false
        }
    }

    fn update_current_page(&mut self) {
        if self.page_size > 0 {
            self.current_page = self.selected_index / self.page_size;
        }
    }

    pub fn selected_file(&self) -> Option<&str> {
        self.selected_file.as_deref()
    }

    pub fn set_selected_file(&mut self, file: Option<String>) {
        self.selected_file = file.clone();
        if let Some(f) = file {
            self.diff_view_mode = DiffViewMode::FileDiff(f);
        } else {
            self.diff_view_mode = DiffViewMode::FullDiff;
        }
    }

    pub fn clear_selected_file(&mut self) {
        self.selected_file = None;
        self.diff_view_mode = DiffViewMode::FullDiff;
    }

    pub fn diff_view_mode(&self) -> &DiffViewMode {
        &self.diff_view_mode
    }

    pub fn set_diff_view_mode(&mut self, mode: DiffViewMode) {
        self.diff_view_mode = mode.clone();
        match mode {
            DiffViewMode::FileDiff(ref file) | DiffViewMode::SideBySideFile(ref file) => {
                self.selected_file = Some(file.clone());
            }
            DiffViewMode::FullDiff | DiffViewMode::SideBySide => {
                self.selected_file = None;
            }
        }
    }

    pub fn base_commit_index(&self) -> Option<usize> {
        self.base_commit_index
    }

    pub fn base_commit(&self) -> Option<&CommitInfo> {
        self.base_commit_index.and_then(|idx| self.commits.get(idx))
    }

    pub fn set_base_commit(&mut self, index: Option<usize>) -> bool {
        if let Some(idx) = index {
            if idx < self.commits.len() {
                self.base_commit_index = Some(idx);
                true
            } else {
                false
            }
        } else {
            self.base_commit_index = None;
            true
        }
    }

    pub fn get_diff_commits(&self) -> Option<(&CommitInfo, &CommitInfo)> {
        let to_commit = self.selected_commit()?;

        if let Some(base_idx) = self.base_commit_index {
            let from_commit = self.commits.get(base_idx)?;
            Some((from_commit, to_commit))
        } else {
            let from_idx = self.selected_index + 1;
            if from_idx >= self.commits.len() {
                return None;
            }
            let from_commit = self.commits.get(from_idx)?;
            Some((from_commit, to_commit))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.commits.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commits.len()
    }

    pub fn clear(&mut self) {
        self.commits.clear();
        self.selected_index = 0;
        self.current_page = 0;
        self.selected_file = None;
        self.diff_view_mode = DiffViewMode::FullDiff;
        self.base_commit_index = None;
        self.search_query = None;
        self.filtered_indices = None;
        self.file_filter = None;
    }

    pub fn toggle_side_by_side(&mut self) {
        self.diff_view_mode = match &self.diff_view_mode {
            DiffViewMode::FullDiff => DiffViewMode::SideBySide,
            DiffViewMode::FileDiff(file) => DiffViewMode::SideBySideFile(file.clone()),
            DiffViewMode::SideBySide => DiffViewMode::FullDiff,
            DiffViewMode::SideBySideFile(file) => DiffViewMode::FileDiff(file.clone()),
        };
    }

    pub fn is_side_by_side(&self) -> bool {
        matches!(
            self.diff_view_mode,
            DiffViewMode::SideBySide | DiffViewMode::SideBySideFile(_)
        )
    }

    pub fn select_first(&mut self) -> bool {
        if !self.commits.is_empty() {
            self.selected_index = 0;
            self.update_current_page();
            true
        } else {
            false
        }
    }

    pub fn select_last(&mut self) -> bool {
        if !self.commits.is_empty() {
            self.selected_index = self.commits.len() - 1;
            self.update_current_page();
            true
        } else {
            false
        }
    }

    pub fn page_up(&mut self) -> bool {
        if self.selected_index >= self.page_size {
            self.selected_index -= self.page_size;
            self.update_current_page();
            true
        } else if self.selected_index > 0 {
            self.selected_index = 0;
            self.update_current_page();
            true
        } else {
            false
        }
    }

    pub fn page_down(&mut self) -> bool {
        let new_index = self.selected_index + self.page_size;
        if new_index < self.commits.len() {
            self.selected_index = new_index;
            self.update_current_page();
            true
        } else if self.selected_index < self.commits.len() - 1 {
            self.selected_index = self.commits.len() - 1;
            self.update_current_page();
            true
        } else {
            false
        }
    }

    pub fn search_query(&self) -> Option<&str> {
        self.search_query.as_deref()
    }

    pub fn set_search_query(&mut self, query: Option<String>) {
        self.search_query = query.clone();
        self.apply_filters();
    }

    pub fn clear_search(&mut self) {
        self.search_query = None;
        self.apply_filters();
    }

    pub fn is_searching(&self) -> bool {
        self.search_query.is_some() && !self.search_query.as_ref().unwrap().is_empty()
    }

    pub fn filtered_commits(&self) -> Vec<&CommitInfo> {
        if let Some(indices) = &self.filtered_indices {
            indices
                .iter()
                .filter_map(|&i| self.commits.get(i))
                .collect()
        } else {
            self.commits.iter().collect()
        }
    }

    pub fn is_commit_visible(&self, index: usize) -> bool {
        if let Some(indices) = &self.filtered_indices {
            indices.contains(&index)
        } else {
            true
        }
    }

    pub fn match_count(&self) -> usize {
        if let Some(indices) = &self.filtered_indices {
            indices.len()
        } else {
            self.commits.len()
        }
    }

    pub fn file_filter(&self) -> Option<&str> {
        self.file_filter.as_deref()
    }

    pub fn set_file_filter(&mut self, filter: Option<String>) {
        self.file_filter = filter;
        self.apply_filters();
    }

    pub fn clear_file_filter(&mut self) {
        self.file_filter = None;
        self.apply_filters();
    }

    pub fn is_file_filtering(&self) -> bool {
        self.file_filter.is_some() && !self.file_filter.as_ref().unwrap().is_empty()
    }

    fn apply_filters(&mut self) {
        let has_search =
            self.search_query.is_some() && !self.search_query.as_ref().unwrap().is_empty();
        let has_file_filter =
            self.file_filter.is_some() && !self.file_filter.as_ref().unwrap().is_empty();

        if !has_search && !has_file_filter {
            self.filtered_indices = None;
            return;
        }

        let mut indices = Vec::new();

        for (idx, commit) in self.commits.iter().enumerate() {
            let mut matches = true;

            if has_search {
                let q_lower = self.search_query.as_ref().unwrap().to_lowercase();
                matches = commit.message.to_lowercase().contains(&q_lower)
                    || commit.author_name.to_lowercase().contains(&q_lower)
                    || commit.author_email.to_lowercase().contains(&q_lower)
                    || commit.id.to_lowercase().contains(&q_lower);
            }

            if matches && has_file_filter {
                let file_pattern = self.file_filter.as_ref().unwrap().to_lowercase();
                matches = commit.message.to_lowercase().contains(&file_pattern);
            }

            if matches {
                indices.push(idx);
            }
        }

        self.filtered_indices = Some(indices);

        if !self.filtered_indices.as_ref().unwrap().is_empty() {
            self.selected_index = self.filtered_indices.as_ref().unwrap()[0];
            self.update_current_page();
        }
    }

    pub fn get_commit_graph(&self) -> Vec<CommitGraphNode> {
        generate_commit_graph(&self.commits)
    }

    pub fn format_commit_line(&self, index: usize, show_annotation: bool) -> Option<Vec<String>> {
        let graph = self.get_commit_graph();
        graph
            .get(index)
            .map(|node| format_graph_line(node, show_annotation))
    }

    pub fn format_all_commit_lines(&self, show_annotations: bool) -> Vec<Vec<String>> {
        let graph = self.get_commit_graph();
        graph
            .iter()
            .map(|node| format_graph_line(node, show_annotations))
            .collect()
    }
}

impl Default for HistoryBrowser {
    fn default() -> Self {
        Self::new()
    }
}
