//! [Ansi escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub mod color;
pub mod cursor;

///Comand Sequence Introducer
const CSI: &str = "\x1b[";
