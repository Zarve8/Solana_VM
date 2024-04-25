use std::fmt::Display;
use solana_program::account_info::{AccountInfo};
use std::slice::Iter;
use solana_program::clock::Clock;
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::kernel::utils::assert;


pub fn read_sysvars<'a>(_accounts_iter: &mut Iter<'a, AccountInfo<'a>>, _program_id: &Pubkey) -> ProgramResult {
    assert(Rent::get().unwrap().lamports_per_byte_year, 1000)?;
    assert(Rent::get().unwrap().exemption_threshold, 2.0)?;
    assert(Rent::get().unwrap().minimum_balance(1 ), 258000)?;
    assert(Clock::get().unwrap().slot, 99)?;
    assert(Clock::get().unwrap().unix_timestamp, 169999999)?;
    msg!("Sysvars Matched");
    Ok(())
}