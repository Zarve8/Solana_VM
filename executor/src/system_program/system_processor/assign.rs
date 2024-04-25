use std::collections::HashSet;
use solana_program::instruction::InstructionError;
use super_lib::prelude::*;
use crate::system_program::system_processor::allocate::allocate;
use crate::system_program::utils::SYSTEM_PROGRAM;


/*
fn assign(
    account: &mut BorrowedAccount,
    address: &Address,
    owner: &Pubkey,
    signers: &HashSet<Pubkey>,
    invoke_context: &InvokeContext,
) -> Result<(), InstructionError> {
    // no work to do, just return
    if account.get_owner() == owner {
        return Ok(());
    }

    if !address.is_signer(signers) {
        ic_msg!(invoke_context, "Assign: account {:?} must sign", address);
        return Err(InstructionError::MissingRequiredSignature);
    }

    account.set_owner(&owner.to_bytes(), &invoke_context.feature_set)
}
 */

pub fn assign(account_index: usize, account_address: &SuperKey, owner: &SuperKey, signers: &HashSet<SuperKey>, transfer: &mut SuperTransfer, reporter: &mut SuperReporter) -> Result<(), InstructionError> {
    let mut account = transfer.metas.get_mut(account_index).unwrap();
    if account.owner.eq(owner) {
        return Ok(());
    }

    if !signers.contains(account_address) {
        reporter.ic_msg(format!("Assign: account {:?} must sign", account_address.to_string()));
        return Err(InstructionError::MissingRequiredSignature);
    }

    set_account_owner(account, transfer.accounts.get_mut(account_address).unwrap(), owner)
}

/*
pub fn set_owner(
        &mut self,
        pubkey: &[u8],
        feature_set: &FeatureSet,
    ) -> Result<(), InstructionError> {
        // Only the owner can assign a new owner
        if !self.is_owned_by_current_program() {
            return Err(InstructionError::ModifiedProgramId);
        }
        // and only if the account is writable
        if !self.is_writable() {
            return Err(InstructionError::ModifiedProgramId);
        }
        // and only if the account is not executable
        if self.is_executable(feature_set) {
            return Err(InstructionError::ModifiedProgramId);
        }
        // and only if the data is zero-initialized or empty
        if !is_zeroed(self.get_data()) {
            return Err(InstructionError::ModifiedProgramId);
        }
        // don't touch the account if the owner does not change
        if self.get_owner().to_bytes() == pubkey {
            return Ok(());
        }
        self.touch()?;
        self.account.copy_into_owner_from_slice(pubkey);
        Ok(())
    }
 */

pub fn set_account_owner(meta: &mut SuperMeta, account: &SuperAccount, owner: &SuperKey) -> Result<(), InstructionError> {
    // Only the owner can assign a new owner
    if !meta.owner.eq(&SYSTEM_PROGRAM) {
        return Err(InstructionError::ModifiedProgramId);
    }

    // and only if the account is writable
    if !meta.writable {
        return Err(InstructionError::ModifiedProgramId);
    }

    // and only if the account is not executable
    if meta.executable {
        return Err(InstructionError::ModifiedProgramId);
    }

    /* Intercepts with allocate */
    // and only if the data is zero-initialized or empty
    // if account.data.len() != 0 {
    //     println!("Data Length: {}", account.data.len());
    //     return Err(InstructionError::ModifiedProgramId);
    // }

    // don't touch the account if the owner does not change
    if meta.owner.eq(owner) {
        return Ok(());
    }

    meta.owner = owner.clone();
    Ok(())
}


/*
fn allocate_and_assign(
    to: &mut BorrowedAccount,
    to_address: &Address,
    space: u64,
    owner: &Pubkey,
    signers: &HashSet<Pubkey>,
    invoke_context: &InvokeContext,
) -> Result<(), InstructionError> {
    allocate(to, to_address, space, signers, invoke_context)?;
    assign(to, to_address, owner, signers, invoke_context)
}
 */

pub fn allocate_and_assign(to_index: usize, to_address: &SuperKey, space: u64, owner: &SuperKey, signers: &HashSet<SuperKey>, transfer: &mut SuperTransfer, reporter: &mut SuperReporter) -> Result<(), InstructionError> {
    allocate(to_index, to_address, signers, transfer, space, reporter)?;
    assign(to_index, to_address, owner, signers, transfer, reporter)
}