mod allocate;
mod assign;
mod create_account;
mod transfer;


use solana_program::system_instruction::SystemInstruction;
use solana_program::instruction::InstructionError;
use super_lib::prelude::*;
use crate::system_program::limited_deserialize::limited_deserialize;
use crate::system_program::system_processor::create_account::create_account_tx;
use crate::system_program::system_processor::transfer::{transfer_tx, transfer_with_seed_tx};
use crate::system_program::utils::{check_number_of_instruction_accounts, copy_transfer};


pub fn system_processor(transfer: SuperTransfer, instruction: &Vec<u8>, vars: &SuperVars, reporter: &mut SuperReporter) -> Result<SuperTransfer, InstructionError> {
    let instruction = limited_deserialize(instruction.as_slice())?;
    let mut result_transfer = copy_transfer(&transfer);
    let signers = transfer.list_signers();
    match instruction {
        SystemInstruction::Transfer {lamports} => {
            check_number_of_instruction_accounts(&transfer, 2)?;
            transfer_tx(0, 1, &mut result_transfer, &vars, lamports, reporter)?;
        }
        SystemInstruction::TransferWithSeed {lamports, from_seed, from_owner } => {
            check_number_of_instruction_accounts(&transfer, 3)?;
            transfer_with_seed_tx(&mut result_transfer, &vars, lamports, &from_seed, &from_owner, reporter)?;
        }
        SystemInstruction::CreateAccount {space, lamports, owner} => {
            check_number_of_instruction_accounts(&transfer, 2)?;
            create_account_tx(
                0,
                1,
                &transfer.metas[1].address.clone(),
                lamports,
                space,
                &SuperKey(owner.to_bytes()),
                &signers,
                &mut result_transfer,
                &vars,
                reporter
            )?;
        },
        SystemInstruction::CreateAccountWithSeed {lamports, space, owner, base, seed} => {
            check_number_of_instruction_accounts(&transfer, 2)?;
            create_account_tx(
                0,
                1,
                &transfer.metas[1].address.clone(),
                lamports,
                space,
                &SuperKey(owner.to_bytes()),
                &signers,
                &mut result_transfer,
                &vars,
                reporter
            )?;
        }
        _ => {/*TODO*/}
    }
    Ok(result_transfer)
}
/*
SystemInstruction::CreateAccount {
            lamports,
            space,
            owner,
        } => {
            instruction_context.check_number_of_instruction_accounts(2)?;
            let to_address = Address::create(
                transaction_context.get_key_of_account_at_index(
                    instruction_context.get_index_of_instruction_account_in_transaction(1)?,
                )?,
                None,
                invoke_context,
            )?;
            create_account(
                0,
                1,
                &to_address,
                lamports,
                space,
                &owner,
                &signers,
                invoke_context,
                transaction_context,
                instruction_context,
            )
        }
        SystemInstruction::CreateAccountWithSeed {
            base,
            seed,
            lamports,
            space,
            owner,
        } => {
            instruction_context.check_number_of_instruction_accounts(2)?;
            let to_address = Address::create(
                transaction_context.get_key_of_account_at_index(
                    instruction_context.get_index_of_instruction_account_in_transaction(1)?,
                )?,
                Some((&base, &seed, &owner)),
                invoke_context,
            )?;
            create_account(
                0,
                1,
                &to_address,
                lamports,
                space,
                &owner,
                &signers,
                invoke_context,
                transaction_context,
                instruction_context,
            )
        }
 */


/*
SystemInstruction::Transfer { lamports } => {
            instruction_context.check_number_of_instruction_accounts(2)?;
            transfer(
                0,
                1,
                lamports,
                invoke_context,
                transaction_context,
                instruction_context,
            )
        }
        SystemInstruction::TransferWithSeed {
            lamports,
            from_seed,
            from_owner,
        } => {
            instruction_context.check_number_of_instruction_accounts(3)?;
            transfer_with_seed(
                0,
                1,
                &from_seed,
                &from_owner,
                2,
                lamports,
                invoke_context,
                transaction_context,
                instruction_context,
            )
        }
 */