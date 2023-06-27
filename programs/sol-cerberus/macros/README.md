<div align="center">
  <img height="170x" src="https://raw.githubusercontent.com/AnderUstarroz/sol-cerberus-website/main/public/images/logo.webp" />

  <h1>Sol Cerberus</h1>
    <p>
        <strong>The new authority</strong>
    </p>
    <p>
        <a href="https://solcerberus.com/"><img alt="Website" src="https://img.shields.io/website?up_message=online&url=http%3A%2F%2Fsolcerberus.com%2F"></a>
        <a href="https://docs.solcerberus.com/"><img alt="Website" src="https://img.shields.io/website?down_message=offline&label=docs&up_color=blueviolet&up_message=online&url=https%3A%2F%2Fdocs.solcerberus.com%2F"></a>
        <a href="https://demo.solcerberus.com/?id=CeRb3rUsMaSMgQDAanF9S5Fgk75ShELtECtvjPsb2fEj"><img alt="Website" src="https://img.shields.io/website?down_message=offline&label=demo&up_color=yellow&up_message=online&url=https%3A%2F%2Fdemo.solcerberus.com%2F"></a>
        <a href="https://crates.io/crates/sol-cerberus"><img alt="Crates.io" src="https://img.shields.io/crates/v/sol-cerberus?color=blue"></a>
    </p>
</div>

# sol-cerberus-macros
Collection of usefull Anchor macros to abstract away the complexity of Sol Cerberus RBAC, integrating a full access constrol system into your program with just a few lines of code.

- [Sol Cerberus website](https://solcerberus.com/)
- [Docs](https://docs.solcerberus.com/)
- [Demo](https://demo.solcerberus.com/)

## Installation
To install the latest version, add `sol-cerberus-macros` into the dependencies of your **Cargo.toml** file:
```
[dependencies]
sol-cerberus-macros  = "*"
```


##  #[rule (Resource, Permission)] macro
The `#[rule]` macro annotates Anchor instructions, it checks if the current user running the instruction is allowed to access the defined `Resource` and `Permission`. For instance the following rule macro example
allows access only to the roles which are allowed to access the Resource `Homepage` and the Permission `Write`:


```
declare_id!("AjO97SU3FWq652tMMzNSbmPMeM4jtKDP3nLJp9APctFA");

const SOL_CERBERUS_APP_ID: Pubkey = pubkey!("9R5QMs9rEJ6BMvSF84yw91qnRBXKEBJbeQnZVX84C3");

#[program]
pub mod my_program {
    use super::*;

    #[rule(Homepage, Write)]
    pub fn my_instruction(_ctx: Context<MyContext>) -> Result<()> {
         Ok(())
    }
}
```
If some user tries to run this instruction without having the mentioned  permissions, will get an `Unauthorized` error.


### #[sol_cerberus_accounts] macro

The `#[sol_cerberus_accounts]` macro, annotates Anchor accounts, adding all the necessary accounts to perform the permission check. A full working example using the `#[rule]` and `#[sol_cerberus_accounts]` macros would look like this:


```
declare_id!("AjO97SU3FWq652tMMzNSbmPMeM4jtKDP3nLJp9APctFA");

pub const SOL_CERBERUS_APP_ID: &'static str = "9R5QMs9rEJ6BMvSF84yw91qnRBXKEBJbeQnZVX84C3";

#[program]
pub mod my_program {
    use super::*;

    #[rule(Homepage, Write)]
    pub fn my_instruction(_ctx: Context<MyContext>) -> Result<()> {
         Ok(())
    }
}

#[sol_cerberus_accounts]
#[derive(Accounts)]
pub struct MyContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Validated on CPI call
    pub sol_cerberus_app: UncheckedAccount<'info>,
    /// CHECK: Validated on CPI call
    pub sol_cerberus_rule: Option<UncheckedAccount<'info>>,
    /// CHECK: Validated on CPI call
    pub sol_cerberus_role: Option<UncheckedAccount<'info>>,
    /// CHECK: Validated on CPI call
    pub sol_cerberus_token: Option<UncheckedAccount<'info>>,
    /// CHECK: Validated on CPI call
    pub sol_cerberus_metadata: Option<UncheckedAccount<'info>>,
    #[account(mut)]
    pub sol_cerberus_seed: Option<UncheckedAccount<'info>>,
    pub sol_cerberus: Program<'info, SolCerberus>,
    pub system_program: Program<'info, System>,
}

```

These are the accounts required by Sol Cerberus to verify user access. Hopefully in future versions of Anchor adding all those `UncheckedAccounts` will not be necessary because `#[sol_cerberus_accounts]` automatically adds all of them. But  Anchor currently requires the accounts to be explicitly defined to be able to build the IDL.
