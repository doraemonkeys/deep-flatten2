pub use deep_flatten::OptionDeepFlatten;

/// Deeply converts self into an [Option<T>], consuming self, and discarding any errors.
pub trait ResultDeepOk<T> {
    /// Convert nested Results e.g. `Result<Result<Result<T, E1>, E2>, E3>` into Option<T>, discarding any errors.
    /// Can convert up to 32 Results.
    fn deep_ok(self) -> Option<T>;
}

macro_rules! __impl_result {
    ($_ignored:ident) => {};
    ($_ignored:ident $($t:ident)+) => {
        impl<T, $($t),+> ResultDeepOk<T> for __impl_result!(@nest_result $($t)+) {
            fn deep_ok(self) -> Option<T> {
                match self {
                    __impl_result!(@nest_ok(inner) $($t)+) => Some(inner),
                    _ => None,
                }
            }
        }
        __impl_result!($($t)*);
    };

    (@nest_result $e:ident) => { Result<T, $e> };
    (@nest_result $e:ident $($t:ident)+) => {
        Result<__impl_result!(@nest_result $($t)+), $e>
    };
    (@nest_ok($dst:ident) $($_ignored:ident)?) => { Ok($dst) };
    (@nest_ok($dst:ident) $_ignored:ident $($t:ident)+) => {
        Ok(__impl_result!(@nest_ok($dst) $($t)+))
    };
}

// Impl for up to 32 Results.
__impl_result!(IGNORED
    E32
    E31
    E30
    E29
    E28
    E27
    E26
    E25
    E24
    E23
    E22
    E21
    E20
    E19
    E18
    E17
    E16
    E15
    E14
    E13
    E12
    E11
    E10
    E9
    E8
    E7
    E6
    E5
    E4
    E3
    E2
    E1
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_deep_ok() {
        let x: Result<Result<i32, &str>, &str> = Ok(Ok(1));
        let x2: Result<Result<i32, String>, &str> = Ok(Ok(1));
        let y: Result<Result<Result<i32, &str>, String>, i32> = Ok(Ok(Ok(1)));
        let z: Result<Result<Result<Result<i64, &str>, String>, i32>, bool> = Ok(Ok(Ok(Ok(1))));

        assert_eq!(x.deep_ok(), Some(1));
        assert_eq!(x2.deep_ok(), Some(1));
        assert_eq!(y.deep_ok(), Some(1));
        assert_eq!(z.deep_ok(), Some(1));

        let y: Result<Result<Result<i32, &str>, &str>, &str> = Ok(Ok(Ok(1)));
        let z: Result<Result<Result<Result<i64, &str>, &str>, &str>, &str> = Ok(Ok(Ok(Ok(1))));

        assert_eq!(y.deep_ok(), Some(1));
        assert_eq!(z.deep_ok(), Some(1));
    }

    #[test]
    fn test_result_deep_ok_err() {
        let x: Result<Result<i32, &str>, &str> = Err("error");
        let x2: Result<Result<i32, String>, &str> = Ok(Err("error".to_string()));
        let y: Result<Result<Result<i32, &str>, String>, i32> = Err(1);
        let z: Result<Result<Result<Result<i64, &str>, String>, i32>, bool> =
            Ok(Ok(Err("1".to_string())));

        assert_eq!(x.deep_ok(), None::<i32>);
        assert_eq!(x2.deep_ok(), None::<i32>);
        assert_eq!(y.deep_ok(), None::<i32>);
        assert_eq!(z.deep_ok(), None::<i64>);
    }
}
