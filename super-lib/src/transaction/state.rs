use std::collections::HashSet;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::prelude::{SuperAccount, SuperKey, SuperMeta, SuperVars};
use crate::vm_id::VMID;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperState {
    pub id: VMID,
    pub vars: SuperVars,
    pub accounts: Vec<(SuperMeta, SuperAccount)>,
}


#[cfg(feature = "executor")]
impl SuperState {
    pub fn list_signers(&self) -> HashSet<SuperKey> {
        let mut set = HashSet::with_capacity(self.accounts.len());
        for (meta, account) in self.accounts.iter() {
            if meta.is_signer {
                set.insert(meta.address.clone());
            }
        }
        set
    }
}