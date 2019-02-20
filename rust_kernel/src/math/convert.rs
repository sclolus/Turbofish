pub trait Convert {
    /// trunc a decimal number
    fn trunc(self) -> Self;
    /// round a decimal number to nearest integer value
    fn round(self) -> Self;
}

extern "C" {
    /// trunc is coded in Assembly langage
    fn _trunc(f: f64) -> f64;
}

impl Convert for f32 {
    fn trunc(self) -> f32 {
        let x: f64 = self as f64;
        x.trunc() as f32
    }
    fn round(self) -> f32 {
        let x: f64 = self as f64;
        x.round() as f32
    }
}

impl Convert for f64 {
    fn trunc(self) -> f64 {
        unsafe { _trunc(self) }
    }
    fn round(self) -> f64 {
        let x: f64 = self.trunc();
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
        let array: [(f64, f64); 10] = [
            (3.14, 3.),
            (-895.11, -895.),
            (0.49, 0.),
            (core::f64::INFINITY, core::f64::INFINITY),
            (0., 0.),
            (4_294_967_295.508, 4_294_967_296.0),
            (4_294_967_295.487, 4_294_967_295.0),
            (-999.51, -1000.),
            (-999.49, -999.),
            (-999.0, -999.0),
        ];

        for i in array.iter() {
            assert_eq!(f64::round(i.0), i.1);
        }
    }
}
