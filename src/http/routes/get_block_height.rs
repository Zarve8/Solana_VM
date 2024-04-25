use serde_json::{json, Value};
use crate::global::state::GLOBAL_STATE;


pub async fn get_block_height() -> Value {
    let state = GLOBAL_STATE.lock().expect("Failed to read Global State");
    json!(state.vars.slot)
}