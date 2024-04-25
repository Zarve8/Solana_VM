use solana_program::account_info::{AccountInfo, next_account_info};
use std::slice::Iter;
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::log::{
    sol_log_compute_units,
    sol_log_64,
    sol_log_slice,
    sol_log_params,
    sol_log
};


pub fn message<'a>(accounts_iter: &mut Iter<'a, AccountInfo<'a>>, program_id: &Pubkey) -> ProgramResult {
    sol_log("test");
    sol_log_64(1, 2, 3, 4, 5);
    sol_log_slice([8].as_slice());
    //sol_log_params() //TODO
    sol_log_compute_units();
    Ok(())
}