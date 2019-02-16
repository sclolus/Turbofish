/// number conversion like rand or trunc
pub mod convert;
pub mod random;

#[derive(Debug, Copy, Clone)]
pub enum MathError {
    OutOfBound,
    DivideByZero,
    Overflow,
    Infinity,
    Unsupported,
}

pub type MathResult<T> = core::result::Result<T, MathError>;
