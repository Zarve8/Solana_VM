use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use actix_web::{HttpResponse};
use actix_web::{post, web};
use serde_json::{json, Value};
use super_lib::prelude::{SuperKey, SuperReporter};
use crate::global::transaction::GLOBAL_EXECUTOR;
use crate::transaction::TransactionOutput;
use crate::transaction::types::compiled_instruction::CompiledInstruction;
use crate::transaction::types::hash::Hash;
use crate::transaction::types::message::Message;
use crate::transaction::types::message_header::MessageHeader;


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
            .body(result)
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


/*
{
    "jsonrpc": "2.0",
    "error": {
        "code": -32002,
        "message": "Transaction simulation failed: Error processing Instruction 1: invalid instruction data",
        "data": {
            "accounts": null,
            "err": {
                "InstructionError": [
                    1,
                    "InvalidInstructionData"
                ]
            },
            "innerInstructions": null,
            "logs": [
                "Program CscAmJEyqL2M3whNUkbjDTcyUUdp3EuvJv1GXDyPuQEA invoke [1]",
                "Program log: Earned 9000 of Oil",
                "Program log: Gathered Region: 1059398392, 1231174896",
                "Program CscAmJEyqL2M3whNUkbjDTcyUUdp3EuvJv1GXDyPuQEA consumed 29683 of 400000 compute units",
                "Program CscAmJEyqL2M3whNUkbjDTcyUUdp3EuvJv1GXDyPuQEA success",
                "Program CscAmJEyqL2M3whNUkbjDTcyUUdp3EuvJv1GXDyPuQEA invoke [1]",
                "Program log: Account for Region: 1059398392, 1231174896 not provided",
                "Program CscAmJEyqL2M3whNUkbjDTcyUUdp3EuvJv1GXDyPuQEA consumed 9561 of 370317 compute units",
                "Program CscAmJEyqL2M3whNUkbjDTcyUUdp3EuvJv1GXDyPuQEA failed: invalid instruction data"
            ],
            "returnData": null,
            "unitsConsumed": 39244
        }
    },
    "id": "473895b4-02f0-4ec4-a015-84c19af23eaa"
 */


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
