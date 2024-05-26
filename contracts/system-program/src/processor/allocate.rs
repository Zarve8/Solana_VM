use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo};
use solana_program::msg;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction::SystemError;
use crate::processor::can_data_be_resized::MAX_PERMITTED_DATA_LENGTH;
use crate::processor::set_account_owner::{SYSTEM_PROGRAM};
use crate::processor::set_data_length::set_data_length;


pub fn allocate<'a>(account: &AccountInfo<'a>, space: u64) -> Result<(), InstructionError> {
    if !account.is_signer {
        msg!(
            "Allocate: 'to' account {:?} must sign",
            account.key
        );
        return Err(InstructionError::MissingRequiredSignature);
    }
    // if it looks like the `to` account is already in use, bail
    //   (note that the id check is also enforced by message_processor)
    if !(account.data_len() == 0) || !account.owner.eq(&SYSTEM_PROGRAM) {
        msg!(
            "Allocate: account {:?} already in use",
            account.key
        );
        return Err(SystemError::AccountAlreadyInUse.into());
    }
    if space > MAX_PERMITTED_DATA_LENGTH {
        msg!(
            "Allocate: requested {}, max allowed {}",
            space,
            MAX_PERMITTED_DATA_LENGTH
        );
        return Err(SystemError::InvalidAccountDataLength.into());
    }
    set_data_length(account, space as usize)?;
    Ok(())
}
