use proc_macro::TokenStream;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNT_POSITIVE: AtomicUsize = AtomicUsize::new(10);

#[proc_macro]
pub fn count(_args: TokenStream) -> TokenStream {
    COUNT_POSITIVE
        .fetch_add(1, Ordering::Relaxed)
        .to_string()
        .parse()
        .unwrap()
}

static COUNT_NEGATIVE: AtomicUsize = AtomicUsize::new(10);

#[proc_macro]
pub fn count_backwards(_args: TokenStream) -> TokenStream {
    COUNT_NEGATIVE
        .fetch_sub(1, Ordering::Relaxed)
        .to_string()
        .parse()
        .unwrap()
}
