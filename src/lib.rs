//! Array multiple elements constructor syntax.
//!
//! While Rust does provide those, they require copy, and you cannot obtain the
//! index that will be created. This crate provides syntax that fixes both of
//! those issues.
//!
//! # Examples
//!
//! ```
//! # #[macro_use]
//! # extern crate array_macro;
//! # fn main() {
//! assert_eq!(array!["string"; 3], ["string", "string", "string"]);
//! assert_eq!(array![|x| x; 3], [0, 1, 2]);
//! # }
//! ```

#![no_std]

#[doc(hidden)]
pub extern crate core as __core;

/// Array constructor macro.
///
/// This macro provides a way to repeat the same macro element multiple times
/// without requiring `Copy` implementation.
///
/// It's possible to define a callback by starting expression with `|` or `move`. As
/// every closure is it own unique type, it is not possible to have an array of
/// closures, so this syntax was reused for creating arrays with known indexes.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate array_macro;
/// # fn main() {
/// assert_eq!(array!["string"; 3], ["string", "string", "string"]);
/// assert_eq!(array![|x| x; 3], [0, 1, 2]);
/// # }
/// ```
#[macro_export]
macro_rules! array {
    [@INTERNAL $callback:expr; $count:expr] => {
        unsafe {
            let mut arr: [_; $count] = ::array_macro::__core::mem::uninitialized();
            for (i, elem) in arr.iter_mut().enumerate() {
                ::array_macro::__core::ptr::write(elem, $callback(i));
            }
            arr
        }
    };
    [| $($rest:tt)*] => {
        array![@INTERNAL | $($rest)*]
    };
    [move $($rest:tt)*] => {
        array![@INTERNAL move $($rest)*]
    };
    [$expr:expr; $count:expr] => {
        array![|_| $expr; $count]
    };
}
