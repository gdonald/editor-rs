#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorMode {
    #[default]
    Normal, // Standard modeless editing
    Insert, // Vim-style insert
    Visual, // Vim-style visual
}
