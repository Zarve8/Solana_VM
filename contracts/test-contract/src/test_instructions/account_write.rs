use std::fmt::Display;
use solana_program::account_info::{AccountInfo, next_account_info};
use std::slice::Iter;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use crate::kernel::{
    account::load_account,
    utils::assert,
};
use crate::kernel::account::save_account;


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MyStorage {
    pub key: Pubkey,
    pub data: Vec<u32>
}


pub fn write_account<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, program_id: &Pubkey) -> ProgramResult {
    let storage_ai = next_account_info(accounts_iter)?;
    assert(storage_ai.key.to_string(), String::from("6yk2s4VtX1i7xN7R15WHGtU4AoMtStSBY98H87LtirW"))?;
    assert(storage_ai.owner.to_string(), String::from("FaGHKV74yrwsAgbp9SxadKhBbfAQteNbJEetsEWUppCa"))?;
    assert(storage_ai.is_signer, true)?;
    assert(storage_ai.is_writable, true)?;
    assert(storage_ai.executable, false)?;
    assert(storage_ai.lamports(), 1000000000)?;
    assert(storage_ai.data_len(), 40)?;
    let mut storage: MyStorage = load_account(storage_ai)?;
    assert(storage.key.to_string(), String::from("11111111111111111111111111111111"))?;
    assert(storage.data.len(), 0)?;
    storage.key = program_id.clone();
    storage.data.push(99);
    save_account(&mut storage, storage_ai)?;
    msg!("Account Written and Saved");
    Ok(())
}