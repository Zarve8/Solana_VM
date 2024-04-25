use std::fmt::Display;
use solana_program::account_info::{AccountInfo, next_account_info};
use std::slice::Iter;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::{msg, system_instruction};
use solana_program::program::{invoke, invoke_signed};
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::kernel::{
    utils::assert,
};



#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MyStorage {
    pub key: Pubkey,
    pub data: Vec<u32>
}


pub fn create_account<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, program_id: &Pubkey, bytes: u32) -> ProgramResult {
    let payer_ai = next_account_info(accounts_iter)?;
    let account1_ai = next_account_info(accounts_iter)?;
    let account2_ai = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    assert(payer_ai.is_signer, true)?;
    assert(account1_ai.is_signer, true)?;
    assert(account2_ai.is_signer, false)?;
    assert(account1_ai.data_len(), 0)?;
    assert(account2_ai.key.to_string(), String::from("72SoBX41P6JbUS47asuvYaRrbofX4gEtM2sA8D1x8hVU"))?;
    assert(account2_ai.data_len(), 0)?;
    let idx = system_instruction::transfer(
        payer_ai.key,
        account1_ai.key,
        Rent::get()?.minimum_balance(bytes as usize),
    );
    invoke(
        &idx,
        &[payer_ai.clone(), account1_ai.clone(), system_program.clone()])?;
    msg!("Account1 funded");
    let idx = system_instruction::create_account(
        payer_ai.key,
        account1_ai.key,
        Rent::get()?.minimum_balance(bytes as usize ),
        bytes as u64,
        &program_id
    );
    invoke(
        &idx,
        &[payer_ai.clone(), account1_ai.clone(), system_program.clone()])?;
    assert(account1_ai.data_len(), bytes as usize)?;
    msg!("Account Created");

    let idx = system_instruction::transfer(
        payer_ai.key,
        account2_ai.key,
        Rent::get()?.minimum_balance(bytes as usize),
    );
    invoke(
        &idx,
        &[payer_ai.clone(), account2_ai.clone(), system_program.clone()])?;
    msg!("Account2 funded");
    let (_key, bump) = Pubkey::find_program_address(&[
        b"my account",
        program_id.as_ref()
    ], program_id);
    let idx = system_instruction::create_account(
        payer_ai.key,
        account2_ai.key,
        Rent::get()?.minimum_balance(bytes as usize ),
        bytes as u64,
        &program_id
    );
    invoke_signed(
        &idx,
        &[payer_ai.clone(), account2_ai.clone(), system_program.clone()],
        &[&[
            b"my account",
            program_id.as_ref(),
            &[bump],
        ]])?;
    assert(account2_ai.data_len(), bytes as usize)?;
    msg!("PDA Account Created");
    Ok(())
}