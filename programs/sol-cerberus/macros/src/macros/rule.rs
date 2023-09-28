use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, Stmt};

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

fn find_local_var(block: &syn::Block, var_name: &str) -> Option<usize> {
    /*
       Finds the index of a Local var statement. Returns None otherwise.
    */
    for i in 0..block.stmts.len() {
        if let Stmt::Local(local) = &block.stmts[i] {
            if let Pat::Type(pat_type) = &local.pat {
                if let Pat::Ident(ident) = &*pat_type.pat {
                    if ident.ident == var_name {
                        return Some(i);
                    }
                }
            }
        }
    }
    None
}

pub fn rule_macro(args: TokenStream, item: TokenStream) -> TokenStream {
    let (resource, permission) = extract_rule(&args);
    let mut item: ItemFn = parse_macro_input!(item as ItemFn);

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

    // Find Sol Cerberus app ID on the current function scope (if defined)
    let sc_app_id_index = find_local_var(&item.block, "sol_cerberus_app_id");
    let sc_app_id_var_name: TokenStream2;
    // Get app ID from local var "sol_cerberus_app_id"
    if sc_app_id_index.is_some() {
        // Add it to the beginning of the function block
        let var_id: Stmt = item.block.stmts.remove(sc_app_id_index.unwrap());
        item.block.stmts.insert(0, var_id);
        sc_app_id_var_name = quote! { sol_cerberus_app_id }
    // Get app ID from global constant "SOL_CERBERUS_APP_ID"
    } else {
        sc_app_id_var_name = quote! { SOL_CERBERUS_APP_ID }
    };
    let ctx_arg_name: Ident = ctx_arg_name.unwrap();
    let cpi_call: TokenStream2 = quote! {
        sol_cerberus::cpi::allowed(
            #ctx_arg_name.accounts.sol_cerberus_ctx(),
            sol_cerberus::instructions::AllowedRule {
                app_id: #sc_app_id_var_name,
                namespace: 0 as u8, // Rule
                resource: #resource.to_string(),
                permission: #permission.to_string(),
            }
        )?;
    };
    // Add the CPI call either:
    //  - At the 1st place of the function block
    //  - At the 2nd place of the function block (when "sol_cerberus_app_id" is defined at 1st place)
    item.block.stmts.insert(
        if sc_app_id_index.is_some() { 1 } else { 0 },
        syn::parse2(cpi_call).unwrap(),
    );
    // eprintln!("GENERATED:\r\n{:#?}", quote!(#item).to_string());
    proc_macro::TokenStream::from(quote!(#item))
}
