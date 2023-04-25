#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use sol_cerberus_macros::sol_cerberus_accounts;

    // Mocking TokenAccount for testing
    mod anchor_spl {

        pub mod token {
            #[derive(Debug)]
            pub struct TokenAccount {
                account: String,
            }
            impl AsRef<str> for TokenAccount {
                fn as_ref(&self) -> &str {
                    &self.account
                }
            }
        }
        pub mod metadata {
            #[derive(Debug)]
            pub struct MetadataAccount {
                account: String,
            }
            impl AsRef<str> for MetadataAccount {
                fn as_ref(&self) -> &str {
                    &self.account
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct AccountInfo<'info> {
        pub account: &'info str,
    }

    // Mocking Account for testing
    #[derive(Debug)]
    pub struct Account<'info, T> {
        pub account: &'info T,
    }

    impl<'info, T: AsRef<str>> Account<'info, T> {
        fn to_account_info(&self) -> AccountInfo<'info> {
            AccountInfo {
                account: self.account.as_ref(),
            }
        }
    }

    #[derive(Debug)]
    pub struct UncheckedAccount<'info> {
        pub account: &'info str,
    }
    impl<'info> UncheckedAccount<'info> {
        fn to_account_info(&self) -> AccountInfo {
            AccountInfo {
                account: self.account,
            }
        }
    }

    mod sol_cerberus {
        pub mod cpi {
            pub mod accounts {
                use crate::tests::AccountInfo;

                #[derive(Debug)]
                pub struct Allowed<'info> {
                    signer: AccountInfo<'info>,
                    sol_cerberus_app: AccountInfo<'info>,
                    sol_cerberus_rule: AccountInfo<'info>,
                    sol_cerberus_role: Option<AccountInfo<'info>>,
                    sol_cerberus_token_acc: Option<AccountInfo<'info>>,
                    sol_cerberus_metadata: Option<AccountInfo<'info>>,
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct SolCerberus {
        name: String,
    }

    impl AsRef<str> for SolCerberus {
        fn as_ref(&self) -> &str {
            self.name.as_ref()
        }
    }

    // Mocking Program for testing
    #[derive(Debug)]
    pub struct Program<'info, T> {
        program: &'info T,
    }

    impl<'info, T: AsRef<str>> Program<'info, T> {
        fn to_account_info(&self) -> AccountInfo {
            AccountInfo {
                account: self.program.as_ref(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Signer {
        name: String,
    }

    impl Signer {
        fn to_account_info(&self) -> AccountInfo {
            AccountInfo {
                account: &self.name,
            }
        }
    }

    pub struct CpiContext<'a, 'b, 'c, 'info, T> {
        pub accounts: T,
        pub program: AccountInfo<'info>,
        pub signer_seeds: &'a [&'b [&'c [u8]]],
    }

    impl<'a, 'b, 'c, 'info, T> CpiContext<'a, 'b, 'c, 'info, T> {
        pub fn new(program: AccountInfo<'info>, accounts: T) -> Self {
            Self {
                accounts,
                program,
                signer_seeds: &[],
            }
        }
    }

    #[test]
    fn test_accounts_macro() {
        // #[sol_cerberus_accounts]
        // #[derive(Debug)]
        // pub struct MyAccounts<'info> {
        //     #[cfg_attr(not(test), account())]
        //     pub signer: Signer,
        // }

        // #[derive(Debug)]
        // pub struct MyAccounts<'info> {
        //     #[cfg_attr(not(test), account())]
        //     pub signer: Signer,
        //     #[doc = r" CHECK: Validated on CPI call"]
        //     pub sol_cerberus_app: UncheckedAccount<'info>,
        //     #[doc = r" CHECK: Validated on CPI call"]
        //     pub sol_cerberus_rule: UncheckedAccount<'info>,
        //     #[doc = r" CHECK: Validated on CPI call"]
        //     pub sol_cerberus_role: Option<UncheckedAccount<'info>>,
        //     #[cfg_attr(not(test), account(
        //         constraint = sol_cerberus_token_acc.mint == sol_cerberus_metadata.as_ref().unwrap().mint @ sol_cerberus :: errors ::Errors :: Unauthorized,
        //         constraint = sol_cerberus_token_acc.owner == signer.key() @ sol_cerberus :: errors :: Errors :: Unauthorized))]
        //     pub sol_cerberus_token_acc: Option<Account<'info, anchor_spl::token::TokenAccount>>,
        //     #[cfg_attr(not(test), account(
        //         seeds = [b"metadata", mpl_token_metadata :: ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()], seeds :: program = mpl_token_metadata :: ID, bump,))]
        //     pub sol_cerberus_metadata:
        //         Option<Account<'info, anchor_spl::metadata::MetadataAccount>>,
        //     pub sol_cerberus: Program<'info, SolCerberus>,
        // }
        // impl<'info> MyAccounts<'info> {
        //     pub fn sol_cerberus_ctx(
        //         &self,
        //     ) -> CpiContext<'_, '_, '_, 'info, sol_cerberus::cpi::accounts::Allowed<'info>>
        //     {
        //         let cpi_program = self.sol_cerberus.to_account_info();
        //         let cpi_accounts = sol_cerberus::cpi::accounts::Allowed {
        //             signer: self.signer.to_account_info(),
        //             sol_cerberus_app: self.sol_cerberus_app.to_account_info(),
        //             sol_cerberus_rule: self.sol_cerberus_rule.to_account_info(),
        //             sol_cerberus_role: match self.sol_cerberus_role.as_ref() {
        //                 None => None,
        //                 Some(x) => Some(x.to_account_info()),
        //             },
        //             sol_cerberus_token_acc: match self.sol_cerberus_token_acc.as_ref() {
        //                 None => None,
        //                 Some(x) => Some(x.to_account_info()),
        //             },
        //             sol_cerberus_metadata: match self.sol_cerberus_metadata.as_ref() {
        //                 None => None,
        //                 Some(x) => Some(x.to_account_info()),
        //             },
        //         };
        //         CpiContext::new(cpi_program, cpi_accounts)
        //     }
        // }

        // let token_acc = anchor_spl::token::TokenAccount {
        //     account: "token_acc".to_string(),
        // };
        // let metadata_acc = anchor_spl::metadata::MetadataAccount {
        //     account: "metadata_acc".to_string(),
        // };
        // let instance = MyAccounts {
        //     signer: Signer {
        //         name: "authority".to_string(),
        //     },
        //     sol_cerberus_app: UncheckedAccount { msg: &"app" },
        //     sol_cerberus_rule: UncheckedAccount { msg: &"rule" },
        //     sol_cerberus_role: Some(UncheckedAccount { msg: &"role" }),
        //     sol_cerberus_token_acc: Some(Account {
        //         account: &token_acc,
        //     }),
        //     sol_cerberus_metadata: Some(Account {
        //         account: &metadata_acc,
        //     }),
        //     sol_cerberus: Program {
        //         program: &SolCerberus {
        //             name: "SolCerberus".to_string(),
        //         },
        //     },
        // };
        // assert_eq!(instance.signer.name, "authority".to_string());
    }
}
