pub use deep_flatten::OptionDeepFlatten;

/// Deeply converts self into an [Option<T>], consuming self, and discarding any errors.
pub trait ResultDeepOk<T, E> {
    /// Convert nested Results e.g. `Result<Result<Result<T, E>, E>, E>` into Option<T>, discarding any errors.
    /// Can convert up to 32 Results.
    fn deep_ok(self) -> Option<T>;
}

macro_rules! __impl_result {
    ($_ignored:ident) => {};
    ($_ignored:ident $($t:ident)+) => {
        impl<T, E> ResultDeepOk<T, E> for __impl_result!(@nest_result $($t)+) {
            fn deep_ok(self) -> Option<T> {
                match self {
                    __impl_result!(@nest_ok(inner) $($t)+) => Some(inner),
                    _ => None,
                }
            }
        }
        __impl_result!($($t)*);
    };

    (@nest_result $_ignored:ident) => { Result<T, E> };
    (@nest_result $_ignored:ident $($t:ident)+) => {
        Result<__impl_result!(@nest_result $($t)+), E>
    };
    (@nest_ok($dst:ident) $($_ignored:ident)?) => { Ok($dst) };
    (@nest_ok($dst:ident) $_ignored:ident $($t:ident)+) => {
        Ok(__impl_result!(@nest_ok($dst) $($t)+))
    };
}

// Impl for up to 32 Results.
__impl_result!(IGNORED
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
    Result
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_deep_ok() {
        let x: Result<Result<i32, &str>, &str> = Ok(Ok(1));
        let y: Result<Result<Result<i32, &str>, &str>, &str> = Ok(Ok(Ok(1)));
        let z: Result<Result<Result<Result<i64, &str>, &str>, &str>, &str> = Ok(Ok(Ok(Ok(1))));

        assert_eq!(x.deep_ok(), Some(1));
        assert_eq!(y.deep_ok(), Some(1));
        assert_eq!(z.deep_ok(), Some(1));
    }
}
