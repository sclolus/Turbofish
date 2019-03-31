//! Ansi cursor move
use super::CSI;
use crate::{terminal, terminal::Pos};
use core::{fmt, fmt::Display};

#[derive(Copy, Clone, Debug)]
pub enum CursorMove {
    ///Moves the cursor n (default 1) cells in the given direction. If the cursor is already at the edge of the screen, this has no effect.
    Up(usize),
    Down(usize),
    Forward(usize),
    Backward(usize),

    ///Moves the cursor to column n
    HorizontalAbsolute(usize),

    ///Moves the cursor to row n, column m. The values are 1-based, and default to 1 (top left corner) if omitted. A sequence such as CSI ;5H is a synonym for CSI 1;5H as well as CSI 17;H is the same as CSI 17H and CSI 17;1H
    Pos(Pos),
}

impl Display for CursorMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CursorMove::*;
        match self {
            Up(x) => write!(f, "{}{}A", CSI, x),
            Down(x) => write!(f, "{}{}B", CSI, x),
            Forward(x) => write!(f, "{}{}C", CSI, x),
            Backward(x) => write!(f, "{}{}D", CSI, x),
            HorizontalAbsolute(x) => write!(f, "{}{}G", CSI, x),
            Pos(terminal::Pos { line, column }) => write!(f, "{}{};{}H", CSI, line, column),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cursor() {
        println!("{}", CursorMove::Pos(Pos { line: 3, column: 3 }));
        println!("{}", CursorMove::Pos(Pos { line: 1, column: 1 }));
        println!("{}", CursorMove::Forward(10));
    }
}
