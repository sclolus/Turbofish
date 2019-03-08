use super::convert::Convert;

use super::MathError;
use super::MathResult;

mod rdrand;
use rdrand::rdrand;

mod lfsr16;
use lfsr16::{lfsr16_get_pseudo_number, lfsr16_set_seed};

use bit_field::BitField;

/// Has provide two methods
/// rand is totally undetermined and use RDRAND cpu feature (ivybridge +)
/// srand is seeded based random and use a seed algorythm
pub trait Random {
    fn rand(self) -> Self;
    fn srand(self) -> Self;
}

/// Enumeration of all randomize methods
pub enum Methods {
    Rdrand,
    Lfsr16,
}

/// generic function rand with classic syntax
pub fn rand<T: Random>(x: T) -> T {
    T::rand(x)
}

/// generic function srand with classic syntax
pub fn srand<T: Random>(x: T) -> T {
    T::srand(x)
}

/// internal trait, Randup (not roundup) is a common family name in US
pub trait Rand {
    fn randup(self, method: Methods) -> Self;
}

/// For now, lfsr16 is the only one method for srand, implentation may be extended in future
pub fn srand_init(seed: u16) -> MathResult<()> {
    lfsr16_set_seed(seed)
}

/// Main trait inherance
impl<T: Rand> Random for T {
    fn rand(self) -> Self {
        T::randup(self, Methods::Rdrand)
    }
    fn srand(self) -> Self {
        T::randup(self, Methods::Lfsr16)
    }
}

trait Generate {
    /// get a random number with the right method
    fn generate(method: Methods) -> Self;
}

impl Generate for u32 {
    fn generate(method: Methods) -> Self {
        match method {
            Methods::Rdrand => rdrand(),
            Methods::Lfsr16 => lfsr16_get_pseudo_number().unwrap(),
        }
    }
}

impl Generate for i32 {
    fn generate(method: Methods) -> Self {
        match method {
            Methods::Rdrand => rdrand() as i32,
            Methods::Lfsr16 => lfsr16_get_pseudo_number().unwrap() as i32,
        }
    }
}

/// f64 rand: -self..+self as f64
impl Rand for f64 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D
    fn randup(self, method: Methods) -> f64 {
        let t: i32 = i32::generate(method);
        t as f64 / core::i32::MIN as f64 * self as f64
    }
}

/// f32 rand: -self..+self as f32
impl Rand for f32 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D
    fn randup(self, method: Methods) -> f32 {
        let t: i32 = i32::generate(method);
        t as f32 / core::i32::MIN as f32 * self as f32
    }
}

/// i32 rand: -self..+self as i32
impl Rand for i32 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn randup(self, method: Methods) -> i32 {
        let t: i32 = i32::generate(method);
        // lack of precision for i32 type with f32, usage of f64 instead
        (t as f64 / core::i32::MIN as f64 * self as f64).round() as i32
    }
}

/// isize rand: -self..+self as isize
impl Rand for isize {
    /// [core::isize::MIN..core::isize::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn randup(self, method: Methods) -> isize {
        let t: i32 = i32::generate(method);
        // lack of precision for isize type with f32, usage of f64 instead
        (t as f64 / core::isize::MIN as f64 * self as f64).round() as isize
    }
}

/// i16 rand: -self..+self as i16
impl Rand for i16 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn randup(self, method: Methods) -> i16 {
        let t: i32 = i32::generate(method);
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i16
    }
}

/// i8 rand: -self..+self as i8
impl Rand for i8 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn randup(self, method: Methods) -> i8 {
        let t: i32 = i32::generate(method);
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i8
    }
}

/// u32 rand: 0..+self as u32
impl Rand for u32 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn randup(self, method: Methods) -> u32 {
        let t: u32 = u32::generate(method);
        // lack of precision for u32 type with f32, usage of f64 instead
        (t as f64 / core::u32::MAX as f64 * self as f64).round() as u32
    }
}

/// usize rand: 0..+self as usize
impl Rand for usize {
    /// [0..core::usize::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn randup(self, method: Methods) -> usize {
        let t: u32 = u32::generate(method);
        // lack of precision for u32 type with f32, usage of f64 instead
        (t as f64 / core::usize::MAX as f64 * self as f64).round() as usize
    }
}

/// u16 rand: 0..+self as u16
impl Rand for u16 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn randup(self, method: Methods) -> u16 {
        let t: u32 = u32::generate(method);
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u16
    }
}

/// u8 rand: 0..+self as u8
impl Rand for u8 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn randup(self, method: Methods) -> u8 {
        let t: u32 = u32::generate(method);
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u8
    }
}

/// bool rand: 0..1 as bool
impl Rand for bool {
    /// [0..core::u32::MAX] € N -> &0b1 [FALSE | TRUE]
    fn randup(self, method: Methods) -> bool {
        let t: u32 = u32::generate(method);
        t.get_bit(0)
    }
}

#[cfg(test)]
mod test {
    use super::Random;

    #[test]
    fn random_out_of_bound_i16_test() {
        for i in (core::i16::MIN..0).into_iter().step_by(128) {
            let x: i16 = i.rand();
            let limit_high = match i {
                core::i16::MIN => core::i16::MAX,
                _ => -1 * i,
            };
            assert!(x >= i && x <= limit_high);
        }
    }
    #[test]
    fn random_out_of_bound_i32_test() {
        for i in (core::i32::MIN..0).into_iter().step_by(4096) {
            // test signed 32
            let x: i32 = i.rand();
            let limit_high = match i {
                core::i32::MIN => core::i32::MAX,
                _ => -1 * i,
            };
            assert!(x >= i && x <= limit_high);
        }
    }
    #[test]
    fn random_out_of_bound_u16_test() {
        for i in (0..core::u16::MAX).into_iter().step_by(128) {
            // test unsigned 16
            let x: u16 = i.rand();
            assert!(x <= i);
        }
    }
    #[test]
    fn random_out_of_bound_u32_test() {
        for i in (0..core::u32::MAX).into_iter().step_by(4096) {
            // test unsigned 32
            let x: u32 = i.rand();
            assert!(x <= i);
        }
    }
    #[test]
    fn random_out_of_bound_f32_test() {
        for i in (0..core::u32::MAX).into_iter().step_by(4096) {
            // test f32
            let x: f32 = (i as f32).rand();
            assert!(x >= (i as f32 * -1.) && x <= i as f32);
        }
    }
    #[test]
    fn random_out_of_bound_f64_test() {
        for i in (0..core::u32::MAX).into_iter().step_by(4096) {
            // test f64
            let x: f64 = (i as f64).rand();
            assert!(x >= (i as f64 * -1.) && x <= i as f64);
        }
    }
    #[test]
    fn random_mediane_test() {
        let mut mediane_u: f64 = 0.;
        // u32 test
        for i in 1..100000 {
            mediane_u += (core::u32::MAX.rand() as f64 - mediane_u) / i as f64;
        }
        // i32 test
        let mut mediane_i: f64 = 0.;
        for i in 1..100000 {
            mediane_i += (core::i32::MIN.rand() as f64 - mediane_i) / i as f64;
        }
        // f32 test
        let mut mediane_f: f64 = 0.;
        for i in 1..100000 {
            mediane_f += ((4242.4242).rand() as f64 - mediane_f) / i as f64;
        }
        println!("{}\nu32 -> {:?}\ni32 -> {:?}\nf32 -> {:?}", function!(), mediane_u, mediane_i, mediane_f);
    }
    #[test]
    fn random_distribution_test() {
        use std::collections::HashMap;
        let mut buckets = HashMap::new();

        let gen_nbr = (0..100000).into_iter().map(|_| u32::rand(1000));
        let n_buckets = 50;
        let gen_key = |nbr| nbr % n_buckets;

        for nbr in gen_nbr {
            *buckets.entry(gen_key(nbr)).or_insert(0) += 1;
        }

        let first_iter = buckets.iter().map(|(_, &value)| value as i64);
        let cloned_iter = first_iter.clone().take((n_buckets - 1) as usize);
        let final_iter = first_iter.skip(1).zip(cloned_iter);

        let s = final_iter.fold(0, |acc, (first, second): (i64, i64)| acc + (first - second).abs());
        println!("{} distribution / 100000 rand(). 50 buckets on range of 1000 -> {:?}", function!(), s);
    }
}
