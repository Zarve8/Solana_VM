use serde_json::{json, Value};
use crate::global::state::GLOBAL_STATE;
use crate::http::interfaces::get_minimum_balance_for_rent_exempt_request::GetMinimumBalanceForRentExemptionRequest;


pub async fn get_minimum_balance_for_rent_exemption(request: GetMinimumBalanceForRentExemptionRequest) -> Value {
    let state = GLOBAL_STATE.lock().expect("Failed to read Global State");
    json!(state.vars.lamports_per_byte_year*2*request.0[0]*8)
}


//getMinimumBalanceForRentExemption
/*
{
  jsonrpc: '2.0',
  result: 2039280,
  id: 'b7ad0a29-949b-45b3-8d61-999c50073fee'
}
 */