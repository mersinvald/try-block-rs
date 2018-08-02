/// Macro to make your error-handling blocks (appear) lambda-less
///
/// #### Before:
/// ```
/// let result: Result<T, E> = || {
///    let a = do_one(x)?;
///    let b = do_two(a)?;
///    Ok(b)
/// }();
/// ```
///
/// #### After:
/// ```    
/// let result: Result<T, E> = try_block! {
///    let a = do_one(x)?;
///    let b = do_two(a)?;
///    Ok(b)
/// };
/// ```

#[macro_export]
macro_rules! try_block {
    { $($token:tt)* } => {{
        let l = || {
            $($token)*
        };
        l()
    }}
}

