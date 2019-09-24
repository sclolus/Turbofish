//! [Ansi escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub mod color;
pub mod cursor;

pub use color::*;
pub use cursor::*;

use core::str::FromStr;

///Definition of what produce a escape sequence
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EscapedCode {
    ///Movement of the cursor
    CursorMove(CursorMove),
    ///Text color
    Color(AnsiColor),
}

///Comand Sequence Introducer
pub const CSI: &str = "\x1b[";

///Iterator on escape sequence
pub fn iter_escaped<'a>(s: &'a str) -> IterEscaped<'a> {
    IterEscaped { s, off: 0 }
}

///Document not founded
pub struct IterEscaped<'a> {
    off: usize,
    s: &'a str,
}

///Document not founded
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EscapedItem<'a> {
    ///Document not founded
    Escaped(EscapedCode),
    ///Document not founded
    Str(&'a str),
}

fn assert_str_is_valid(s: &str) {
    assert!(s.is_char_boundary(0));
    assert!(s.is_char_boundary(s.len()));
    ::core::str::from_utf8(s.as_bytes()).expect("What the actual fuck...");
}

impl<'a> Iterator for IterEscaped<'a> {
    type Item = EscapedItem<'a>;
    #[no_mangle]
    fn next(&mut self) -> Option<Self::Item> {
        use EscapedItem::*;
        if self.off >= self.s.len() {
            return None;
        }
        assert_str_is_valid(&self.s[self.off..]);
        Some(if self.s[self.off..].starts_with(0x1b as char) {
            assert!(self.s[self.off..].len() != 0);
            assert!(self.s.is_char_boundary(self.off));
            assert_str_is_valid(&self.s[self.off..]);
            let next_alpha = self.off
                + self.s[self.off..]
                    .find(|x: char| x.is_ascii_alphabetic())
                    .unwrap_or(self.s[self.off..].len() - 1);
            assert!(next_alpha < self.s.len());
            let ret = &self.s[self.off..=next_alpha];
            assert_str_is_valid(ret);
            self.off = next_alpha + 1;
            assert_str_is_valid(&self.s[next_alpha..next_alpha + 1]);
            if &self.s[next_alpha..next_alpha + 1] == "m" {
                match AnsiColor::from_str(ret) {
                    Ok(c) => Escaped(EscapedCode::Color(c)),
                    Err(_) => Str(ret),
                }
            } else {
                match CursorMove::from_str(ret) {
                    Ok(c) => Escaped(EscapedCode::CursorMove(c)),
                    Err(_) => Str(ret),
                }
            }
        } else {
            assert_str_is_valid(&self.s[self.off..]);
            let next_escape = self.off
                + self.s[self.off..]
                    .find(|x: char| x == 0x1b as char)
                    .unwrap_or(self.s[self.off..].len());
            assert!(next_escape <= self.s.len());
            let ret = &self.s[self.off..next_escape];
            assert_str_is_valid(ret);
            self.off = next_escape;
            Str(ret)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ansi_escape_code::color::{AnsiColor, Colored};
    #[test]
    fn test_iter_escape() {
        use EscapedItem::*;
        let s = format!("{}", "I AM BLACK".black());
        let iterator = iter_escaped(&s);
        assert_eq!(iterator.count(), 3);

        let mut iterator = iter_escaped(&s);
        assert_eq!(
            iterator.next().unwrap(),
            Escaped(EscapedCode::Color(AnsiColor::BLACK))
        );
        assert_eq!(iterator.next().unwrap(), Str("I AM BLACK"));
        assert_eq!(
            iterator.next().unwrap(),
            Escaped(EscapedCode::Color(AnsiColor::default()))
        );
        // for s in iter_escaped(&format!("{}", "I AM BLACK".black())) {
        //     dbg!(s);
        // }
    }

    #[test]
    fn test_iter_no_escape() {
        use EscapedItem::*;
        let s = format!("{}", "I AM BLACK");
        let mut iterator = iter_escaped(&s);
        assert_eq!(iterator.next().unwrap(), Str("I AM BLACK"));
        assert_eq!(iterator.next(), None);
    }
    #[test]
    fn test_iter_one_escape() {
        use EscapedItem::*;
        let s = format!("{}{}", AnsiColor::RED, "H");
        let mut iterator = iter_escaped(&s);
        assert_eq!(
            iterator.next().unwrap(),
            Escaped(EscapedCode::Color(AnsiColor::RED))
        );
        assert_eq!(iterator.next().unwrap(), Str("H"));
        assert_eq!(iterator.next(), None);
    }
}
