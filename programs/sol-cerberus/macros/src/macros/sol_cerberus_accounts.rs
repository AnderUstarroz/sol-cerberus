use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Field, Fields, ItemStruct, Lifetime};

fn is_signer(field: &syn::Field) -> bool {
    if let syn::Type::Path(ref path) = field.ty {
        if let Some(ref segment) = path.path.segments.first() {
            return segment.ident == "Signer";
        }
    }

    false
}

fn parse_field(quoted_field: TokenStream2) -> Field {
    syn::Field::parse_named.parse2(quoted_field).unwrap()
}

pub fn sol_cerberus_accounts_macro<'info>(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);
    // Get Signer
    let mut signer: Option<Ident> = None;
    // Remove all fields starting by "sol_cerberus" to protect namespace
    let mut new_fields = match item.fields {
        Fields::Named(named_fields) => {
            let mut new_named_fields = named_fields.clone();
            new_named_fields.named = named_fields
                .named
                .into_iter()
                .filter(|field| {
                    if is_signer(field) {
                        signer = field.ident.clone();
                    }
                    if let Some(ref ident) = field.ident {
                        !ident.to_string().starts_with("sol_cerberus")
                    } else {
                        true
                    }
                })
                .collect();
            Fields::Named(new_named_fields)
        }
        other_fields => other_fields,
    };

    if signer.is_none() {
        panic!("Structs annotated with #[sol_cerberus_accounts] require a Signer")
    }
    // Get lifetime
    if !item.generics.lifetimes().next().is_some() {
        panic!("Structs annotated with #[sol_cerberus_accounts] require a lifetime param. E.g: pub struct MyStruct<'info>")
    }
    let lifetime: &Lifetime = &item.generics.lifetimes().next().unwrap().lifetime;
    // Add required Sol Cerberus accounts to struct:
    if let syn::Fields::Named(ref mut fields) = new_fields {
        fields.named.push(parse_field(quote! {
            /// CHECK: Validated on CPI call
            pub sol_cerberus_app: UncheckedAccount<#lifetime>
        }));
        fields.named.push(parse_field(quote! {
            /// CHECK: Validated on CPI call
            pub sol_cerberus_rule: Option<UncheckedAccount<#lifetime>>
        }));
        fields.named.push(parse_field(quote! {
            /// CHECK: Validated on CPI call
            pub sol_cerberus_role: Option<UncheckedAccount<#lifetime>>
        }));
        fields.named.push(parse_field(quote! {
            #[cfg_attr(not(test), account())]
            pub sol_cerberus_token: Option<Box<Account<#lifetime, anchor_spl::token::TokenAccount>>>
        }));
        fields.named.push(parse_field(quote! {
            #[cfg_attr(not(test), account(
                seeds = [b"metadata", sol_cerberus::mpl_token_metadata::ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()],
                seeds::program = sol_cerberus::mpl_token_metadata::ID,
                bump,
            ))]
            pub sol_cerberus_metadata: Option<Box<Account<#lifetime, anchor_spl::metadata::MetadataAccount>>>
        }));
        fields.named.push(parse_field(quote! {
            pub sol_cerberus: Program<'info, SolCerberus>
        }));
    }
    // Replace fields
    item.fields = new_fields;
    let struct_name = &item.ident;
    let result = quote! {
        #item
        impl<'info> #struct_name<'info> {
            pub fn sol_cerberus_ctx(&self) -> CpiContext<'_, '_, '_, 'info, sol_cerberus::cpi::accounts::Allowed<'info>> {
                let cpi_program = self.sol_cerberus.to_account_info();
                let cpi_accounts = sol_cerberus::cpi::accounts::Allowed {
                    signer: self.signer.to_account_info(),
                    sol_cerberus_app: self.sol_cerberus_app.to_account_info(),
                    sol_cerberus_rule: match self.sol_cerberus_rule.as_ref() {
                        None => None,
                        Some(x) => Some(x.to_account_info()),
                    },
                    sol_cerberus_role: match self.sol_cerberus_role.as_ref() {
                        None => None,
                        Some(x) => Some(x.to_account_info()),
                    },
                    sol_cerberus_token: match self.sol_cerberus_token.as_ref() {
                        None => None,
                        Some(x) => Some(x.to_account_info()),
                    },
                    sol_cerberus_metadata: match self.sol_cerberus_metadata.as_ref() {
                        None => None,
                        Some(x) => Some(x.to_account_info()),
                    },
                };
                CpiContext::new(cpi_program, cpi_accounts)
            }
        }
    };
    // println!("GENERATED:\n\n{:#?}", result.to_string());
    proc_macro::TokenStream::from(result)
}
