/// Macro for ok-wrapping any `Try` type. This works on stable through dark type magic.
///
/// Note that type inference is very finicky; you should give this a type ascription ASAP.
/// ```
/// # use try_block::wrap_ok;
/// let r: Result<_, ()> = wrap_ok!(1);
/// assert_eq!(r, Ok(1));
/// ```
#[macro_export]
macro_rules! wrap_ok {
    ($e:expr) => {{
        ::std::iter::empty().try_fold($e, |_, x: std::convert::Infallible| match x {})
    }};
}

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
/// ```
/// # use try_block::try_block;
/// # type T = (); type E = ();
/// # fn do_one((): T) -> Result<T, E> { Ok(()) }
/// # fn do_two((): T) -> Result<T, E> { Ok(()) }
/// # let x = ();
/// let result: Result<T, E> = try_block! {
///    let a = do_one(x)?;
///    let b = do_two(a)?;
///    b
/// };
/// ```
#[macro_export]
macro_rules! try_block {
    { $($token:tt)* } => {{
        ( || $crate::wrap_ok!(
            { $($token)* }
        ))()
    }}
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
