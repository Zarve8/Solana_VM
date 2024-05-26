use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo};
use solana_program::pubkey::Pubkey;
use crate::global::ForceAccountUpdate;


pub const SYSTEM_PROGRAM: Pubkey = Pubkey::new_from_array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);


pub fn set_account_owner<'a>(account: &AccountInfo<'a>, owner: &Pubkey) -> Result<(), InstructionError> {
    if !account.owner.eq(&SYSTEM_PROGRAM){
        return Err(InstructionError::ModifiedProgramId);
    }
    // and only if the account is writable
    if !account.is_writable {
        return Err(InstructionError::ModifiedProgramId);
    }
    // and only if the account is not executable
    if account.executable {
        return Err(InstructionError::ModifiedProgramId);
    }
    // and only if the data is zero-initialized or empty
    // !is_zeroed(self.get_data())
    if account.data_len() != 0 {
        return Err(InstructionError::ModifiedProgramId);
    }
    // don't touch the account if the owner does not change
    if account.owner.eq(owner){
        return Ok(());
    }
    ForceAccountUpdate::push_owner(account.key, owner.clone());
    Ok(())
}
