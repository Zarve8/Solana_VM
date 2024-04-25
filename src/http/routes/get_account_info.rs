use serde_json::{json, Value};
use crate::global::data_manager::GLOBAL_DATA;
use crate::http::interfaces::get_account_info::GetAccountInfoRequest;
use crate::http::interfaces::params::optional::OptionalParams;
use data_manager::prelude::AccountManager;


pub async fn get_account_info(request: &GetAccountInfoRequest, params: OptionalParams) -> Value {
    let manager = GLOBAL_DATA.read().expect("Failed to read Global Data");
    let meta = manager.get_account_meta(&request.0[0]);
    let mut space: usize = 0;
    match meta {
        None => {
            Value::Null
        }
        Some(meta) => {
            let data = manager.get_account_data(&request.0[0]);
            let bytes = match data {
                None => {
                    json!([
                        "",
                        "base64"
                    ])
                }
                Some(data) => {
                    space = data.bytes.len();
                    json!([
                        data.to_base64(),
                        "base64"
                    ])
                }
            };
            json!({
                "data": bytes,
                "executable": meta.executable,
                "lamports": meta.lamports,
                "owner": meta.owner.to_string(),
                "rentEpoch": 0,
                "space": space
            })
        }
    }
}