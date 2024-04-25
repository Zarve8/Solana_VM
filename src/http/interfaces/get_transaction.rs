use serde::{Deserialize, Serialize};
use crate::http::interfaces::rpc_request::RpcRequest;



#[derive(Deserialize, Serialize, Debug)]
pub struct GetTransactionRequest(pub (String, String));


impl RpcRequest for GetTransactionRequest {
    fn length() -> usize {
        2
    }

    fn validate(&self) {
        //TODO
    }
}