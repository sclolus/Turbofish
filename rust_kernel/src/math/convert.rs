//! This module provides convertion functions

// use core::intrinsics::truncf32;

/// Convert trait implementation
pub trait Convert {
    /// trunc a decimal number
    fn trunc(self) -> Self;
    /// round a decimal number to nearest integer value
    fn round(self) -> Self;
}

impl Convert for f32 {
    fn trunc(self) -> f32 {
        (self as i32) as f32
    }
    fn round(self) -> f32 {
        let x: f32 = self.trunc();
        if self >= 0. {
            if self - x <= 0.5 {
                x
            } else {
                x + 1.
            }
        } else {
            if self - x >= -0.5 {
                x
            } else {
                x - 1.
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::math::convert::Convert;
    #[test]
    fn test_round() {
        let array: [(f32, f32); 10] = [
            (3.14, 3.),
            (-895.11, -895.),
            (0.49, 0.),
            (core::f32::INFINITY, core::f32::INFINITY),
            (0., 0.),
            (4_294_967_295.508, 4_294_967_296.0),
            (4_294_967_295.487, 4_294_967_295.0),
            (-999.51, -1000.),
            (-999.49, -999.),
            (-999.0, -999.0),
        ];

        for i in array.iter() {
            assert_eq!(f32::round(i.0), i.1);
        }
    }
}
