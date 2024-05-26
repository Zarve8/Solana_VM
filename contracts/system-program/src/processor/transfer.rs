use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo, next_account_info};
use std::slice::Iter;
use solana_program::msg;
use solana_program::system_instruction::SystemError;
use solana_program::pubkey::Pubkey;


pub fn transfer_verified<'a>(from: &AccountInfo<'a>, to: &AccountInfo<'a>, lamports: u64) -> Result<(), InstructionError> {
    if from.data_len() != 0{
        msg!("Transfer: `from` must not carry data");
        return Err(InstructionError::InvalidArgument);
    }
    if lamports > from.lamports() {
        msg!(
            "Transfer: insufficient lamports {}, need {}",
            from.lamports(),
            lamports
        );
        return Err(SystemError::ResultWithNegativeLamports.into());
    }
    **from.lamports.borrow_mut() = from.lamports() - lamports;
    **to.lamports.borrow_mut() = to.lamports() + lamports;
    Ok(())
}


pub fn transfer<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, lamports: u64) -> Result<(), InstructionError> {
    let from = next_account_info(accounts_iter)
        .map_err(|_| InstructionError::NotEnoughAccountKeys)?;
    let to = next_account_info(accounts_iter)
        .map_err(|_| InstructionError::NotEnoughAccountKeys)?;
    if !from.is_signer {
        msg!(
            "Transfer: `from` account {} must sign",
            from.key.to_string(),
        );
        return Err(InstructionError::MissingRequiredSignature);
    }
    transfer_verified(from, to, lamports)
}


pub fn transfer_with_seeds<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, lamports: u64, from_seed: String, from_owner: Pubkey) -> Result<(), InstructionError> {
    let from = next_account_info(accounts_iter)
        .map_err(|_| InstructionError::NotEnoughAccountKeys)?;
    let base = next_account_info(accounts_iter)
        .map_err(|_| InstructionError::NotEnoughAccountKeys)?;
    let to = next_account_info(accounts_iter)
        .map_err(|_| InstructionError::NotEnoughAccountKeys)?;
    if !base.is_signer {
        msg!(
            "Transfer: `from` account {} must sign",
            from.key.to_string(),
        );
        return Err(InstructionError::MissingRequiredSignature);
    }
    let address_from_seed = Pubkey::create_with_seed(
        base.key,
        &from_seed,
        &from_owner,
    )?;
    if *from.key != address_from_seed {
        msg!(
            "Transfer: 'from' address {} does not match derived address {}",
            from.key,
            address_from_seed
        );
        return Err(SystemError::AddressWithSeedMismatch.into());
    }
    transfer_verified(from, to, lamports)
}
