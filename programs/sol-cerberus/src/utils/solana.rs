use crate::Errors::InsufficientFunds;
use anchor_lang::prelude::*;

/// Transfers lamports (From account must be owned by Sol Cerberus)
/// to another account. The recipient can by any account
pub fn transfer_lamports(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    lamports: u64,
) -> Result<()> {
    // Does the from account have enough lamports to transfer?
    if **from_account.try_borrow_lamports()? < lamports {
        return Err(error!(InsufficientFunds));
    }
    // Debit from_account and credit to_account
    **from_account.try_borrow_mut_lamports()? -= lamports;
    **to_account.try_borrow_mut_lamports()? += lamports;
    Ok(())
}
