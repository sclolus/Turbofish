//! [Ansi escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub mod color;
pub mod cursor;

///Comand Sequence Introducer
const CSI: &str = "\x1b[";

pub fn iter_escaped<'a>(s: &'a str) -> IterEscaped<'a> {
    IterEscaped { s, off: 0 }
}

pub struct IterEscaped<'a> {
    off: usize,
    s: &'a str,
}

impl<'a> Iterator for IterEscaped<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.off >= self.s.len() {
            return None;
        }
        if self.s[self.off..].starts_with(0x1b as char) {
            let next_alpha =
                self.off + self.s[self.off..].find(|x: char| x.is_ascii_alphabetic()).unwrap_or(self.s.len() - 1);

            let ret = Some(&self.s[self.off..=next_alpha]);
            self.off = next_alpha + 1;
            ret
        } else {
            let next_escape = self.off + self.s[self.off..].find(0x1b as char).unwrap_or(self.s.len());

            let ret = Some(&self.s[self.off..next_escape]);
            self.off = next_escape;
            ret
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::terminal::ansi_escape_code::color::{AnsiColor, Colored};
    use std::str::FromStr;
    #[test]
    fn test_iter_escape() {
        let s = format!("{}", "I AM BLACK".black());
        let iterator = iter_escaped(&s);
        assert_eq!(iterator.count(), 3);

        let mut iterator = iter_escaped(&s);
        assert_eq!(AnsiColor::from_str(iterator.next().unwrap()).unwrap(), AnsiColor::BLACK);
        assert_eq!(iterator.next().unwrap(), "I AM BLACK");
        assert_eq!(AnsiColor::from_str(iterator.next().unwrap()).unwrap(), AnsiColor::default());
        // for s in iter_escaped(&format!("{}", "I AM BLACK".black())) {
        //     dbg!(s);
        // }
    }
}
