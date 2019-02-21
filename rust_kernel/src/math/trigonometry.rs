/// Trigonometry main implementation
pub trait Trigonometry {
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn tan(self) -> Self;
}

/// generic function cos with classic syntax
pub fn cos<T: Trigonometry>(f: T) -> T {
    T::cos(f)
}

/// generic function sin with classic syntax
pub fn sin<T: Trigonometry>(f: T) -> T {
    T::sin(f)
}

/// generic function tan with classic syntax
pub fn tan<T: Trigonometry>(f: T) -> T {
    T::tan(f)
}

extern "C" {
    fn _cos(f: f64) -> f64;
    fn _sin(f: f64) -> f64;
    fn _tan(f: f64) -> f64;
}

impl Trigonometry for f32 {
    fn cos(self) -> f32 {
        let x: f64 = self as f64;
        x.cos() as f32
    }
    fn sin(self) -> f32 {
        let x: f64 = self as f64;
        x.sin() as f32
    }
    fn tan(self) -> f32 {
        let x: f64 = self as f64;
        x.tan() as f32
    }
}

impl Trigonometry for f64 {
    fn cos(self) -> f64 {
        unsafe { _cos(self) }
    }
    fn sin(self) -> f64 {
        unsafe { _sin(self) }
    }
    fn tan(self) -> f64 {
        unsafe { _tan(self) }
    }
}
