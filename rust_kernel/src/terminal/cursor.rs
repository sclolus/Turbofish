/// Simple and Basic implementation of cursor

/// Usable to select write position for characters
#[derive(Debug, Copy, Clone, Default)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Cursor {
    pub pos: Pos,
    pub nb_lines: usize,
    pub nb_columns: usize,
    pub visible: bool,
}

impl Cursor {
    /// Increment the cursor by one, return Option of line must be refreshed
    pub fn forward(&mut self) -> Option<usize> {
        self.pos.column += 1;
        if self.pos.column == self.nb_columns {
            self.cariage_return()
        } else {
            None
        }
    }
    /// Do a cariage_return, return Option of line must be refreshed
    pub fn cariage_return(&mut self) -> Option<usize> {
        let ret = Some(self.pos.line);

        self.pos.column = 0;
        if self.pos.line != self.nb_lines - 1 {
            self.pos.line += 1;
        }
        ret
    }
    /// Decrement the cursor by one
    pub fn backward(&mut self) -> Option<usize> {
        if self.pos.column == 0 {
            self.pos.column = self.nb_columns - 1;
            if self.pos.line != 0 {
                self.pos.line -= 1;
            }
        } else {
            self.pos.column -= 1;
        }
        None
    }
}
