//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
///
/// If scenario where function could panic, document here in this common heading.
///
/// # Errors
///
/// If the function returns a `Result`, describe the kinds of errors and what
/// conditions might cause those errors.
///
/// # Safety
///
/// If the function is `unsafe` to call, explain why `unsafe` and cover invariants
/// that the function expects callers to uphold.
pub fn add_one(x: i32) -> i32 {
    x + 1
}
