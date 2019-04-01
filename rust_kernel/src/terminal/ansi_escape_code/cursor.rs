//! Ansi cursor move
use super::CSI;
use crate::{terminal, terminal::Pos};
use core::{fmt, fmt::Display};
use core::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Debug)]
pub struct ParseCursorError;

impl FromStr for CursorMove {
    type Err = ParseCursorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CursorMove::*;
        if s.len() < 4 || &s[0..=1] != CSI {
            return Err(ParseCursorError);
        }
        match &s[(s.len() - 1)..s.len()] {
            "H" => s.find(';').ok_or(ParseCursorError).and_then(|off| {
                let line: usize = s[2..off].parse().map_err(|_e| ParseCursorError)?;
                if off + 1 >= s.len() {
                    return Err(ParseCursorError);
                }
                let column: usize = s[off + 1..s.len() - 1].parse().map_err(|_e| ParseCursorError)?;
                Ok(Pos(terminal::Pos { line, column }))
            }),
            _ => {
                let nb: usize = s[2..s.len() - 1].parse().map_err(|_e| ParseCursorError)?;
                Ok(match &s[(s.len() - 1)..s.len()] {
                    "A" => Up(nb),
                    "B" => Down(nb),
                    "C" => Forward(nb),
                    "D" => Backward(nb),
                    "G" => HorizontalAbsolute(nb),
                    _ => Err(ParseCursorError)?,
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cursor() {
        use CursorMove::*;
        // println!("{}", Pos(terminal::Pos { line: 3, column: 3 }));
        // println!("{}", Pos(terminal::Pos { line: 1, column: 1 }));
        // println!("{}", Forward(10));

        let cursors = [
            Pos(terminal::Pos { line: 1, column: 42 }),
            Up(10),
            Down(32),
            Forward(84),
            Backward(128),
            HorizontalAbsolute(16),
        ];

        for cursor in cursors.iter() {
            let cursor_str = &format!("{}", cursor);
            assert_eq!(CursorMove::from_str(cursor_str).unwrap(), *cursor);
        }
    }
}
