use super::convert::Convert;
use super::MathError;
use super::MathResult;

static mut SEED: Option<u16> = None;

pub fn srand_init(seed: u16) -> MathResult<()> {
    if seed == 0 {
        Err(MathError::OutOfBound)
    } else {
        unsafe {
            SEED = Some(seed);
        }
        // generate diagram
        Ok(())
    }
}

/// This is THE trait
trait GetSeq<T> {
    fn get_seq(self) -> T;
}

/// i16
impl GetSeq<i16> for i16 {
    fn get_seq(self) -> i16 {
        0
    }
}

/// i8
impl GetSeq<i8> for i8 {
    fn get_seq(self) -> i8 {
        0
    }
}

/// u16
impl GetSeq<u16> for u16 {
    fn get_seq(self) -> u16 {
        0
    }
}

/// u8
impl GetSeq<u8> for u8 {
    fn get_seq(self) -> u8 {
        0
    }
}

/// Has provide just one method
/// srand is seeded based random and use a seed algorythm
pub trait Srand<T> {
    fn srand(self) -> T;
}

/// i16 srand: -self..+self as i16
impl Srand<i16> for i16 {
    /// [core::i16::MIN..core::i16::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn srand(self) -> i16 {
        let t: i16 = self.get_seq();
        (t as f32 / core::i16::MIN as f32 * self as f32).round() as i16
    }
}

/// i8 srand: -self..+self as i8
impl Srand<i8> for i8 {
    /// [core::i8::MIN..core::i8::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn srand(self) -> i8 {
        let t: i8 = self.get_seq();
        (t as f32 / core::i8::MIN as f32 * self as f32).round() as i8
    }
}

/// u16 srand: 0..+self as u16
impl Srand<u16> for u16 {
    /// [0..core::u16::MAX] € Z -> [0..+1] € D -> [0..+self] € D -> [0..+self] € Z
    fn srand(self) -> u16 {
        let t: u16 = self.get_seq();
        (t as f32 / core::u16::MAX as f32 * self as f32).round() as u16
    }
}

/// u8 srand: 0..+self as u8
impl Srand<u8> for u8 {
    /// [0..core::u8::MAX] € Z -> [0..+1] € D -> [0..+self] € D -> [0..+self] € Z
    fn srand(self) -> u8 {
        let t: u8 = self.get_seq();
        (t as f32 / core::u8::MAX as f32 * self as f32).round() as u8
    }
}
