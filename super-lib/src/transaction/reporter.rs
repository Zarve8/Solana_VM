use borsh::{BorshDeserialize, BorshSerialize};
use crate::prelude::{SuperAccount, SuperMeta};
use crate::transaction::inner_instruction::SuperInnerInstruction;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SuperReporter {
    pub error: Option<u64>,
    pub log_messages: Vec<String>,
    pub inner_instructions: Vec<SuperInnerInstruction>,
    pub changed_accounts: Vec<(SuperMeta, SuperAccount)>
}


#[cfg(feature = "executor")]
impl SuperReporter {
    pub fn new() -> Self {
        SuperReporter {
            error: None,
            log_messages: Vec::new(),
            inner_instructions: Vec::new(),
            changed_accounts: Vec::new()
        }
    }

    pub fn log(&mut self, msg: String) {
/*        println!("{}", msg);*/
        self.log_messages.push(msg);
    }

    pub fn ic_msg(&mut self, msg: String) {
        let s = format!("Program logged: {}", msg);
        // println!("{}", s);
        self.log_messages.push(s);
    }
}

#[cfg(feature = "super")]
impl SuperReporter {
    pub fn from_internal_failure() -> Self {
        SuperReporter {
            error: Some(0),
            log_messages: vec![String::from("Execution failed with internal critical error")],
            inner_instructions: Vec::new(),
            changed_accounts: Vec::new()
        }
    }
}