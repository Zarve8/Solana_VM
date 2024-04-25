use borsh::{BorshDeserialize, BorshSerialize};
use crate::transaction::instruction::SuperInstruction;
use crate::transaction::state::SuperState;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperTransaction {
    pub instructions: Vec<SuperInstruction>,
    pub state: SuperState,
    pub payer: usize
}