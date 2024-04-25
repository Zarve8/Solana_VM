use borsh::{BorshDeserialize, BorshSerialize};
use std::io::{self, Read, Write};
use crate::messenger::utils;
use crate::messenger::utils::MESSAGE_CODE;


pub struct Messenger {}

impl Messenger {
    pub fn send<T: BorshSerialize>(data: T) {
        let mut bytes: Vec<u8> = Vec::new();
        data.serialize(&mut bytes).expect("Failed to serialize message to send");
        io::stdout().write_all(&MESSAGE_CODE).expect("Failed to send message to stdout");
        io::stdout().write_all(&utils::u32_to_bytes(bytes.len())).expect("Failed to send message to stdout");
        io::stdout().write_all(bytes.as_slice()).expect("Failed to send message to stdout");
        io::stdout().flush().expect("Failed to flush stdout");
    }

    pub fn receive<T: BorshDeserialize>() -> T {
        let mut head = [0, 0, 0, 0];
        io::stdin().read_exact(&mut head).expect("Failed to receive message from stdin");
        let length = utils::bytes_to_u32(head);
        let mut bytes: Vec<u8> = vec![0; length];
        io::stdin().read_exact(bytes.as_mut_slice()).expect("Failed to receive message body from stdin");
        io::stdout().flush().expect("Failed to flush stdin");
        T::deserialize(&mut bytes.as_slice()).expect("Failed to deserialize message")
    }
}