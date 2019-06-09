//! This module provides little math functions

pub mod convert;
pub mod random;
pub mod trigonometry;

/// List of all possible math errors
#[derive(Debug, Copy, Clone)]
pub enum MathError {
    /// Number Out Of Bound
    OutOfBound,
    /// Must be Initialized
    NotInitialized,
    /// A Division by zero is a impossible operation
    DivideByZero,
    /// Mathematical Out Of Bound
    Overflow,
    /// This reak number seems to be infinite
    Infinity,
    /// Unsupported operation
    Unsupported,
}

/// Math result convention
pub type MathResult<T> = core::result::Result<T, MathError>;
