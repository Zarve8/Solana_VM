use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo, next_account_info};
use std::slice::Iter;
use solana_program::msg;
use solana_program::system_instruction::SystemError;
use solana_program::pubkey::Pubkey;
use crate::processor::allocate_and_assign::allocate_and_assign;
use crate::processor::transfer::transfer_verified;


pub fn create_account<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, lamports: u64, space: u64, owner: &Pubkey) -> Result<(), InstructionError> {
    let from = next_account_info(accounts_iter)
        .map_err(|_| InstructionError::NotEnoughAccountKeys)?;
    let to = next_account_info(accounts_iter)
        .map_err(|_| InstructionError::NotEnoughAccountKeys)?;
    if to.lamports() > 0 {
        msg!(
                "Create Account: account {:?} already in use",
                to.key
            );
        return Err(SystemError::AccountAlreadyInUse.into());
    }
    allocate_and_assign(to, space, owner)?;
    if !from.is_signer {
        msg!(
            "Transfer: `from` account {} must sign",
            from.key.to_string(),
        );
        return Err(InstructionError::MissingRequiredSignature);
    }
    transfer_verified(from, to, lamports)
}