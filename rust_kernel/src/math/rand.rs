use super::convert::Convert;

/// Ivybridge+ RDRAND feature.
/// large unsafe autocast with T, BE CAREFULL dont do nasty things with float types !
/// rdrand set the carry flag to 1 if the random is well done, else loop while it works
fn rdrand<T>() -> T {
    let result: T;

    unsafe {
        asm!("
            1:
            rdrand %eax
            jnc 1b" : "={eax}"(result) :::);
    }
    result
}

/// Has provide just one method
/// rand is totally undetermined and use RDRAND cpu feature (ivybridge +)
pub trait Rand<T> {
    fn rand(self) -> T;
}

/// f32 rand: -self..+self as f32
impl Rand<f32> for f32 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D
    fn rand(self) -> f32 {
        let t: i32 = rdrand();
        t as f32 / core::i32::MIN as f32 * self as f32
    }
}

/// i32 rand: -self..+self as i32
impl Rand<i32> for i32 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn rand(self) -> i32 {
        let t: i32 = rdrand();
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i32
    }
}

/// i16 rand: -self..+self as i16
impl Rand<i16> for i16 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn rand(self) -> i16 {
        let t: i32 = rdrand();
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i16
    }
}

/// i8 rand: -self..+self as i8
impl Rand<i8> for i8 {
    /// [core::i32::MIN..core::i32::MAX] € Z -> [+1..~-1] € D -> [+self..-self] € D -> [+self..-self] € Z
    fn rand(self) -> i8 {
        let t: i32 = rdrand();
        (t as f32 / core::i32::MIN as f32 * self as f32).round() as i8
    }
}

/// u32 rand: 0..+self as u32
impl Rand<u32> for u32 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn rand(self) -> u32 {
        let t: u32 = rdrand();
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u32
    }
}

/// u16 rand: 0..+self as u16
impl Rand<u16> for u16 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn rand(self) -> u16 {
        let t: u32 = rdrand();
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u16
    }
}

/// u8 rand: 0..+self as u8
impl Rand<u8> for u8 {
    /// [0..core::u32::MAX] € N -> [0..+1] € D -> [0..+self] € D -> [0..+self] € N
    fn rand(self) -> u8 {
        let t: u32 = rdrand();
        (t as f32 / core::u32::MAX as f32 * self as f32).round() as u8
    }
}
