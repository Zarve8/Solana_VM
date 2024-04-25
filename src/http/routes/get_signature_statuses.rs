use serde_json::{json, Value};
use data_manager::prelude::TransactionManager;
use crate::global::data_manager::GLOBAL_DATA;
use crate::http::interfaces::get_signature_statuses::GetSignatureStatusesRequest;
use crate::http::interfaces::params::optional::OptionalParams;


pub async fn get_signature_statuses(request: &GetSignatureStatusesRequest, _params: OptionalParams) -> Value {
    let manager = GLOBAL_DATA.read().expect("Failed to read Global Data");
    let mut result: Vec<Value> = Vec::with_capacity(request.0.len());
    for id in request.0.iter() {
        let data = manager.get_transaction(&id[0]);
        result.push(match data {
            None => {
                Value::Null
            },
            Some(data) => {
                json!({
                    "slot": data.slot,
                    "confirmations": null,
                    "err": null,
                    "status": {
                        "Ok": null
                    },
                    "confirmationStatus": "finalized"
                })
            }
        });
    }
    json!(result)
}