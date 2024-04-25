use borsh::{BorshDeserialize, BorshSerialize};
use crate::prelude::SuperKey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperInnerInstruction {
    pub program: SuperKey,
    pub accounts: Vec<SuperKey>,
    pub data: Vec<u8>,
    pub stack_height: usize
}
