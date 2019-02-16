use super::convert::Convert;
use super::MathError;
use super::MathResult;
use bit_field::BitField;

// see https://en.wikipedia.org/wiki/Linear-feedback_shift_register
const SEQ_SIZE: usize = 1 << 11;
static mut LFSR_FIBONACCI_ARRAY: [u32; SEQ_SIZE] = [0; SEQ_SIZE];

/// Fibonacci LFSR
pub fn srand_init(seed: u16) -> MathResult<()> {
    if seed == 0 {
        Err(MathError::OutOfBound)
    } else {
        let mut lfsr: u16 = seed;
        unsafe {
            // lfsr fly time must be 1 ^ 16
            for elem in LFSR_FIBONACCI_ARRAY.iter_mut() {
                for j in 0..32 {
                    let bits: u16 = (lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5);
                    lfsr = lfsr >> 1;
                    lfsr.set_bit(15, bits.get_bit(0));
                    (*elem).set_bit(j, bits.get_bit(0));
                }
            }
        }
        // partial check of algorythm calculation success
        assert!(lfsr << 1 == seed & 0xfffe);
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
