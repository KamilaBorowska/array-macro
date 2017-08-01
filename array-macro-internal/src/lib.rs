#[macro_use]
extern crate proc_macro_hack;

proc_macro_expr_impl! {
    /// Add one to an expression.
    pub fn internal_array_impl(input: &str) -> String {
        let mut parts = input.splitn(2, ' ');
        let count = parts.next().unwrap().parse().unwrap();
        let expression = parts.next().unwrap();
        let mut output = format!("{{let mut __internal_callback = {}; [", expression);
        for i in 0..count {
            output += &format!("__internal_callback({}),", i);
        }
        output + "]}"
    }
}
