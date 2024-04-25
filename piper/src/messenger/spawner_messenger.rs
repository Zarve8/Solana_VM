use borsh::{BorshDeserialize, BorshSerialize};
use std::io::{Read, Write};
use std::process::{Child};
use crate::messenger::utils;
use crate::messenger::utils::MESSAGE_CODE;


pub struct Messenger<T> {
    child: Child,
    pub data: T
}


impl<D> Messenger<D> {
    pub fn new(child: Child, data: D) -> Self {
        Self { child, data }
    }

    pub fn send<T: BorshSerialize>(&mut self, data: T) {
        let mut bytes: Vec<u8> = Vec::new();
        data.serialize(&mut bytes).expect("Failed to serialize message to send");
        let stdin =self.child.stdin.as_mut().unwrap();
        stdin.write_all(&utils::u32_to_bytes(bytes.len())).expect("Failed to send message to pipein");
        stdin.write_all(bytes.as_slice()).expect("Failed to send message to pipein");
    }

    pub fn receive<T: BorshDeserialize>(&mut self) -> T {
        let code = self.read_code();
        let stdout = self.child.stdout.as_mut().unwrap();
        match code {
            Some(code) => {
                println!(" {}", code);
                let mut data = vec![0; 256];
                let length = stdout.read(data.as_mut_slice()).expect("Failed to read whole string from pipe");
                data.drain(length..);
                match String::from_utf8(data.clone()) {
                    Ok(s) => {
                        println!("Got Unexpected Message: <<{}{}>>", code, s);
                    }
                    Err(_) => {
                        println!("Got Unexpected Unparsed Message: <<{}{:?}>>", code, data);
                    }
                };
                panic!("Unexpected STDOUT")
            },
            None => {}
        }
        let mut head = [0, 0, 0, 0];
        stdout.read_exact(&mut head).expect("Failed to receive message from pipe");
        let length = utils::bytes_to_u32(head);
        let mut bytes: Vec<u8> = vec![0; length];
        stdout.read_exact(bytes.as_mut_slice()).expect("Failed to receive message body from stdin");
        T::deserialize(&mut bytes.as_slice()).expect("Failed to deserialize message")
    }

    pub fn wait(&mut self) {
        self.child.wait().expect("Child process failed");
    }

    pub fn kill(&mut self) {
        self.child.kill().expect("Child process failed");
    }

    pub fn read_str(&mut self) -> String {
        let stdout = self.child.stdout.as_mut().unwrap();
        let mut s = String::new();
        stdout.read_to_string(&mut s).expect("Failed to read string from pipe");
        s
    }

    fn read_code(&mut self) -> Option<String> {
        let stdout = self.child.stdout.as_mut().unwrap();
        let mut code = String::new();
        for b in MESSAGE_CODE.iter() {
            let mut c = [0];
            stdout.read_exact(&mut c).expect("Failed to read code char from pipe");
            if c[0] != *b {
                code.push_str( & match String::from_utf8(Vec::from(c)) {
                    Ok(s) => s,
                    Err(_) => String::from("_")
                });
                return Some(code);
            }
        }
        None
    }
}