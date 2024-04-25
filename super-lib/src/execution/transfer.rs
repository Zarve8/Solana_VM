use std::collections::{HashMap, HashSet};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::account::account::SuperAccount;
use crate::account::key::SuperKey;
use crate::account::meta::SuperMeta;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperTransfer {
    pub metas: Vec<SuperMeta>,
    pub accounts: HashMap<SuperKey, SuperAccount>
}

impl SuperTransfer {
    pub fn filter(&mut self, program_id: &SuperKey) {
        self.metas.retain(|meta| meta.writable && meta.owner.eq(program_id));
        let addresses: HashSet<SuperKey> =   HashSet::from_iter(self.metas
            .iter()
            .map(|meta| &meta.address)
            .cloned());
        self.accounts.retain(|key, _account| addresses.contains(key) );
    }

    pub fn empty() -> Self {
        SuperTransfer {
            metas: Vec::new(),
            accounts: HashMap::new()
        }
    }

    pub fn list_signers(&self) -> HashSet<SuperKey> {
        self.metas.iter()
            .filter(|meta| meta.is_signer)
            .map(|meta| meta.address.to_owned())
            .collect()
    }
}