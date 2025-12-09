use crate::editor::EditorState;
use crate::error::Result;

pub trait EditorView {
    fn render(&mut self, state: &EditorState) -> Result<()>;

    fn set_status_message(&mut self, message: String);

    fn get_viewport_height(&self) -> usize;

    fn get_viewport_width(&self) -> usize;
}
