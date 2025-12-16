use crate::CommitInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffViewMode {
    FullDiff,
    FileDiff(String),
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
        }
    }

    pub fn set_commits(&mut self, commits: Vec<CommitInfo>) {
        self.commits = commits;
        self.selected_index = 0;
        self.current_page = 0;
        self.selected_file = None;
        self.diff_view_mode = DiffViewMode::FullDiff;
        self.base_commit_index = None;
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
            DiffViewMode::FileDiff(ref file) => {
                self.selected_file = Some(file.clone());
            }
            DiffViewMode::FullDiff => {
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
    }
}

impl Default for HistoryBrowser {
    fn default() -> Self {
        Self::new()
    }
}
