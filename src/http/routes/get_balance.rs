use serde_json::{json, Value};
use crate::global::data_manager::GLOBAL_DATA;
use crate::http::interfaces::get_balance::GetBalanceRequest;
use crate::http::interfaces::params::optional::OptionalParams;
use data_manager::prelude::AccountManager;


pub async fn get_balance(request: &GetBalanceRequest, params: OptionalParams) -> Value {
    let manager = GLOBAL_DATA.read().expect("Failed to read Global Data");
    let meta = manager.get_account_meta(&request.0[0]);
    match meta {
        None => {
            json!(0)
        }
        Some(meta) => {
            json!(meta.lamports)
        }
    }
}