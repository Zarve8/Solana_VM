use serde::{Deserialize, Serialize};
use crate::http::interfaces::rpc_request::RpcRequest;


#[derive(Deserialize, Serialize, Debug)]
pub struct GetMinimumBalanceForRentExemptionRequest(pub [u64; 1]);


impl RpcRequest for GetMinimumBalanceForRentExemptionRequest {
    fn length() -> usize {
        1
    }

    fn validate(&self) {
        //TODO
    }
}