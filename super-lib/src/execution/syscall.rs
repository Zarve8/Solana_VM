use borsh::{BorshDeserialize, BorshSerialize};
use crate::account::key::SuperKey;
use crate::execution::transfer::SuperTransfer;
use crate::transaction::vars::SuperVars;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum SuperSysCall {
    StartProgram { // From Executor to Program
    program_id: SuperKey,
        transfer: SuperTransfer,
        instruction: Vec<u8>,
        vars: SuperVars
    },
    CPI { // From Program to Executor
    program_id: SuperKey,
        transfer: SuperTransfer,
        instruction: Vec<u8>,
        accounts: Vec<SuperKey>,
        signed: Option<SuperKey>
    },
    Log {message: String}, // From Program to Executor
    ProgramFinished { // ProgramCalled -> Executor -> ProgramCaller
        error: Option<u64>,
        transfer: SuperTransfer,
    },
}