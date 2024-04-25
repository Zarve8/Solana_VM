use std::fmt::Debug;
use actix_web::{HttpResponse};
use actix_web::{post, web};
use serde_json::{json, Value};
use crate::global::state::GLOBAL_STATE;
use crate::http::interfaces:: {
    base_request::BaseHttpRequest,
    get_account_info::GetAccountInfoRequest,
    get_balance::GetBalanceRequest,
    params::{
        optional::OptionalParams
    }
};
use crate::http::interfaces::get_signature_statuses::GetSignatureStatusesRequest;
use crate::http::interfaces::get_transaction::GetTransactionRequest;
use crate::http::parse_params::parse_params;
use crate::http::routes::{
    get_account_info::get_account_info,
    get_balance::get_balance,
    get_block_height::get_block_height
};
use crate::http::routes::get_signature_statuses::get_signature_statuses;
use crate::http::routes::get_transaction::get_transaction;


fn pack_context(value: Value) -> Value{
    let state = GLOBAL_STATE.lock().expect("Failed access GLOBAL STATE");
    let slot = state.vars.slot;
    json!({
        "context": {
            "slot": slot
        },
        "value": value
    })
}

#[post("/rpc")]
pub async fn rpc(body: web::Json<BaseHttpRequest>) -> HttpResponse {
    let result = std::panic::catch_unwind(|| async {
        println!("=========={body:?}=========");
        let method: &str = &body.method;
        let value = match method {
            "getAccountInfo" => {
                let (request, optional) = parse_params::<GetAccountInfoRequest>(body.params.clone().unwrap());
                pack_context(get_account_info(&request, OptionalParams::from_option(optional)).await)
            },
            "getBalance" => {
                let (request, optional) = parse_params::<GetBalanceRequest>(body.params.clone().unwrap());
                pack_context(get_balance(&request, OptionalParams::from_option(optional)).await)
            },
            "getBlockHeight" => {
                get_block_height().await
            },
            "getTransaction" => {
                let (request, optional) = parse_params::<GetTransactionRequest>(body.params.clone().unwrap());
                get_transaction(&request, OptionalParams::from_option(optional)).await
            },
            "getSignatureStatuses" => {
                let (request, optional) = parse_params::<GetSignatureStatusesRequest>(body.params.clone().unwrap());
                pack_context(get_signature_statuses(&request, OptionalParams::from_option(optional)).await)
            },
            _ => {
                json!("Non implemented method")
            }
        };
        HttpResponse::Ok().body(json!({
        "jsonrpc": "2.0",
        "result": value,
        "id": body.id
        }).to_string())
    });
    if result.is_ok() {
        result.unwrap().await
    }
    else {
        //println!("RPC Error: {:?}", result);
        HttpResponse::ExpectationFailed().body(json!({
            "jsonrpc": "2.0",
            "error": {
                "code": null,
                "message": "Internal Error"
            },
            "id": body.id
        }).to_string())
    }
}



