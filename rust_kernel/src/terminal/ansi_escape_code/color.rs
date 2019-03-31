use super::CSI;
use core::str::FromStr;
use core::{fmt, fmt::Display};

/// (0 ≤ r, g, b ≤ 5)
#[derive(Copy, Clone, Debug, PartialEq)]
struct AnsiRGB {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum AnsiColor {
    /// 0 -> 7
    Standard(u8),
    /// 0 -> 7
    HighIntensity(u8),
    AnsiRGB(AnsiRGB),
    /// 0 -> 24
    Grey(u8),
}

impl Default for AnsiColor {
    fn default() -> Self {
        AnsiColor::Standard(7)
    }
}

impl AnsiColor {
    const BLACK: Self = AnsiColor::Standard(0);
    const RED: Self = AnsiColor::Standard(1);
    const GREEN: Self = AnsiColor::Standard(2);
    const YELLOW: Self = AnsiColor::Standard(3);
    const BLUE: Self = AnsiColor::Standard(4);
    const MAGENTA: Self = AnsiColor::Standard(5);
    const CYAN: Self = AnsiColor::Standard(6);
    const WHITE: Self = AnsiColor::Standard(7);
}

impl From<AnsiColor> for u8 {
    fn from(c: AnsiColor) -> u8 {
        use AnsiColor::*;
        match c {
            Standard(x) => x,
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
            0...7 => Standard(c),
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

#[derive(Debug)]
struct ParseColorError;

impl FromStr for AnsiColor {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 6 {
            return Err(ParseColorError);
        }
        if &s[0..=1] != CSI {
            return Err(ParseColorError);
        }
        // TODO: handle other color esape codes
        if &s[2..=6] != "38;5;" || &s[(s.len() - 1)..s.len()] != "m" {
            return Err(ParseColorError);
        }
        let nb: u8 = s[7..s.find('m').unwrap()].parse().map_err(|_e| ParseColorError)?;
        Ok(nb.into())
    }
}

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
                Standard(x) => HighIntensity(x),
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
            let color_str = format!("{}", format_args!("{}", color));
            assert_eq!(
                AnsiColor::from_str(&color_str)
                    .expect(&format!("failed to parse colors_str: {} at it: {}", color_str, i)),
                color
            );
        }
    }
}
