//! Contains initialisation colors
use super::CSI;
use core::str::FromStr;
use core::{fmt, fmt::Display};

/// (0 ≤ r, g, b ≤ 5)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AnsiRGB {
    r: u8,
    g: u8,
    b: u8,
}

/// minimal set of colors for VGA compatibility
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StandardColor {
    Black = 0,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl From<StandardColor> for AnsiColor {
    fn from(c: StandardColor) -> Self {
        AnsiColor::Standard(c)
    }
}

/// handle differents types of colors
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AnsiColor {
    /// 0 -> 7
    Standard(StandardColor),
    /// 0 -> 7
    HighIntensity(u8),
    /// 0 -> 24
    AnsiRGB(AnsiRGB),
    /// 0 -> 24
    Grey(u8),
}

impl Default for AnsiColor {
    fn default() -> Self {
        AnsiColor::Standard(StandardColor::White)
    }
}

// associate colors with ANSI standard
#[allow(missing_docs)]
impl AnsiColor {
    pub const BLACK: Self = AnsiColor::Standard(StandardColor::Black);
    pub const RED: Self = AnsiColor::Standard(StandardColor::Red);
    pub const GREEN: Self = AnsiColor::Standard(StandardColor::Green);
    pub const YELLOW: Self = AnsiColor::Standard(StandardColor::Yellow);
    pub const BLUE: Self = AnsiColor::Standard(StandardColor::Blue);
    pub const MAGENTA: Self = AnsiColor::Standard(StandardColor::Magenta);
    pub const CYAN: Self = AnsiColor::Standard(StandardColor::Cyan);
    pub const WHITE: Self = AnsiColor::Standard(StandardColor::White);
}

impl From<AnsiColor> for u8 {
    fn from(c: AnsiColor) -> u8 {
        use AnsiColor::*;
        match c {
            Standard(x) => x as u8,
            HighIntensity(x) => x + 8,
            AnsiRGB(x) => 16 + 36 * x.r + 6 * x.g + x.b,
            Grey(x) => 232 + x,
        }
    }
}

impl From<u8> for AnsiColor {
    fn from(c: u8) -> AnsiColor {
        use AnsiColor::*;
        match c {
            0...7 => Standard(unsafe { core::mem::transmute(c) }),
            8...15 => HighIntensity(c - 8),
            16...231 => AnsiRGB(self::AnsiRGB { r: (c - 16) / (6 * 6), g: ((c - 16) / 6) % 6, b: (c - 16) % 6 }),
            232...255 => Grey(c - 232),
        }
    }
}

impl Display for AnsiColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}38;5;{}m", CSI, Into::<u8>::into(*self))
    }
}

/// local error enum
#[derive(Debug)]
pub struct ParseColorError;

impl FromStr for AnsiColor {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // eprintln!("{:?}", s);
        if s == "\x1b[0m" {
            return Ok(AnsiColor::WHITE);
        }
        if &s[(s.len() - 1)..s.len()] != "m" {
            return Err(ParseColorError);
        }
        if &s[0..=1] != CSI {
            return Err(ParseColorError);
        }
        // TODO: handle other color esape codes
        if s.len() < 6 || &s[2..=6] != "38;5;" {
            return Err(ParseColorError);
        }
        let nb: u8 = s[7..s.find('m').unwrap()].parse().map_err(|_e| ParseColorError)?;
        Ok(nb.into())
    }
}

/// definitition of ansi string
#[derive(Copy, Clone, Debug)]
pub struct AnsiStr<'a> {
    s: &'a str,
    color: AnsiColor,
}

impl<'a> AnsiStr<'a> {
    #[allow(dead_code)]
    fn bright(self) -> Self {
        use AnsiColor::*;
        Self {
            color: match self.color {
                Standard(x) => HighIntensity(x as u8),
                o => o,
            },
            ..self
        }
    }
}

impl<'a> Display for AnsiStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.color, self.s, AnsiColor::default())
    }
}

/// this trait contains method to associate a string to a color
#[allow(missing_docs)]
pub trait Colored {
    fn black(&self) -> AnsiStr;
    fn red(&self) -> AnsiStr;
    fn green(&self) -> AnsiStr;
    fn yellow(&self) -> AnsiStr;
    fn blue(&self) -> AnsiStr;
    fn magenta(&self) -> AnsiStr;
    fn cyan(&self) -> AnsiStr;
    fn white(&self) -> AnsiStr;
    /// r, g, b are >= 0 && < 6
    fn rgb(&self, r: u8, g: u8, b: u8) -> AnsiStr;
    fn grey(&self, intensity: u8) -> AnsiStr;
}

impl Colored for str {
    fn black<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::BLACK }
    }
    fn red<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::RED }
    }
    fn green<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::GREEN }
    }
    fn yellow<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::YELLOW }
    }
    fn blue<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::BLUE }
    }
    fn magenta<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::MAGENTA }
    }
    fn cyan<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::CYAN }
    }
    fn white<'a>(&'a self) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::WHITE }
    }
    fn rgb<'a>(&'a self, r: u8, g: u8, b: u8) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::AnsiRGB(AnsiRGB { r, g, b }) }
    }
    fn grey<'a>(&'a self, intensity: u8) -> AnsiStr<'a> {
        AnsiStr { s: self, color: AnsiColor::Grey(intensity) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_color() {
        println!("{}", "I AM BLACK".black());
        println!("{}", "I AM RED".red());
        println!("{}", "I AM GREEN".green());
        println!("{}", "I AM YELLOW".yellow());
        println!("{}", "I AM BLUE".blue());
        println!("{}", "I AM MAGENTA".magenta());
        println!("{}", "I AM CYAN".cyan());
        println!("{}", "I AM WHITE".white());

        println!("{}", "I AM BLACK".black().bright());
        println!("{}", "I AM RED".red().bright());
        println!("{}", "I AM GREEN".green().bright());
        println!("{}", "I AM YELLOW".yellow().bright());
        println!("{}", "I AM BLUE".blue().bright());
        println!("{}", "I AM MAGENTA".magenta().bright());
        println!("{}", "I AM CYAN".cyan().bright());
        println!("{}", "I AM WHITE".white().bright());

        for r in 0..6 {
            for g in 0..6 {
                for b in 0..6 {
                    print!("{}", "O".rgb(r, g, b));
                }
            }
        }
        for grey in 0..24 {
            print!("{}", "G".grey(grey));
        }
        // println!("{}", "I AM MAGENTA".magenta());
        for i in 0..=255 {
            let color: AnsiColor = i.into();
            assert_eq!(Into::<u8>::into(color), i);
            let color_str = format!("{}", color);
            assert_eq!(
                AnsiColor::from_str(&color_str)
                    .expect(&format!("failed to parse colors_str: {} at it: {}", color_str, i)),
                color
            );
        }
    }
}
