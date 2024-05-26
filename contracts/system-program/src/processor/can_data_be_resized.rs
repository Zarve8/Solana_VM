use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo};
use solana_program::pubkey::Pubkey;
use crate::processor::set_account_owner::SYSTEM_PROGRAM;


pub const MAX_PERMITTED_DATA_LENGTH: u64 = 10 * 1024 * 1024;
pub const MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION: i64 =
    MAX_PERMITTED_DATA_LENGTH as i64 * 2;


pub fn can_data_be_resized<'a>(account: &AccountInfo<'a>, new_length: usize) -> Result<(), InstructionError> {
    let old_length = account.data_len();
    // Only the owner can change the length of the data
    if new_length != old_length && !account.owner.eq(&SYSTEM_PROGRAM) {
        return Err(InstructionError::AccountDataSizeChanged);
    }
    // The new length can not exceed the maximum permitted length
    if new_length > MAX_PERMITTED_DATA_LENGTH as usize {
        return Err(InstructionError::InvalidRealloc);
    }
    // The resize can not exceed the per-transaction maximum
    let length_delta = (new_length as i64).saturating_sub(old_length as i64);
    if length_delta > MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION
    {
        return Err(InstructionError::MaxAccountsDataAllocationsExceeded);
    }
    Ok(())
}