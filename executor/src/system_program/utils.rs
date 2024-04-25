use solana_program::instruction::InstructionError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use super_lib::prelude::*;


pub fn check_number_of_instruction_accounts(transfer: &SuperTransfer, expected_at_least: usize) -> Result<(), InstructionError> {
    if transfer.metas.len() < expected_at_least {
        Err(InstructionError::NotEnoughAccountKeys)
    } else {
        Ok(())
    }
}


pub fn copy_transfer(transfer: &SuperTransfer) -> SuperTransfer {
    SuperTransfer {
        metas: transfer.metas.clone(),
        accounts: transfer.accounts.clone(),
    }
}


pub const SYSTEM_PROGRAM: SuperKey = SuperKey([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);


pub fn format_program_error(err_data: u64) -> String {
    let error = ProgramError::from(err_data);
    error.to_msg()
}


pub fn convert_system_error(error: InstructionError) -> u64 {
    //TODO
    0
}
