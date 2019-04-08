//TODO: implemented std::error::Error
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EitherError {
    NotLeft,
    NotRight,
}

#[derive(Debug, Copy, Clone)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

use Either::*;
impl<T, U> Either<T, U> {
    pub fn is_left(&self) -> bool {
        match self {
            Left(_) => true,
            Right(_) => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Left(_) => false,
            Right(_) => true,
        }
    }

    pub fn unwrap_left(self) -> T {
        if let Left(value) = self {
            value
        } else {
            panic!("{} was called on Either::Right value", function!())
        }
    }

    pub fn unwrap_right(self) -> U {
        if let Right(value) = self {
            value
        } else {
            panic!("{} was called on Either::Left value", function!())
        }
    }

    pub fn map_left<A, F: FnOnce(T) -> A>(self, closure: F) -> Either<A, U> {
        if self.is_left() {
            Left(closure(self.unwrap_left()))
        } else {
            Right(self.unwrap_right())
        }
    }

    pub fn map_right<A, F: FnOnce(U) -> A>(self, closure: F) -> Either<T, A> {
        if self.is_right() {
            Right(closure(self.unwrap_right()))
        } else {
            Left(self.unwrap_left())
        }
    }
}

impl<T> Either<T, T> {
    pub fn move_out(self) -> T {
        match self {
            Left(value) => value,
            Right(value) => value,
        }
    }
}

// impl<T, U> From<T> for Either<T, U> {
//     fn from(value: T) -> Self {
//         Left(value)
//     }
// }

// impl<T, U> From<U> for Either<T, U> {
//     fn from(value: U) -> Self {
//         Right(value)
//     }
// }

// impl<T, U> TryFrom<T> for Either<T, U> {
//     type Error = EitherError;

//     fn try_from(value: T) -> Result<Self, Self::Error> {
//     }
// }
