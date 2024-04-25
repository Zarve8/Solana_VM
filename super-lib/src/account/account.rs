use crate::account::data::SuperData;
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperAccount {
    pub lamports: u64,
    pub data: SuperData,
}
