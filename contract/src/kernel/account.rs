use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::borsh0_9::try_from_slice_unchecked;
use solana_program::system_instruction;
use solana_program::rent::Rent;
use solana_program::program::{invoke, invoke_signed};
use solana_program::sysvar::Sysvar;
use crate::kernel::programs::SELF_PROGRAM_ID;


pub fn load_account<T: BorshDeserialize + BorshSerialize + Clone>(ai: &AccountInfo) -> Result<T, ProgramError> {
    let state = try_from_slice_unchecked::<T>(&ai.data.borrow())
        .map_err(|_| ProgramError::BorshIoError(String::from("Cannot load Account")))?;
    Ok(state)
}

pub fn load_owned_account<T: BorshDeserialize + BorshSerialize + Clone>(ai: &AccountInfo) -> Result<T, ProgramError> {
    if !SELF_PROGRAM_ID.eq(ai.owner) {
        return Err(ProgramError::IllegalOwner);
    }
    let state = try_from_slice_unchecked::<T>(&ai.data.borrow())
        .map_err(|_| ProgramError::BorshIoError(String::from("Cannot load Account")))?;
    Ok(state)
}

pub fn save_account<T: BorshDeserialize + BorshSerialize + Clone>(data: &T, ai: &AccountInfo) -> Result<(), ProgramError> {
    data.serialize(&mut *ai.data.borrow_mut())
        .map_err(|_| ProgramError::BorshIoError(String::from("Cannot save Account")))?;
    Ok(())
}

pub fn alloc_account<'a>(size: usize, ai: &'a AccountInfo<'a>, payer: &'a AccountInfo<'a>, system_program: &'a AccountInfo<'a>, seeds: &[&[&[u8]]]) -> Result<(), ProgramError>{
    let idx = system_instruction::create_account(
        payer.key,
        ai.key,
        Rent::get()?.minimum_balance(size ),
        size as u64,
        &SELF_PROGRAM_ID
    );
    invoke_signed(
        &idx,
        &[payer.clone(), ai.clone(), system_program.clone()],
        seeds)
}


pub fn realloc_account<'a>(size: usize, ai: &'a AccountInfo<'a>, payer: &'a AccountInfo<'a>, system_program: &'a AccountInfo<'a>) -> Result<(), ProgramError>{
    let idx = system_instruction::transfer(
        payer.key,
        ai.key,
        Rent::get()?.minimum_balance(size),
    );
    invoke(
        &idx,
        &[payer.clone(), ai.clone(), system_program.clone()])?;
    ai.realloc(size, false)
}
