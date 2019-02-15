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
    fn get_seq(self) -> (T, T);
}

/// i16 i8
impl GetSeq<i16> for i16 {
    fn get_seq(self) -> (i16, i16) {
        (1, 1)
    }
}

/// u16 u8
impl GetSeq<u16> for u16 {
    fn get_seq(self) -> (u16, u16) {
        (1, 1)
    }
}

/// Has provide just one method
/// srand is seeded based random and use a seed algorythm
pub trait Srand<T> {
    fn srand(self) -> T;
}

/// i16 srand: -self..+self as i16
impl Srand<i16> for i16 {
    /// [-1..+1] € D -> [-self..+self] € D -> [-self..+self] € Z
    fn srand(self) -> i16 {
        let t: (i16, i16) = self.get_seq();
        (t.0 as f32 / t.1 as f32 * self as f32).round() as i16
    }
}

/// i8 srand: -self..+self as i8
impl Srand<i8> for i8 {
    /// [-1..+1] € D -> [-self..+self] € D -> [-self..+self] € Z
    fn srand(self) -> i8 {
        let t: (i16, i16) = (self as i16).get_seq();
        (t.0 as f32 / t.1 as f32 * self as f32).round() as i8
    }
}

/// u16 srand: 0..+self as u16
impl Srand<u16> for u16 {
    /// [0..+1] € D -> [0..+self] € D -> [0..+self] € Z
    fn srand(self) -> u16 {
        let t: (u16, u16) = self.get_seq();
        (t.0 as f32 / t.1 as f32 * self as f32).round() as u16
    }
}

/// u8 srand: 0..+self as u8
impl Srand<u8> for u8 {
    /// [0..+1] € D -> [0..+self] € D -> [0..+self] € Z
    fn srand(self) -> u8 {
        let t: (u16, u16) = (self as u16).get_seq();
        (t.0 as f32 / t.1 as f32 * self as f32).round() as u8
    }
}
