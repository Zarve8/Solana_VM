use std::fmt::Display;
use solana_program::account_info::{AccountInfo};
use std::slice::Iter;
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;


pub fn throw_error<'a>(_accounts_iter: &mut Iter<'a, AccountInfo<'a>>, _program_id: &Pubkey) -> ProgramResult {
    msg!("Trowing Error: InvalidSeeds");
    Err(ProgramError::InvalidSeeds)
}