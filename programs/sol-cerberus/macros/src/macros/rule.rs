use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat};

pub fn valid_rule(text: &String) -> bool {
    if text.is_empty() || text.as_bytes().len() > 16 {
        return false;
    }
    for char in text.chars() {
        if !char.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}
pub fn extract_rule(args: &TokenStream) -> (String, String) {
    /*
        Extracts "resource" and "permission" arguments from the "rule" macro.
        Example of a valid macro:

            #[rule(Dashboard, Write)]
            pub fn some_function(){
                ...
            }
    */
    let args_str: String = args.to_string();
    let args: Vec<String> = args_str
        .split(',')
        .map(|s| String::from(s).replace(" ", ""))
        .collect();
    if args.len() == 2 && valid_rule(&args[0]) && valid_rule(&args[1]) {
        return (args[0].to_string(), args[1].to_string());
    }
    panic!(
        "The #[rule] macro requires the resource and permission params (only ASCII alphanumeric \
               characters allowed). E.g:  #[rule(Comments, Write)]"
    );
}

pub fn rule_macro(args: TokenStream, item: TokenStream) -> TokenStream {
    let (resource, permission) = extract_rule(&args);
    let mut item = parse_macro_input!(item as ItemFn);

    // Get context argument name
    let ctx_arg_name: Option<Ident> = match item.sig.inputs.first() {
        Some(FnArg::Typed(arg)) => match &*arg.pat {
            Pat::Ident(ident) => Some(ident.ident.clone()),
            _ => None,
        },
        _ => None,
    };
    // Return unmodified function if there are no arguments
    if ctx_arg_name.is_none() {
        return proc_macro::TokenStream::from(quote!(#item));
    }

    let ctx_arg_name: Ident = ctx_arg_name.unwrap();
    // Add the CPI call at the beginning of the function
    let cpi_call: TokenStream2 = quote::quote! {
        let _ = sol_cerberus::cpi::allowed(
            #ctx_arg_name.accounts.sol_cerberus_ctx(),
            sol_cerberus::instructions::AllowedRule {
                app_id: SOL_CERBERUS_APP_ID,
                namespace: 0 as u8, // Rule
                resource: #resource.to_string(),
                permission: #permission.to_string(),
            }
        );
    };
    // Add the new statement to the beginning of the function block
    let block = &mut item.block;
    block.stmts.insert(0, syn::parse2(cpi_call).unwrap());
    proc_macro::TokenStream::from(quote!(#item))
}
