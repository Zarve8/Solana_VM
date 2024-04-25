use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::{Pubkey};
use std::slice::Iter;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::msg;
use solana_program::program::{invoke_signed};
use crate::kernel::utils::assert;


pub fn program_signed_call<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, program_id: &Pubkey) -> ProgramResult {
    let program_ai = next_account_info(accounts_iter)?;
    let account_ai = next_account_info(accounts_iter)?;
    assert(account_ai.is_signer, false);
    let idx = Instruction::new_with_bytes(
        program_ai.key.clone(),
        &[7],
        vec![AccountMeta::new(*account_ai.key, account_ai.is_signer)]
    );
    let (key, bump) = Pubkey::find_program_address(&[
        b"my account",
        program_id.as_ref()
        ], program_id);
    assert(account_ai.key.to_string(), String::from("72SoBX41P6JbUS47asuvYaRrbofX4gEtM2sA8D1x8hVU"));
    assert(key.to_string(), String::from("72SoBX41P6JbUS47asuvYaRrbofX4gEtM2sA8D1x8hVU"));
    invoke_signed(
        &idx,
        &[account_ai.clone(), program_ai.clone()],
        &[
            &[
                b"my account",
                program_id.as_ref(),
                &[bump],
            ]
        ]
    )?;
    msg!("Program Invoked Signed");
    assert(account_ai.is_signer, false);
    Ok(())
}