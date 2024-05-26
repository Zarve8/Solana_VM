use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo};
use solana_program::msg;
use solana_program::pubkey::Pubkey;
use crate::processor::set_account_owner::set_account_owner;


pub fn assign<'a>(account: &AccountInfo<'a>, owner: &Pubkey) -> Result<(), InstructionError> {
    if account.owner.eq(owner) {
        return Ok(());
    }
    if !account.is_signer {
        msg!("Assign: account {:?} must sign", account.key);
        return Err(InstructionError::MissingRequiredSignature);
    }
    set_account_owner(account, owner)
}
