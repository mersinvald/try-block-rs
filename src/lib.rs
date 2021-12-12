/// Macro to make your error-handling blocks (appear) lambda-less
/// and perform Ok-wrapping on the final value.
///
/// #### Before:
/// ```ignore
/// let result: Result<T, E> = || {
///    let a = do_one(x)?;
///    let b = do_two(a)?;
///    Ok(b)
/// }();
/// ```
///
/// #### After:
/// ```ignore
/// let result: Result<T, E> = try_block! {
///    let a = do_one(x)?;
///    let b = do_two(a)?;
///    b
/// };
/// ```

#[macro_export]
macro_rules! try_block {
    { $($token:tt)* } => {{
        ( || $crate::FromOk::from_ok(
            { $($token)* }
        ))()
    }}
}

/// A type that can Ok-wrap some value, for use in the [`try_block`] macro.
pub trait FromOk {
    type Ok;
    /// Constructs the wrapped type from an ok value.
    fn from_ok(val: Self::Ok) -> Self;
}

impl<T, E> FromOk for Result<T, E> {
    type Ok = T;
    #[inline]
    fn from_ok(val: T) -> Self {
        Ok(val)
    }
}

impl<T> FromOk for Option<T> {
    type Ok = T;
    #[inline]
    fn from_ok(val: T) -> Self {
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_sum() {
        let result: Result<_, std::num::ParseIntError> = try_block! {
            let x = "1".parse::<i32>()?;
            let x = "2".parse::<i32>()? + x * 10;
            let x = "3".parse::<i32>()? + x * 10;
            x
        };
        assert_eq!(result, Ok(123));
    }

    #[test]
    fn option() {
        assert_eq!(
            Some(520),
            try_block! {
                "400".parse::<i32>().ok()? + "20".parse::<i32>().ok()? * "6".parse::<i32>().ok()?
            },
        );
    }
}
