use std::collections::HashSet;
use solana_program::instruction::InstructionError;
use solana_program::system_instruction::SystemError;
use super_lib::prelude::*;
use crate::system_program::utils::SYSTEM_PROGRAM;


/*
fn allocate(
    account: &mut BorrowedAccount,
    address: &Address,
    space: u64,
    signers: &HashSet<Pubkey>,
    invoke_context: &InvokeContext,
) -> Result<(), InstructionError> {
    if !address.is_signer(signers) {
        ic_msg!(
            invoke_context,
            "Allocate: 'to' account {:?} must sign",
            address
        );
        return Err(InstructionError::MissingRequiredSignature);
    }

    // if it looks like the `to` account is already in use, bail
    //   (note that the id check is also enforced by message_processor)
    if !account.get_data().is_empty() || !system_program::check_id(account.get_owner()) {
        ic_msg!(
            invoke_context,
            "Allocate: account {:?} already in use",
            address
        );
        return Err(SystemError::AccountAlreadyInUse.into());
    }

    if space > MAX_PERMITTED_DATA_LENGTH {
        ic_msg!(
            invoke_context,
            "Allocate: requested {}, max allowed {}",
            space,
            MAX_PERMITTED_DATA_LENGTH
        );
        return Err(SystemError::InvalidAccountDataLength.into());
    }

    account.set_data_length(space as usize, &invoke_context.feature_set)?;

    Ok(())
}
*/

pub const MAX_PERMITTED_DATA_LENGTH: u64 = 10 * 1024 * 1024;

/// Maximum permitted size of new allocations per transaction, in bytes.
///
/// The value was chosen such that at least one max sized account could be created,
/// plus some additional resize allocations.
pub const MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION: i64 =
    MAX_PERMITTED_DATA_LENGTH as i64 * 2;


pub fn allocate(account_index: usize, account_address: &SuperKey, signers: &HashSet<SuperKey>, transfer: &mut SuperTransfer, space: u64, reporter: &mut SuperReporter) -> Result<(), InstructionError> {
    {
        let mut account = transfer.metas.get_mut(account_index).unwrap();
        if !signers.contains(account_address) {
            reporter.ic_msg(format!("Allocate: 'to' account {:?} must sign", account_address.to_string()));
            return Err(InstructionError::MissingRequiredSignature);
        }

        // if it looks like the `to` account is already in use, bail
        //   (note that the id check is also enforced by message_processor)
        if transfer.accounts.get(account_address).unwrap().data.len() != 0 || !account.owner.eq(&SYSTEM_PROGRAM) {
            reporter.ic_msg(format!(
                "Allocate: account {:?} already in use",
                account_address.to_string()
            ));
            return Err(SystemError::AccountAlreadyInUse.into());
        }

        if space > MAX_PERMITTED_DATA_LENGTH {
            reporter.ic_msg(format!(
            "Allocate: requested {}, max allowed {}",
            space,
            MAX_PERMITTED_DATA_LENGTH
        ));
            return Err(SystemError::InvalidAccountDataLength.into());
        }
    }

    set_account_data_length(&transfer.metas[account_index],
                            transfer.accounts.get_mut(account_address).unwrap(),
                            space as usize)?;
    Ok(())
}

/*
    pub fn set_data_length(
        &mut self,
        new_length: usize,
        feature_set: &FeatureSet,
    ) -> Result<(), InstructionError> {
        self.can_data_be_resized(new_length)?;
        self.can_data_be_changed(feature_set)?;
        // don't touch the account if the length does not change
        if self.get_data().len() == new_length {
            return Ok(());
        }
        self.touch()?;
        self.update_accounts_resize_delta(new_length)?;
        self.account.resize(new_length, 0);
        Ok(())
    }
 */

pub fn set_account_data_length(meta: &SuperMeta, account: &mut SuperAccount, new_length: usize) -> Result<(), InstructionError> {
    can_account_data_be_resized(meta, account, new_length)?;
    can_account_data_be_changed(meta)?;
    // don't touch the account if the length does not change
    if account.data.len() == new_length {
        return Ok(());
    }

    if(account.data.len() < new_length) {
        account.data.append(&mut vec![0u8; new_length - account.data.len()])
    }
    else {
        account.data.drain(new_length..);
    }
    Ok(())
}

/*
pub fn can_data_be_changed(&self, feature_set: &FeatureSet) -> Result<(), InstructionError> {
        // Only non-executable accounts data can be changed
        if self.is_executable(feature_set) {
            return Err(InstructionError::ExecutableDataModified);
        }
        // and only if the account is writable
        if !self.is_writable() {
            return Err(InstructionError::ReadonlyDataModified);
        }
        // and only if we are the owner
        if !self.is_owned_by_current_program() {
            return Err(InstructionError::ExternalAccountDataModified);
        }
        Ok(())
    }
 */

pub fn can_account_data_be_changed(meta: &SuperMeta) -> Result<(), InstructionError> {
    // Only non-executable accounts data can be changed
    if meta.executable {
        return Err(InstructionError::ExecutableDataModified);
    }
    // and only if the account is writable
    if !meta.writable {
        return Err(InstructionError::ReadonlyDataModified);
    }
    // and only if we are the owner
    if !meta.owner.eq(&SYSTEM_PROGRAM) {
        return Err(InstructionError::ExternalAccountDataModified);
    }
    Ok(())
}

/*
pub fn can_data_be_resized(&self, new_length: usize) -> Result<(), InstructionError> {
        let old_length = self.get_data().len();
        // Only the owner can change the length of the data
        if new_length != old_length && !self.is_owned_by_current_program() {
            return Err(InstructionError::AccountDataSizeChanged);
        }
        // The new length can not exceed the maximum permitted length
        if new_length > MAX_PERMITTED_DATA_LENGTH as usize {
            return Err(InstructionError::InvalidRealloc);
        }
        // The resize can not exceed the per-transaction maximum
        let length_delta = (new_length as i64).saturating_sub(old_length as i64);
        if self
            .transaction_context
            .accounts_resize_delta()?
            .saturating_add(length_delta)
            > MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION
        {
            return Err(InstructionError::MaxAccountsDataAllocationsExceeded);
        }
        Ok(())
    }
 */

pub fn can_account_data_be_resized(meta: &SuperMeta, account:&mut SuperAccount, new_length: usize) -> Result<(), InstructionError> {
    let old_length = account.data.len();
    if new_length != old_length && !meta.owner.eq(&SYSTEM_PROGRAM) {
        return Err(InstructionError::AccountDataSizeChanged);
    }
    // The new length can not exceed the maximum permitted length
    if new_length > MAX_PERMITTED_DATA_LENGTH as usize {
        return Err(InstructionError::InvalidRealloc);
    }
    // The resize can not exceed the per-transaction maximum
    let length_delta = (new_length as i64).saturating_sub(old_length as i64);
    if length_delta > MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION
    {
        return Err(InstructionError::MaxAccountsDataAllocationsExceeded);
    }
    Ok(())
}