use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperVars {
    pub slot: u64,
    pub timestamp: u64,
    pub lamports_per_byte_year: u64
}

impl SuperVars {
    pub fn default() -> Self {
        SuperVars {
            slot: 0,
            timestamp: 0,
            lamports_per_byte_year: 3480
        }
    }
}