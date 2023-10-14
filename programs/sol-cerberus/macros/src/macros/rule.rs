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

fn extract_sc_local_vars(block: &mut syn::Block, var_values: &mut [TokenStream2; 3]) {
    /*
       Extracts the following variables from the function body (when present) to apply the dynamically assigned values.
         - sol_cerberus_app_id
         - sol_cerberus_resource
         - sol_cerberus_permission
    */
    let var_names: [&str; 3] = [
        "sol_cerberus_app_id",
        "sol_cerberus_resource",
        "sol_cerberus_permission",
    ];
    let mut found_indexes: Vec<usize> = Vec::new();
    for i in 0..block.stmts.len() {
        if let Stmt::Local(local) = &block.stmts[i] {
            let mut var_name: Option<&Ident> = None;
            // Local variable with type defined
            if let Pat::Type(pat_type) = &local.pat {
                if let Pat::Ident(ident) = &*pat_type.pat {
                    var_name = Some(&ident.ident);
                }
            // Local variable without type defined
            } else if let Pat::Ident(ident) = &local.pat {
                var_name = Some(&ident.ident);
            }
            // Find the SC variable and extract the value
            if var_name.is_some() {
                for k in 0..3 {
                    if var_name.unwrap() == var_names[k] {
                        if let Some(init) = &local.init {
                            let value = &init.expr;
                            var_values[k] = quote! {#value};
                            found_indexes.push(i);
                        }
                        break;
                    }
                }
            }
        }
    }
    // Order array in descendent order to avoid breaking block.stmts vector while deleting
    // the variables that are going to be replaced.
    found_indexes.sort_by(|a, b| b.cmp(a));
    // Delete found statements from the function block
    for index in &found_indexes {
        block.stmts.remove(*index);
    }
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

    // Sol Cerberus var defaults:
    let mut sc_vars: [TokenStream2; 3] = [
        quote! { SOL_CERBERUS_APP_ID },
        quote! { #resource },
        quote! { #permission },
    ];
    // Find Sol Cerberus app ID on the current function scope (when defined)
    extract_sc_local_vars(&mut item.block, &mut sc_vars);
    let sc_app_id: &TokenStream2 = &sc_vars[0];
    let sc_resource: &TokenStream2 = &sc_vars[1];
    let sc_permission: &TokenStream2 = &sc_vars[2];
    let ctx_arg_name: Ident = ctx_arg_name.unwrap();
    let cpi_call: TokenStream2 = quote! {
        sol_cerberus::cpi::allowed(
            #ctx_arg_name.accounts.sol_cerberus_ctx(),
            sol_cerberus::instructions::AllowedRule {
                app_id: #sc_app_id,
                namespace: 0 as u8, // Rule
                resource: #sc_resource.to_string(),
                permission: #sc_permission.to_string(),
            }
        )?;
    };
    // Add the CPI call either at the beginning of the function block
    item.block.stmts.insert(0, syn::parse2(cpi_call).unwrap());
    // eprintln!("GENERATED:\r\n{:#?}", quote!(#item).to_string());
    proc_macro::TokenStream::from(quote!(#item))
}
