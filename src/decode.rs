
mod Transaction {
    use {
        serde::Serialize,
        generic_array::{typenum::U64, GenericArray},
        solana_program::{
            pubkey::Pubkey,
            hash::Hash
        },
        borsh::{BorshDeserialize, BorshSerialize}
    };

    #[repr(transparent)]
    #[derive(BorshDeserialize, Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Signature([u8; 32]); //GenericArray<u8, U64>


    #[derive(BorshDeserialize, Default, Debug, PartialEq, Eq, Clone, Copy)]
    pub struct MessageHeader {
        pub num_required_signatures: u8,

        pub num_readonly_signed_accounts: u8,

        pub num_readonly_unsigned_accounts: u8,
    }


    #[derive(BorshDeserialize, Debug, PartialEq, Eq, Clone)]
    pub struct CompiledInstruction {
        pub program_id_index: u8,

        // #[serde(with = "short_vec")]
        pub accounts: Vec<u8>,

        // #[serde(with = "short_vec")]
        pub data: Vec<u8>,
    }


    #[derive(BorshDeserialize, Default, Debug, PartialEq, Eq, Clone)]
    pub struct LegacyMessage {
        pub header: MessageHeader,

        // #[serde(with = "short_vec")]
        pub account_keys: Vec<Pubkey>,

        pub recent_blockhash: Hash,

        // #[serde(with = "short_vec")]
        pub instructions: Vec<CompiledInstruction>,
    }


    #[derive(BorshDeserialize, Default, Debug, PartialEq, Eq, Clone)]
    pub struct MessageAddressTableLookup {
        pub account_key: Pubkey,

        // #[serde(with = "short_vec")]
        pub writable_indexes: Vec<u8>,
        // #[serde(with = "short_vec")]
        pub readonly_indexes: Vec<u8>,
    }


    #[derive(BorshDeserialize, Default, Debug, PartialEq, Eq, Clone)]
    pub struct MessageV0 {
        pub header: MessageHeader,

        // #[serde(with = "short_vec")]
        pub account_keys: Vec<Pubkey>,

        pub recent_blockhash: Hash,

        // #[serde(with = "short_vec")]
        pub instructions: Vec<CompiledInstruction>,

        // #[serde(with = "short_vec")]
        pub address_table_lookups: Vec<MessageAddressTableLookup>,
    }


    #[derive(BorshDeserialize, Debug, PartialEq, Eq, Clone)]
    pub enum VersionedMessage {
        Legacy(LegacyMessage),
        V0(MessageV0),
    }

    #[derive(BorshDeserialize, Debug, PartialEq, Eq, Clone)]
    pub struct VersionedTransaction {
        // #[serde(with = "short_vec")]
        pub signatures: Vec<Signature>,
        pub message: VersionedMessage,
    }
}


#[cfg(test)]
mod test_tx_decode {
    // use solana_sdk::{
    //     transaction::VersionedTransaction
    // };
    // use serde::Deserialize;

    use borsh::{BorshDeserialize, BorshSerialize};
    use crate::decode::Transaction::VersionedTransaction;

    use base64::{engine::general_purpose, Engine as _};




    // // constrain output types to have the `Deserialize` trait
    // fn deserialize<'a, T>(data: &'a [u8]) -> T where T: Deserialize<'a> {
    //     let msg = str::from_utf8(data).unwrap();
    //     serde_json::from_str::<T>(msg).unwrap()
    // }

    #[test]
    pub fn decode() {
        let packed_tx = "AWjChNn3jSSGEODcVYyeRWLl48tnMREXezqqt8xG/t42VJEn2yl9glZRCeeyZXuxNGUpT5VEt52akJcD750xfw8BAAIGUPeCMtJJOu/nsQNtcegTMeY2IpwDDR7Nl1Euw/Pwb6UPEs68+QkFIMuOirFnQ6PtYlxz3s3wnKKiQoX9q1gbZrunyqFkw3dlKk4LtxTHLwpIvdau06mK0Iqkrjzr0N9z5M5zqiDUoTxK1C8qdT/V5IKzZXzDd+W0oJLp5xdcsCwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALBnXJKqc3ySlNRoJcxadZeLtRHbImwrf8h6noO9eny31cPJPshuvQE5BYSQPSTG+8NzbOo835tfQhtdvcetMyoBBQUABAIDAQ0DAQAAAEiwe1L4FW7F";
        let mut packed_buf = general_purpose::STANDARD.decode(packed_tx).expect("Failed to parse tx from base64 string");
        println!("Tx Buf: {:?}", packed_buf);
        let tx = VersionedTransaction::deserialize(&mut packed_buf.as_slice()).expect("Failed to deserialize versioned transaction");
        // serde
        // let tx = VersionedTransaction::deserialize(packed_buf.as_slice()).expect("Failed to deserialize versioned transaction");
    }
}