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
//! assert_eq!(array![String::from("x"); 2], [String::from("x"), String::from("x")]);
//! assert_eq!(array![|x| x; 3], [0, 1, 2]);
//! # }
//! ```

#![no_std]

#[doc(hidden)]
pub extern crate core as __core;

#[allow(unused_imports)]
use core::mem::MaybeUninit; // Rust 1.36+ required

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
#[macro_export(local_inner_macros)]
macro_rules! array {
    [@INTERNAL $callback:expr; $count:expr] => {{
        let callback = $callback;
        const COUNT: usize = $count;
        struct ArrayVec<T> {
            start: *mut T,
            position: usize,
        }
        impl<T> Drop for ArrayVec<T> {
            fn drop(&mut self) {
                for i in 0..self.position {
                    unsafe {
                        $crate::__core::ptr::drop_in_place(self.start.add(i));
                    }
                }
            }
        }
        #[allow(unsafe_code)]
        fn create_arr<T>(mut callback: impl FnMut(usize) -> T) -> [T; COUNT] {
            let mut arr = $crate::__core::mem::MaybeUninit::uninit();
            let mut vec = ArrayVec {
                start: arr.as_mut_ptr() as *mut T,
                position: 0,
            };
            unsafe {
                for i in 0..COUNT {
                    vec.position = i;
                    $crate::__core::ptr::write(vec.start.add(i), callback(i));
                }
                $crate::__core::mem::forget(vec);
                arr.assume_init()
            }
        }
        create_arr(callback)
    }};
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
