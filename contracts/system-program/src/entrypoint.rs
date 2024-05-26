use std::collections::HashMap;
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    msg
};
use solana_program::program_error::ProgramError;
use crate::global::ForceAccountUpdate;
use crate::processor::Processor;
use crate::processor::set_account_owner::SYSTEM_PROGRAM;

// #[cfg(not(feature = "no-entrypoint"))]
// entrypoint!(process_instruction);


#[no_mangle]
pub unsafe extern "C" fn entrypoint<'g>(
    program_id: &'g Pubkey,
    accounts: &'g [AccountInfo<'g>],
    instruction_data: &[u8],
) -> Result<(HashMap<Pubkey, Vec<u8>>, HashMap<Pubkey, Pubkey>), u64>{
    match Processor::process_instruction(program_id, accounts, instruction_data) {
        Ok(_) => {
            msg!("End");
            let mut update = ForceAccountUpdate::pack();
            msg!("Update Out: {:?}", update);
            if update.0.len() == 0 {
                update.0.insert(SYSTEM_PROGRAM.clone(), Vec::new());
            }
            if update.1.len() == 0 {
                update.1.insert(SYSTEM_PROGRAM.clone(), SYSTEM_PROGRAM.clone());
            }
            Ok(update)
        },
        Err(err) => { //TODO
            // let code: u64 = err.into();
            Err(ProgramError::Custom(0).into())
        }
    }
}


/*
#[macro_export]
macro_rules! system_entrypoint {
    ($process_instruction:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint<'a>(program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>], instruction_data: &'a [u8],
            vars: &'a $crate::custom::global_storage::GlobalVars) -> Result<(HashMap<Pubkey, Vec<u8>>, HashMap<Pubkey, Pubkey>), u64> {
            {
                let mut storage = $crate::custom::global_storage::GLOBAL_VARS.lock().unwrap();
                storage.consume_vars(&vars);
            }
            match $process_instruction(&program_id, &accounts, &instruction_data) {
                Ok(()) => $crate::entrypoint::SUCCESS,
                Err(error) => error.into(),
            }
        }
        $crate::custom_heap_default!();
        $crate::custom_panic_default!();
    };
}
 */
