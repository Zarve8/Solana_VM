use serde::{Deserialize, Serialize};
use crate::http::interfaces::rpc_request::RpcRequest;



#[derive(Deserialize, Serialize, Debug)]
pub struct GetSignatureStatusesRequest(pub Vec<Vec<String>>);


impl RpcRequest for GetSignatureStatusesRequest{
    fn length() -> usize {
        1
    }

    fn validate(&self) {
        //TODO
    }
}