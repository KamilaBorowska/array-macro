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
//! assert_eq!(array![x => x; 3], [0, 1, 2]);
//! # }
//! ```

#![no_std]

#[doc(hidden)]
pub extern crate core as __core;

// This Token exists to prevent macro users from constructing their own
// __ArrayVec objects which have Drop implementation that could cause UB.
#[doc(hidden)]
#[non_exhaustive]
pub struct Token;

impl Token {
    #[doc(hidden)]
    #[inline]
    pub const unsafe fn new() -> Self {
        Token
    }
}

/// Creates an array containing the arguments.
///
/// This macro provides a way to repeat the same macro element multiple times
/// without requiring `Copy` implementation as array expressions require.
///
/// There are two forms of this macro.
///
/// - Create an array from a given element and size. This will `Clone` the element.
///
///   ```
///   use array_macro::array;
///   assert_eq!(array![vec![1, 2, 3]; 2], [[1, 2, 3], [1, 2, 3]]);
///   ```
///
///   Unlike array expressions this syntax supports all elements which implement
///   `Clone`.
///
/// - Create an array from a given expression that is based on index and size.
///   This doesn't require the element to implement `Clone`.
///
///   ```
///   use array_macro::array;
///   assert_eq!(array![x => x * 2; 3], [0, 2, 4]);
///   ```
///
///   This form can be used for declaring `const` variables.
///
///   ```
///   use array_macro::array;
///   const ARRAY: [String; 3] = array![_ => String::new(); 3];
///   assert_eq!(ARRAY, ["", "", ""]);
///   ```
///
/// # Limitations
///
/// When using a form with provided index it's not possible to use `break`
/// or `continue` without providing a label. This won't compile.
///
/// ```compile_fail
/// use array_macro::array;
/// loop {
///     array![_ => break; 1];
/// }
/// ```
///
/// To work-around this issue you can provide a label.
///
/// ```
/// use array_macro::array;
/// 'label: loop {
///     array![_ => break 'label; 1];
/// }
/// ```
#[macro_export]
macro_rules! array {
    [$expr:expr; $count:expr] => {{
        let value = $expr;
        $crate::array![_ => $crate::__core::clone::Clone::clone(&value); $count]
    }};
    [$i:pat => $e:expr; $count:expr] => {{
        const __COUNT: $crate::__core::primitive::usize = $count;

        #[repr(transparent)]
        struct __ArrayVec<T>(__ArrayVecInner<T>);

        impl<T> $crate::__core::ops::Drop for __ArrayVec<T> {
            fn drop(&mut self) {
                // This is safe as arr[..len] is initialized due to
                // __ArrayVecInner's type invariant.
                for val in &mut self.0.arr[..self.0.len] {
                    unsafe { val.as_mut_ptr().drop_in_place() }
                }
            }
        }

        // Type invariant: arr[..len] must be initialized
        struct __ArrayVecInner<T> {
            arr: [$crate::__core::mem::MaybeUninit<T>; __COUNT],
            len: $crate::__core::primitive::usize,
            token: $crate::Token,
        }

        #[repr(C)]
        union __Transmuter<T> {
            init_uninit_array: $crate::__core::mem::ManuallyDrop<$crate::__core::mem::MaybeUninit<[T; __COUNT]>>,
            uninit_array: $crate::__core::mem::ManuallyDrop<[$crate::__core::mem::MaybeUninit<T>; __COUNT]>,
            out: $crate::__core::mem::ManuallyDrop<[T; __COUNT]>,
        }

        #[repr(C)]
        union __ArrayVecTransmuter<T> {
            vec: $crate::__core::mem::ManuallyDrop<__ArrayVec<T>>,
            inner: $crate::__core::mem::ManuallyDrop<__ArrayVecInner<T>>,
        }

        let mut vec = __ArrayVec(__ArrayVecInner {
            // An uninitialized `[MaybeUninit<_>; LEN]` is valid.
            arr: $crate::__core::mem::ManuallyDrop::into_inner(unsafe {
                __Transmuter {
                    init_uninit_array: $crate::__core::mem::ManuallyDrop::new($crate::__core::mem::MaybeUninit::uninit()),
                }
                .uninit_array
            }),
            // Setting len to  0 is safe. Type requires that arr[..len] is initialized.
            // For 0, this is arr[..0] which is an empty array which is always initialized.
            len: 0,
            // This is an unsafe token that is a promise that we will follow type
            // invariant. It needs to exist as __ArrayVec is accessible for macro
            // callers, and we don't want them to cause UB if they go out of the way
            // to create new instances of this type.
            token: unsafe { $crate::Token::new() },
        });
        // Loop invariant: vec.0.arr[..vec.0.len] is valid
        while vec.0.len < __COUNT {
            let $i = vec.0.len;
            let _please_do_not_use_continue_without_label;
            let value;
            struct __PleaseDoNotUseBreakWithoutLabel;
            loop {
                _please_do_not_use_continue_without_label = ();
                value = $e;
                break __PleaseDoNotUseBreakWithoutLabel;
            };
            // This writes an initialized element.
            vec.0.arr[vec.0.len] = $crate::__core::mem::MaybeUninit::new(value);
            // We just wrote a valid element, so we can add 1 to len, it's valid.
            vec.0.len += 1;
        }
        // When leaving this loop, vec.0.len must equal to __COUNT due
        // to loop condition. It cannot be more as len is increased by 1
        // every time loop is iterated on, and __COUNT never changes.

        // __ArrayVec is representation compatible with __ArrayVecInner
        // due to #[repr(transparent)] in __ArrayVec.
        let inner = $crate::__core::mem::ManuallyDrop::into_inner(unsafe {
            __ArrayVecTransmuter {
                vec: $crate::__core::mem::ManuallyDrop::new(vec),
            }
            .inner
        });
        // At this point the array is fully initialized, as vec.0.len == __COUNT,
        // so converting an array of potentially uninitialized elements into fully
        // initialized array is safe.
        $crate::__core::mem::ManuallyDrop::into_inner(unsafe {
            __Transmuter {
                uninit_array: $crate::__core::mem::ManuallyDrop::new(inner.arr),
            }
            .out
        })
    }};
}
