use serde::{Deserialize, Serialize};
use crate::http::interfaces::rpc_request::RpcRequest;


#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountInfoRequest(pub [String; 1]);


impl RpcRequest for GetAccountInfoRequest {
    fn length() -> usize {
        1
    }

    fn validate(&self) {
        //TODO
    }
}