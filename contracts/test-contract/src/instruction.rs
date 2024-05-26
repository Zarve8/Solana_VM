use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, PartialEq, Clone)]
pub enum Instruction {
    Message, // 0
    WriteAccount, // 1
    CreateAccount {bytes: u32}, // 2
    ReallocAccount {bytes: u32}, // 3
    TransferSol {amount: u32}, // 4
    CallProgram, // 5
    CallProgramSigned, // 6
    CheckSignature, // 7
    ReadSysvars, // 8
    ThrowError, // 9
}