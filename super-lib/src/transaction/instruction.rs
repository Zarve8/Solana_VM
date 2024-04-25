use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperInstruction {
    pub program: usize,
    pub accounts: Vec<usize>,
    pub data: Vec<u8>
}
