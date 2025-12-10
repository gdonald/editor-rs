use crate::cursor::CursorPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Normal,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub anchor: CursorPosition,
    pub cursor: CursorPosition,
    pub mode: SelectionMode,
}

impl Selection {
    pub fn new(anchor: CursorPosition, cursor: CursorPosition) -> Self {
        Self {
            anchor,
            cursor,
            mode: SelectionMode::Normal,
        }
    }

    pub fn new_block(anchor: CursorPosition, cursor: CursorPosition) -> Self {
        Self {
            anchor,
            cursor,
            mode: SelectionMode::Block,
        }
    }

    pub fn start(&self) -> CursorPosition {
        if self.anchor.line < self.cursor.line
            || (self.anchor.line == self.cursor.line && self.anchor.column < self.cursor.column)
        {
            self.anchor
        } else {
            self.cursor
        }
    }

    pub fn end(&self) -> CursorPosition {
        if self.anchor.line > self.cursor.line
            || (self.anchor.line == self.cursor.line && self.anchor.column > self.cursor.column)
        {
            self.anchor
        } else {
            self.cursor
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.cursor
    }

    pub fn is_block(&self) -> bool {
        self.mode == SelectionMode::Block
    }

    pub fn contains(&self, pos: CursorPosition) -> bool {
        if self.is_empty() {
            return false;
        }

        match self.mode {
            SelectionMode::Normal => {
                let start = self.start();
                let end = self.end();

                if pos.line < start.line || pos.line > end.line {
                    return false;
                }

                if pos.line == start.line && pos.column < start.column {
                    return false;
                }

                if pos.line == end.line && pos.column > end.column {
                    return false;
                }

                true
            }
            SelectionMode::Block => {
                let start = self.start();
                let end = self.end();

                pos.line >= start.line
                    && pos.line <= end.line
                    && pos.column >= start.column.min(end.column)
                    && pos.column <= start.column.max(end.column)
            }
        }
    }
}
