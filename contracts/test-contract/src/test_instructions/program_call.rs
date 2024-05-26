use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use std::slice::Iter;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::msg;
use solana_program::program::invoke;
use crate::kernel::account::load_account;
use crate::kernel::utils::assert;
use crate::test_instructions::account_write::MyStorage;


pub fn program_call<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, program_id: &Pubkey) -> ProgramResult {
    let program_ai = next_account_info(accounts_iter)?;
    let account_ai = next_account_info(accounts_iter)?;
    let idx = Instruction::new_with_bytes(
        program_ai.key.clone(),
        &[1],
        vec![AccountMeta::new(*account_ai.key, account_ai.is_signer)]
    );
    invoke(&idx, &[account_ai.clone(), program_ai.clone()])?;
    msg!("Program Invoked");
    let mut storage: MyStorage = load_account(account_ai)?;
    assert(storage.key.to_string(), program_id.to_string())?;
    assert(storage.data.len(), 1)?;
    assert(storage.data[0], 99)?;
    msg!("Id stored into account: {}", program_id.to_string());
    Ok(())
}
