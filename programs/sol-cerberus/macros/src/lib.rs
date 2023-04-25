extern crate proc_macro;
use macros::{rule_macro, sol_cerberus_accounts_macro};
use proc_macro::TokenStream;

mod macros;

#[proc_macro_attribute]
pub fn rule(args: TokenStream, input: TokenStream) -> TokenStream {
    rule_macro(args, input)
}

#[proc_macro_attribute]
pub fn sol_cerberus_accounts(args: TokenStream, input: TokenStream) -> TokenStream {
    sol_cerberus_accounts_macro(args, input)
}
