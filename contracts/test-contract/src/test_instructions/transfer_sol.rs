use std::fmt::Display;
use solana_program::account_info::{AccountInfo, next_account_info};
use std::slice::Iter;
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::{msg, system_instruction};
use solana_program::program::invoke;
use crate::kernel::utils::assert;


pub fn transfer_sol<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, _program_id: &Pubkey, amount: u64) -> ProgramResult {
    let to_ai = next_account_info(accounts_iter)?;
    let from_ai = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    assert(from_ai.try_lamports().unwrap(), 2000001)?;
    assert(to_ai.try_lamports().unwrap(), 101)?;
    let idx = system_instruction::transfer(
        from_ai.key,
        to_ai.key,
        amount,
    );
    let v: Vec<u8> = idx.data.clone();
    msg!("IDX: {:?}", idx);
    // msg!("{:?}", idx.data);
    invoke(
        &idx,
        &[from_ai.clone(), to_ai.clone(), system_program.clone()])?;
    assert(from_ai.try_lamports().unwrap(), 2000001 - amount)?;
    assert(to_ai.try_lamports().unwrap(), 101 + amount)?;
    msg!("Direct Transfer Succeed");
    Ok(())
}