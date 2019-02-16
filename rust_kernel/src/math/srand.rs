use super::convert::Convert;
use super::MathError;
use super::MathResult;
use bit_field::BitField;

// see https://en.wikipedia.org/wiki/Linear-feedback_shift_register
const SEQ_SIZE: usize = 1 << 11;
static mut LFSR_FIBONACCI: ([u32; SEQ_SIZE], usize) = ([0; SEQ_SIZE], 0);

/// Fibonacci LFSR
pub fn srand_init(seed: u16) -> MathResult<()> {
    if seed == 0 {
        Err(MathError::OutOfBound)
    } else {
        let mut lfsr: u16 = seed;
        unsafe {
            // lfsr fly time must be 1 ^ 16
            for (i, elem) in LFSR_FIBONACCI.0.iter_mut().enumerate() {
                for j in 0..32 {
                    let bits: u16 = (lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5);
                    lfsr = lfsr >> 1;
                    lfsr.set_bit(15, bits.get_bit(0));
                    (*elem).set_bit(j, bits.get_bit(0));

                    // check of algorythm calculation coherency
                    assert!(lfsr != seed || (lfsr == seed && i as usize == SEQ_SIZE - 1 && j == 30));
                }
            }
        }
        // partial check of algorythm calculation success
        assert!(lfsr << 1 == seed & 0xfffe);

        Ok(())
    }
}

fn move_offset(offset: usize) -> usize {
    if offset == SEQ_SIZE - 1 {
        0
    } else {
        offset + 1
    }
}

trait GetPseudoNumber<T> {
    fn get_pseudo_number() -> T;
}

impl GetPseudoNumber<u32> for u32 {
    fn get_pseudo_number() -> u32 {
        let result: u32;
        unsafe {
            result = LFSR_FIBONACCI.0[LFSR_FIBONACCI.1];
            LFSR_FIBONACCI.1 = move_offset(LFSR_FIBONACCI.1);
        }
        result
    }
}

impl GetPseudoNumber<i32> for i32 {
    fn get_pseudo_number() -> i32 {
        let result: i32;
        unsafe {
            result = LFSR_FIBONACCI.0[LFSR_FIBONACCI.1] as i32;
            LFSR_FIBONACCI.1 = move_offset(LFSR_FIBONACCI.1);
        }
        result
    }
}

/// Has provide just one method
/// srand is seeded based random and use a seed algorythm
pub trait Srand<T> {
    fn srand(self) -> T;
}

/// f32 srand: -self..+self as f32
impl Srand<f32> for f32 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D
    fn srand(self) -> f32 {
        let t: i32 = i32::get_pseudo_number();
        t as f32 / core::i32::MIN as f32 * self as f32
    }
}

/// i32 srand: -self..+self as i32
impl Srand<i32> for i32 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn srand(self) -> i32 {
        let t: i32 = i32::get_pseudo_number();
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i32
    }
}

/// i16 srand: -self..+self as i16
impl Srand<i16> for i16 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn srand(self) -> i16 {
        let t: i32 = i32::get_pseudo_number();
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i16
    }
}

/// i8 srand: -self..+self as i8
impl Srand<i8> for i8 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn srand(self) -> i8 {
        let t: i32 = i32::get_pseudo_number();
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i8
    }
}

/// u32 srand: 0..+self as u32
impl Srand<u32> for u32 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn srand(self) -> u32 {
        let t: u32 = u32::get_pseudo_number();
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u32
    }
}

/// u16 srand: 0..+self as u16
impl Srand<u16> for u16 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn srand(self) -> u16 {
        let t: u32 = u32::get_pseudo_number();
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u16
    }
}

/// u8 srand: 0..+self as u8
impl Srand<u8> for u8 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn srand(self) -> u8 {
        let t: u32 = u32::get_pseudo_number();
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u8
    }
}
