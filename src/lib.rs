//! Useful extensions to convert `bool` to other Rust types.
//!
//! At the moment there is just one extension: `.true_or()`.
//!
//! # Example
//!
//! ```
//! use crate::MyError;
//! use bool_mappings::*;
//!
//! // Turn a bool into a Result
//! fn some_fn() -> Result<(), MyError> {
//!     true.true_or(MyError)?
//! }
//! ```

/// This extension trait adds additional methods to `bool`s
pub trait BoolMappings {
    fn true_or<T>(&self, err: T) -> Result<(), T>;
}

/// This is the impl of the extension trait
impl BoolMappings for bool {
    /// Convert a `bool` into a `Result<(),T>`
    fn true_or<T>(&self, err: T) -> Result<(), T> {
        self.then_some(()).ok_or::<T>(err)
    }
}

#[cfg(test)]
mod tests {
    use super::BoolMappings;
    use std::fmt::*;

    #[derive(Debug, PartialEq, Eq)]
    struct MyError;

    impl Display for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    #[test]
    fn test_true() {
        assert!(true.true_or(MyError).is_ok())
    }

    #[test]
    fn test_false() {
        assert_eq!(false.true_or(MyError).err().unwrap(), MyError);
    }
}
