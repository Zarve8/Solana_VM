use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo};
use solana_program::pubkey::Pubkey;
use crate::processor::set_account_owner::SYSTEM_PROGRAM;


pub fn can_data_be_changed<'a>(account: &AccountInfo<'a>) -> Result<(), InstructionError> {
    // Only non-executable accounts data can be changed
    if account.executable {
        return Err(InstructionError::ExecutableDataModified);
    }
    // and only if the account is writable
    if !account.is_writable{
        return Err(InstructionError::ReadonlyDataModified);
    }
    // and only if we are the owner
    if !account.owner.eq(&SYSTEM_PROGRAM){
        return Err(InstructionError::ExternalAccountDataModified);
    }
    Ok(())
}