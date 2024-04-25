use std::collections::HashMap;
use super_lib::prelude::{SuperKey, SuperReporter, SuperTransaction};
use borsh::{BorshDeserialize, BorshSerialize};
use base64::{engine::general_purpose, Engine as _};


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TransactionData {
    pub block_time: u64,
    pub compute_units_consumed: u64,
    pub fee: u64,
    pub log_messages: Vec<String>,
    pub post_balances: Vec<u64>,
    pub post_token_balances: Vec<u64>,
    pub pre_balances: Vec<u64>,
    pub pre_token_balances: Vec<u64>,
    pub inner_instruction: Vec<InstructionData>,
    pub slot: u64,
    pub recent_blockhash: String,
    pub signatures: Vec<String>,
    pub accounts: Vec<AccountMetaData>,
    pub instructions: Vec<InstructionData>
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct InstructionData {
    pub index: u8,
    pub program: usize,
    pub accounts: Vec<usize>,
    pub data: Vec<u8>,
    pub stack_height: u8,
}

impl InstructionData {
    pub fn encode(&self) -> String {
        general_purpose::STANDARD.encode(&self.data)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct AccountMetaData {
    pub pubkey: SuperKey,
    pub signer: bool,
    pub writable: bool
}

impl TransactionData {
    pub fn from_reporter(transaction: &SuperTransaction,
                         reporter: SuperReporter,
                         signatures: Vec<String>,
                         block_time: u64,
                         recent_blockhash: String) -> Self {
        let mut accounts = Vec::with_capacity(transaction.state.accounts.len());
        let mut instructions = Vec::with_capacity(transaction.instructions.len());
        let mut inner_instructions = Vec::with_capacity(reporter.inner_instructions.len());
        let mut balance_map = HashMap::new();
        let mut account_map: HashMap<SuperKey, usize> = HashMap::new();
        for (meta, acc) in reporter.changed_accounts.iter() {
            balance_map.insert(meta.address, acc.lamports);
        }
        for i in 0..transaction.state.accounts.len() {
            let (meta, acc) = &transaction.state.accounts[i];
            accounts.push(AccountMetaData{
                pubkey: meta.address.clone(),
                writable: meta.writable,
                signer: meta.is_signer
            });
            account_map.insert(meta.address.clone(), i);
        }
        for i in 0..transaction.instructions.len() {
            let instr = &transaction.instructions[i];
            instructions.push(InstructionData {
                index: 0,
                program: instr.program.clone(),
                data: instr.data.clone(),
                accounts: instr.accounts.clone(),
                stack_height: 1,
            });
        }
        for i in 0..reporter.inner_instructions.len() {
            let instr = &reporter.inner_instructions[i];
            inner_instructions.push(InstructionData {
                index: i as u8,
                program: account_map.get(&instr.program).unwrap().clone(),
                accounts: instr.accounts.iter()
                    .map(|key| account_map.get(key).unwrap().clone())
                    .collect(),
                data: instr.data.clone(),
                stack_height: instr.stack_height as u8,
            })
        }
        Self {
            log_messages: reporter.log_messages,
            pre_balances: transaction.state.accounts.iter()
                .map(|(meta, acc)| acc.lamports)
                .collect(),
            post_token_balances: vec![],
            post_balances: transaction.state.accounts.iter()
                .map(|(meta, acc)| {
                    match balance_map.get(&meta.address) {
                        None => acc.lamports,
                        Some(lamports) => *lamports
                    }
                })
                .collect(),
            pre_token_balances: vec![],
            instructions,
            accounts,
            signatures,
            block_time,
            compute_units_consumed: 0,
            slot: transaction.state.vars.slot,
            fee: 0, //TODO take_fees
            inner_instruction: inner_instructions,
            recent_blockhash,
        }
    }
}