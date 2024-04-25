use std::collections::HashMap;
use solana_program::instruction::InstructionError;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction::SystemError;
use super_lib::prelude::*;


pub fn transfer_tx(from_account_index: usize, to_account_index: usize, transfer: &mut SuperTransfer, vars: &SuperVars, lamports: u64, reporter: &mut SuperReporter) -> Result<(), InstructionError>{
    let mut from = &transfer.metas[from_account_index];
    let mut to = &transfer.metas[to_account_index];
    if !from.is_signer {
        reporter.ic_msg(format!("Transfer: `from` account {} must sign", from.address.to_string()));
        return Err(InstructionError::MissingRequiredSignature);
    }
    transfer_verified(
        from,
        to,
        &mut transfer.accounts,
        lamports,
        reporter)
}

fn transfer_verified(from_meta: &SuperMeta, to_meta: &SuperMeta, accounts: &mut HashMap<SuperKey, SuperAccount>, lamports: u64, reporter: &mut SuperReporter) -> Result<(), InstructionError> {
    {
        let mut from_account = accounts.get_mut(&from_meta.address).unwrap();
        if from_account.data.len() > 0 {
            reporter.ic_msg(format!("Transfer: `from` must not carry data"));
            return Err(InstructionError::InvalidArgument);
        }
        if from_account.lamports < lamports {
            reporter.ic_msg(format!(
            "Transfer: insufficient lamports {}, need {}",
            from_account.lamports,
            lamports
            ));
            return Err(SystemError::ResultWithNegativeLamports.into());
        }
        from_account.lamports -= lamports;
    }
    {
        let mut to_account = accounts.get_mut(&to_meta.address).unwrap();
        to_account.lamports += lamports;
    }
    Ok(())
}

pub fn transfer_with_seed_tx(transfer: &mut SuperTransfer, vars: &SuperVars, lamports: u64, seed: &str, from_owner: &Pubkey, reporter: &mut SuperReporter) -> Result<(), InstructionError>{
    let mut from_account = &transfer.metas[0];
    let mut from_base_account = &transfer.metas[1];
    let to_account = &transfer.metas[2];
    if !from_base_account.is_signer {
        reporter.ic_msg(format!("Transfer: `from` account {} must sign", from_base_account.address.to_string()));
        return Err(InstructionError::MissingRequiredSignature);
    }
    let address_from_seed = Pubkey::create_with_seed(
        &Pubkey::from(from_base_account.address.0),
        seed,
        &from_owner,
    )?;

    let from_key = Pubkey::from(from_account.address.0);
    if from_key != address_from_seed {
        reporter.ic_msg(format!(
            "Transfer: 'from' address {} does not match derived address {}",
            from_key,
            address_from_seed
        ));
        return Err(SystemError::AddressWithSeedMismatch.into());
    }
    transfer_verified(
        from_account,
        to_account,
        &mut transfer.accounts,
        lamports,
        reporter
    )
}

/*
fn transfer_with_seed(
    from_account_index: IndexOfAccount,
    from_base_account_index: IndexOfAccount,
    from_seed: &str,
    from_owner: &Pubkey,
    to_account_index: IndexOfAccount,
    lamports: u64,
    invoke_context: &InvokeContext,
    transaction_context: &TransactionContext,
    instruction_context: &InstructionContext,
) -> Result<(), InstructionError> {
    if !instruction_context.is_instruction_account_signer(from_base_account_index)? {
        ic_msg!(
            invoke_context,
            "Transfer: 'from' account {:?} must sign",
            transaction_context.get_key_of_account_at_index(
                instruction_context
                    .get_index_of_instruction_account_in_transaction(from_base_account_index)?,
            )?,
        );
        return Err(InstructionError::MissingRequiredSignature);
    }
    let address_from_seed = Pubkey::create_with_seed(
        transaction_context.get_key_of_account_at_index(
            instruction_context
                .get_index_of_instruction_account_in_transaction(from_base_account_index)?,
        )?,
        from_seed,
        from_owner,
    )?;

    let from_key = transaction_context.get_key_of_account_at_index(
        instruction_context.get_index_of_instruction_account_in_transaction(from_account_index)?,
    )?;
    if *from_key != address_from_seed {
        ic_msg!(
            invoke_context,
            "Transfer: 'from' address {} does not match derived address {}",
            from_key,
            address_from_seed
        );
        return Err(SystemError::AddressWithSeedMismatch.into());
    }

    transfer_verified(
        from_account_index,
        to_account_index,
        lamports,
        invoke_context,
        transaction_context,
        instruction_context,
    )
}
 */
/*
fn transfer_verified(
    from_account_index: IndexOfAccount,
    to_account_index: IndexOfAccount,
    lamports: u64,
    invoke_context: &InvokeContext,
    transaction_context: &TransactionContext,
    instruction_context: &InstructionContext,
) -> Result<(), InstructionError> {
    let mut from = instruction_context
        .try_borrow_instruction_account(transaction_context, from_account_index)?;
    if !from.get_data().is_empty() {
        ic_msg!(invoke_context, "Transfer: `from` must not carry data");
        return Err(InstructionError::InvalidArgument);
    }
    if lamports > from.get_lamports() {
        ic_msg!(
            invoke_context,
            "Transfer: insufficient lamports {}, need {}",
            from.get_lamports(),
            lamports
        );
        return Err(SystemError::ResultWithNegativeLamports.into());
    }

    from.checked_sub_lamports(lamports, &invoke_context.feature_set)?;
    drop(from);
    let mut to = instruction_context
        .try_borrow_instruction_account(transaction_context, to_account_index)?;
    to.checked_add_lamports(lamports, &invoke_context.feature_set)?;
    Ok(())
}
 */
//
// fn transfer(
//     from_account_index: IndexOfAccount,
//     to_account_index: IndexOfAccount,
//     lamports: u64,
//     invoke_context: &InvokeContext,
//     transaction_context: &TransactionContext,
//     instruction_context: &InstructionContext,
// ) -> Result<(), InstructionError> {
//     if !instruction_context.is_instruction_account_signer(from_account_index)? {
//         ic_msg!(
//             invoke_context,
//             "Transfer: `from` account {} must sign",
//             transaction_context.get_key_of_account_at_index(
//                 instruction_context
//                     .get_index_of_instruction_account_in_transaction(from_account_index)?,
//             )?,
//         );
//         return Err(InstructionError::MissingRequiredSignature);
//     }
//
//     transfer_verified(
//         from_account_index,
//         to_account_index,
//         lamports,
//         invoke_context,
//         transaction_context,
//         instruction_context,
//     )
// }