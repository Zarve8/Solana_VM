use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use actix_web::{HttpResponse};
use actix_web::{post, web};
use serde_json::{json, Value};
use serde_json::Value::Null;
use data_manager::prelude::VMState;
use super_lib::prelude::{SuperKey, SuperReporter, VMID};
use crate::global::transaction::GLOBAL_EXECUTOR;
use crate::transaction::TransactionOutput;
use crate::transaction::types::compiled_instruction::CompiledInstruction;
use crate::transaction::types::hash::Hash;
use crate::transaction::types::message::Message;
use crate::transaction::types::message_header::MessageHeader;
use crate::global::data_manager::GLOBAL_DATA;
use data_manager::prelude::StateManager;


#[post("/super/send_transaction")]
pub async fn send_transaction(body: web::Json<SSendTransactionRequest>) -> HttpResponse {
    println!("=========={body:?}=========");
    let msg = Message {
        header: MessageHeader {
            num_required_signatures: body.signers.len() as u8,
            num_readonly_unsigned_accounts: (body.accounts.len() - body.signers.len()) as u8,
            num_readonly_signed_accounts: 0, //TODO
        },
        account_keys: body.accounts.iter()
            .map(|s| SuperKey::from_string(s))
            .collect(),
        recent_blockhash: Hash::new(&[0; 32]),
        signers: body.signers.iter()
            .map(|index| SuperKey::from_string(&body.accounts[*index]))
            .collect(),
        writables: body.writable.iter()
            .map(|index| SuperKey::from_string(&body.accounts[*index]))
            .collect(),
        instructions: body.instructions.iter()
            .map(|instr| CompiledInstruction {
                program_id_index: instr.program_id,
                accounts: instr.accounts.clone(),
                data: instr.data.clone(),
            })
            .collect(),
        payer: body.payer
    };
    println!("New Message Constructed");
    let async_result = {
        let mut stack = GLOBAL_EXECUTOR.lock().expect("Failed to access stack");
        stack.push(msg)
    };
    let result = async_result.await.expect("Failed to receive message from oneshot channel");
    match result {
        Ok(result) => {
            HttpResponse::Ok()
            .body(json!({
                "result": result
            }).to_string())
        }
        Err(reporter) => {
            HttpResponse::ExpectationFailed()
                .body(construct_error_report(reporter).to_string())
        }
    }
}

pub fn construct_error_report(reporter: SuperReporter) -> Value {
    json!({
        "error": {
            "code": -32002,
            "message": "Transaction simulation failed", // TODO add description
            "data": {
                "accounts": null,
                "err": null,
                "innerInstructions": null,
                "logs": reporter.log_messages,
                "returnData": Value::Null,
                "unitsConsumed": 0
            }
        }
    })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SSendTransactionRequest {
    accounts: Vec<String>,
    instructions: Vec<SInstruction>,
    signers: Vec<usize>,
    writable: Vec<usize>,
    payer: usize
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SInstruction{
    program_id: u8,
    accounts: Vec<u8>,
    data: Vec<u8>
}



#[post("/super/airdrop")]
pub async fn airdrop(body: web::Json<SAirdropRequest>) -> HttpResponse {
    println!("=========={body:?}=========");
    let msg = Message {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_unsigned_accounts: 0,
            num_readonly_signed_accounts: 0,
        },
        account_keys: vec![SuperKey::system_program(), SuperKey::from_string(&body.to)],
        recent_blockhash: Hash::new(&[0; 32]),
        signers: HashSet::from([SuperKey::system_program()]),
        writables: HashSet::from([SuperKey::system_program(), SuperKey::from_string(&body.to)]),
        instructions: vec![CompiledInstruction {
            program_id_index: 0,
            accounts: vec![1, 0, 0],
            data: vec![4, 99, 0, 0, 0],
        }],
        payer: 0
    };
    println!("New Message Constructed");
    let async_result = {
        let mut stack = GLOBAL_EXECUTOR.lock().expect("Failed to access stack");
        stack.push(msg)
    };
    let result = async_result.await.expect("Failed to receive message from oneshot channel");
    match result {
        Ok(result) => {
            HttpResponse::Ok()
                .body(result)
        }
        Err(reporter) => {
            HttpResponse::ExpectationFailed()
                .body(construct_error_report(reporter).to_string())
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SAirdropRequest{
    to: String
}



#[post("/new")]
pub async fn new_vm(body: web::Json<SNewVMRequest>) -> HttpResponse {
    println!("=========={body:?}=========");
    let state = VMState::new_with_id(body.id);
    {
        let data_manager = GLOBAL_DATA.read().expect("Failed to access Global Data on load");
        state.initialize(&data_manager);
        data_manager.set_state(&state);
    }
    HttpResponse::Ok().body("Ok")
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SNewVMRequest{
    id: VMID
}