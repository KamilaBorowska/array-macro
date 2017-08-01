#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate array_macro_internal;
pub use array_macro_internal::*;

proc_macro_expr_decl!(__internal_array! => internal_array_impl);

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
        array![@INTERNAL |_| $expr; $count]
    };
}
