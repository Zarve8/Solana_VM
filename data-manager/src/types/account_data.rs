use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct AccountData {
    pub bytes: Vec<u8>
}


impl AccountData {
    pub fn to_base64(&self) -> String {
        base64::encode(&self.bytes)
    }
}