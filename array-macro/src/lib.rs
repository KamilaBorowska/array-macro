#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate array_macro_internal;
#[doc(hidden)]
pub use array_macro_internal::*;

proc_macro_expr_decl!(#[doc(hidden)] __internal_array! => internal_array_impl);

/// Array constructor macro.
///
/// This macro provides a way to repeat the same macro element multiple times
/// without requiring `Copy` implementation.
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
    [@INTERNAL $callback:expr; $count:tt] => {
        __internal_array!($count $callback)
    };
    [| $($rest:tt)*] => {
        array![@INTERNAL | $($rest)*]
    };
    [move $($rest:tt)*] => {
        array![@INTERNAL move $($rest)*]
    };
    [$expr:expr; $count:tt] => {
        array![|_| $expr; $count]
    };
}
