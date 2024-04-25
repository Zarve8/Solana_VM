use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};
use crate::processor::Processor;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);


fn process_instruction<'g>(
    program_id: &'g Pubkey,
    accounts: &'g [AccountInfo<'g>],
    instruction_data: &[u8],
) -> ProgramResult {
    match {
        msg!("==== Start Program ====");
        Processor::process_instruction(program_id, accounts, instruction_data)?;
        msg!("==== End Program ====");
        Ok(())
    } {
        Ok(_) => Ok(()),
        Err(E) => {
            msg!("Error: {}", E);
            Err(E)
        }
    }
}

