use std::collections::{HashSet};
use solana_program::instruction::InstructionError;
use super_lib::prelude::*;
use crate::system_program::system_processor::assign::allocate_and_assign;
use crate::system_program::system_processor::transfer::transfer_tx;


/*
fn create_account(
    from_account_index: IndexOfAccount,
    to_account_index: IndexOfAccount,
    to_address: &Address,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
    signers: &HashSet<Pubkey>,
    invoke_context: &InvokeContext,
    transaction_context: &TransactionContext,
    instruction_context: &InstructionContext,
) -> Result<(), InstructionError> {
    // if it looks like the `to` account is already in use, bail
    {
        let mut to = instruction_context
            .try_borrow_instruction_account(transaction_context, to_account_index)?;
        if to.get_lamports() > 0 {
            ic_msg!(
                invoke_context,
                "Create Account: account {:?} already in use",
                to_address
            );
            return Err(SystemError::AccountAlreadyInUse.into());
        }

        allocate_and_assign(&mut to, to_address, space, owner, signers, invoke_context)?;
    }
    transfer(
        from_account_index,
        to_account_index,
        lamports,
        invoke_context,
        transaction_context,
        instruction_context,
    )
}
 */


pub fn create_account_tx(
    from_account_index: usize,
    to_account_index: usize,
    to_address: &SuperKey,
    lamports: u64,
    space: u64,
    owner: &SuperKey,
    signers: &HashSet<SuperKey>,
    transfer: &mut SuperTransfer, vars: &SuperVars, reporter: &mut SuperReporter
    ) -> Result<(), InstructionError> {

    {
        let mut to = transfer.accounts.get(to_address).unwrap();
        /* Fails when new account funded */
        // if to.lamports > 0 {
        //     ic_msg!(
        //         "Create Account: account {:?} already in use",
        //         to_address.to_string()
        //     );
        //     return Err(SystemError::AccountAlreadyInUse.into());
        // }

        allocate_and_assign(to_account_index, to_address, space, owner, signers, transfer, reporter)?
    }

    transfer_tx(
        from_account_index,
        to_account_index,
        transfer,
        vars,
        lamports,
        reporter
    )

}
