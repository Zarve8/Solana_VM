use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct BaseHttpRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Option<Vec<serde_json::value::Value>>
}