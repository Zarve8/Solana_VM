use borsh::{BorshDeserialize, BorshSerialize};
use bs58;


#[derive(
BorshDeserialize,
BorshSerialize,
Clone,
Copy,
Default,
Eq,
Hash,
Ord,
PartialEq,
PartialOrd,
Debug
)]
pub struct SuperKey(pub [u8; 32]);

impl ToString for SuperKey {
    fn to_string(&self) -> String {
        bs58::encode(self.0).into_string()
    }
}

impl SuperKey {
    pub fn from_string(s: &str) -> Self {
        if s.len() > 48 {
            panic!("Invalid Length for a Pubkey: {}", s);
        }
        let pubkey_vec = bs58::decode(s)
            .into_vec()
            .expect("Failed to parse base58 string");
        SuperKey(<[u8; 32]>::try_from(pubkey_vec.as_slice()).unwrap())
    }

    pub fn as_any<T>(&self) -> &T {
        let ptr: *const u8 = self.0.as_ptr();
        let target_ptr = ptr as *mut T;
        unsafe {
            target_ptr.as_ref().unwrap()
        }
    }

    pub fn system_program() -> Self {
        SuperKey([0; 32])
    }

    pub fn faucet() -> Self {
        SuperKey([0; 32])
    }
}
