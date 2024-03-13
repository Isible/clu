/// Macro for cleaner implementation of a trait for a struct
#[macro_export]
macro_rules! impl_trait {
    ($_trait:ty => $_struct:ty) => {
        impl $_trait for $_struct {}
    };
}

/// Macro for implementing error on a struct
#[macro_export]
macro_rules! impl_error {
    ($ty:ty) => {
        use std::error::Error;

        impl Error for $ty {
        }
    };
}