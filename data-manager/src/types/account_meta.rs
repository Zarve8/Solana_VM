use super_lib::prelude::SuperKey;
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct AccountMeta {
    pub address: SuperKey,
    pub owner: SuperKey,
    pub executable: bool,
    pub lamports: u64
}
