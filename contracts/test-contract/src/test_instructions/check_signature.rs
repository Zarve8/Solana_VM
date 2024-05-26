use std::fmt::Display;
use solana_program::account_info::{AccountInfo, next_account_info};
use std::slice::Iter;
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use crate::kernel::{
    utils::assert,
};


pub fn check_signature<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, program_id: &Pubkey) -> ProgramResult {
    let storage_ai = next_account_info(accounts_iter)?;
    assert(storage_ai.is_signer, true)?;
    msg!("Account Signed");
    Ok(())
}