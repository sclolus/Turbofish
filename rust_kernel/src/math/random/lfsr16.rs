use super::MathError;
use super::MathResult;

use bit_field::BitField;

/// see https://en.wikipedia.org/wiki/Linear-feedback_shift_register
const SEQ_SIZE: usize = 1 << 11;
/// That tupple contains 'fibo array' 'current_offset' 'stored seed'
static mut LFSR_FIBONACCI: ([u32; SEQ_SIZE], usize, Option<u16>) = ([0; SEQ_SIZE], 0, None);

/// Fibonacci LFSR
pub fn lfsr16_srand_init(seed: u16) -> MathResult<()> {
    if seed == 0 {
        Err(MathError::OutOfBound)
    } else {
        let mut lfsr: u16 = seed;
        unsafe {
            // lfsr fly time must be at 1 ^ 16
            // enumerator is only used for assert! check
            for (i, elem) in LFSR_FIBONACCI.0.iter_mut().enumerate() {
                for j in 0..32 {
                    let bits: u16 = (lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5);
                    lfsr = lfsr >> 1;
                    lfsr.set_bit(15, bits.get_bit(0));
                    (*elem).set_bit(j, bits.get_bit(0));

                    // check of algorythm mathematical coherency
                    assert!(lfsr != seed || (lfsr == seed && i as usize == SEQ_SIZE - 1 && j == 30));
                }
            }
            LFSR_FIBONACCI.2 = Some(seed);
        }
        // partial check of algorythm calculation success
        assert!(lfsr << 1 == seed & 0xfffe);
        Ok(())
    }
}

/// Return the current lfsr seed
pub fn lfsr16_get_seed() -> MathResult<u16> {
    match unsafe { LFSR_FIBONACCI.2 } {
        Some(s) => Ok(s),
        None => Err(MathError::NotInitialized),
    }
}

/// move offset into flsr
fn move_offset(offset: usize) -> usize {
    if offset == SEQ_SIZE - 1 {
        0
    } else {
        offset + 1
    }
}

pub trait GetPseudoNumber<T> {
    /// get a pseudo random number from the lfsr fibonacci suite
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
