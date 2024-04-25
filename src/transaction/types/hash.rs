use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use std::str::FromStr;
use std::mem;
use crate::transaction::types::constants::{HASH_BYTES, MAX_BASE58_LEN};

#[derive(
Serialize,
Deserialize,
BorshSerialize,
BorshDeserialize,
BorshSchema,
Clone,
Copy,
Default,
Eq,
PartialEq,
Ord,
PartialOrd,
Hash,
)]
pub struct Hash(pub(crate) [u8; HASH_BYTES]);

impl FromStr for Hash {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() > MAX_BASE58_LEN {
            return Err("Wrong Hash Size");
        }
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|_| "Invalid Hash")?;
        if bytes.len() != mem::size_of::<Hash>() {
            Err("Wrong Hash Size")
        } else {
            Ok(Hash::new(&bytes))
        }
    }
}

impl Hash {
    pub fn new(hash_slice: &[u8]) -> Self {
        Hash(<[u8; HASH_BYTES]>::try_from(hash_slice).unwrap())
    }

    pub const fn new_from_array(hash_array: [u8; HASH_BYTES]) -> Self {
        Self(hash_array)
    }

    pub fn to_bytes(self) -> [u8; HASH_BYTES] {
        self.0
    }
}
