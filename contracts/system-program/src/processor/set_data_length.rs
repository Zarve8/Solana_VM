use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo};
use solana_program::pubkey::Pubkey;
use crate::global::ForceAccountUpdate;
use crate::processor::can_data_be_changed::can_data_be_changed;
use crate::processor::can_data_be_resized::can_data_be_resized;


pub fn set_data_length<'a>(account: &AccountInfo<'a>, new_length: usize) -> Result<(), InstructionError> {
    can_data_be_resized(account, new_length)?;
    can_data_be_changed(account)?;
    // don't touch the account if the length does not change
    if account.data_len() == new_length {
        return Ok(());
    }
    let old_length = account.data_len() as usize;
    let mut data = account.data.borrow().to_vec();
    if old_length > new_length {
        data.drain(new_length..);
    }
    else {
        data.append(&mut vec![0u8; new_length - old_length]);
    }
    ForceAccountUpdate::push_data(account.key, data);
    Ok(())
}