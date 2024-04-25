use crate::account::key::SuperKey;
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperMeta {
    pub address: SuperKey,
    pub owner: SuperKey,
    pub executable: bool,
    pub writable: bool,
    pub is_signer: bool
}